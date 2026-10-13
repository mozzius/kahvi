"""One ticket through the pipeline: worker → gates → blind verifier → merge,
and the bookkeeping that decides what happens to the ticket next.

Interruptions (usage limits, overload, logged out) are not the ticket's
fault: the attempt doesn't count, partial work is kept for the next attempt,
and a finished attempt that only lacks its verification resumes there.
"""

import os
import shutil
import threading
import time
from pathlib import Path

import gates
import llm
import merge
import verifier
import worker
from common import ROOT, WORK, log, read_json, run, write_json

MERGE_LOCK = threading.Lock()
MAX_ERRORS = 3


def next_tier(cfg, tier):
    tiers = cfg.escalation.get("tiers", ["haiku", "sonnet", "opus"])
    i = tiers.index(tier) if tier in tiers else len(tiers) - 1
    return tiers[i + 1] if i + 1 < len(tiers) else None


def strike(cfg, db, ticket, aid, reason, weight=1):
    """Records a failed attempt; enough strikes move the ticket up a tier."""
    row = db.ticket(ticket.id)
    db.note(ticket.id, aid, reason)
    strikes = row["strikes"] + weight
    if row["attempts"] >= cfg.escalation.get("max_attempts", 6):
        db.set_ticket(ticket.id, status="needs-architect", strikes=strikes, salvage_attempt=None,
                      reason=f"{row['attempts']} attempts without finishing; last: {reason[:300]}")
        return
    if strikes >= cfg.escalation.get("strikes_per_tier", 2):
        up = next_tier(cfg, row["tier"])
        if up is None:
            db.set_ticket(ticket.id, status="needs-architect", strikes=strikes, salvage_attempt=None,
                          reason=f"failed at the top tier; last: {reason[:300]}")
            return
        log(f"{ticket.id}: escalating {row['tier']} -> {up}")
        db.note(ticket.id, aid, f"escalated from {row['tier']} to {up}")
        db.set_ticket(ticket.id, status="open", tier=up, strikes=0, salvage_attempt=None)
        return
    db.set_ticket(ticket.id, status="open", strikes=strikes, salvage_attempt=None)


def infra_error(db, ticket, aid, what):
    """A run that failed for reasons other than limits (crash, bad config).
    A few in a row means something needs a human."""
    row = db.ticket(ticket.id)
    errors = row["errors"] + 1
    db.note(ticket.id, aid, f"{what[:400]}")
    if errors >= MAX_ERRORS:
        db.set_ticket(ticket.id, status="needs-architect", errors=errors, reason=f"{errors} errors in a row: {what[:300]}")
    else:
        db.set_ticket(ticket.id, status="open", errors=errors)


def process(cfg, db, ticket, slot):
    row = db.ticket(ticket.id)
    if row["resume_attempt"]:
        aid = row["resume_attempt"]
        a = db.one("SELECT * FROM attempts WHERE id = ?", aid)
        adir = worker.ATTEMPTS / str(aid)
        rep = read_json(adir / "gate.json")
        if a and a["head"] and rep and rep.get("decision") == "pass":
            log(f"{ticket.id}: resuming verification of attempt {aid}")
            db.set_ticket(ticket.id, resume_attempt=None)
            att = worker.Attempt(aid, "", a["base"], a["head"], 0.0, 0, None, None, str(adir))
            return judge(cfg, db, ticket, att, rep)
        db.set_ticket(ticket.id, resume_attempt=None)

    tier = row["tier"]
    log(f"{ticket.id}: attempt on slot {slot} with {tier}")
    att = worker.attempt(cfg, db, ticket, slot, tier, salvage_attempt=row["salvage_attempt"])
    log(f"{ticket.id}: attempt {att.id} finished (${att.cost:.3f}, {att.turns} turns{', ' + att.kind if att.kind else ''})")

    if att.kind in llm.INTERRUPTED:
        db.set_attempt(att.id, outcome="interrupted", finished=time.time())
        db.uncount_attempt(ticket.id)
        if att.head:
            db.note(ticket.id, att.id, f"interrupted ({att.kind}); the next attempt continues from its partial work")
            db.set_ticket(ticket.id, status="open", salvage_attempt=att.id)
        else:
            db.set_ticket(ticket.id, status="open")
        return
    if att.error and not att.head:
        db.set_attempt(att.id, outcome="error", finished=time.time())
        infra_error(db, ticket, att.id, f"worker run failed: {att.error}")
        return
    if att.blocked and not att.head:
        db.set_attempt(att.id, outcome="blocked", finished=time.time())
        db.note(ticket.id, att.id, f"BLOCKED: {att.blocked}")
        db.set_ticket(ticket.id, status="blocked", reason=att.blocked, errors=0)
        log(f"{ticket.id}: blocked: {att.blocked}")
        return
    if att.blocked:
        # A diff still gets judged on its merits; the reason is kept for context.
        db.note(ticket.id, att.id, f"worker reported BLOCKED but changed files: {att.blocked}")
    if not att.head:
        db.set_attempt(att.id, outcome="no-diff", finished=time.time())
        strike(cfg, db, ticket, att.id, "produced no changes")
        return
    db.set_ticket(ticket.id, errors=0)

    rep = gates.check_attempt(cfg, ticket, slot, att.slot_dir, att.base, att.head, att.dir)
    if rep["decision"] == "reject":
        db.set_attempt(att.id, outcome="gate-reject", finished=time.time())
        log(f"{ticket.id}: gate rejected attempt {att.id}: {'; '.join(rep['reject'])[:200]}")
        strike(cfg, db, ticket, att.id, "rejected by gates: " + "; ".join(rep["reject"])[:600])
        return
    judge(cfg, db, ticket, att, rep)


def judge(cfg, db, ticket, att, rep):
    diff = Path(att.dir, "diff.patch").read_text()
    v, vcost = verifier.verify_pipeline(cfg, ticket, worker.spec_text(ticket.spec), gates.render(rep), diff,
                                        record_dir=Path(att.dir))
    write_json(Path(att.dir) / "verdict.json", v)
    decision = v["decision"]
    prev = db.one("SELECT verifier_cost FROM attempts WHERE id = ?", att.id)
    db.set_attempt(att.id, outcome=decision, verifier_cost=(prev["verifier_cost"] if prev else 0) + vcost)
    log(f"{ticket.id}: verifier: {decision}{' (fakery)' if v.get('fakery') else ''} {v.get('summary', '')[:150]}")
    notes = v.get("notes_for_next_attempt", "")

    if decision == "interrupted":
        db.set_ticket(ticket.id, status="open", resume_attempt=att.id)
        return
    if decision == "error":
        infra_error(db, ticket, att.id, f"verifier produced no decision: {v.get('summary', '')}")
        db.set_ticket(ticket.id, resume_attempt=att.id)
        return
    if decision == "accept":
        db.set_ticket(ticket.id, status="merging")
        with MERGE_LOCK:
            m = merge.merge(cfg, ticket, att.id, att.head, decision, att.dir)
        write_json(Path(att.dir) / "merge.json", m)
        if not m["merged"]:
            log(f"{ticket.id}: merge refused: {m['reason'][:200]}")
            db.note(ticket.id, att.id, f"accepted by the verifier but not merged: {m['reason'][:600]}")
            db.set_ticket(ticket.id, status="open", salvage_attempt=att.id)
        else:
            db.set_attempt(att.id, merged=1, new_passes=m["new_passes"])
            log(f"{ticket.id}: merged (+{m['new_passes']}, {m['total_pass']} total)")
            if m["target_complete"]:
                db.set_ticket(ticket.id, status="done", strikes=0, salvage_attempt=None, reason="")
            else:
                left = "\n".join(m["remaining"][:10])
                db.note(ticket.id, att.id, f"merged partial progress. Still failing:\n{left}")
                db.set_ticket(ticket.id, status="open", strikes=0, salvage_attempt=None)
    elif decision == "revise":
        db.note(ticket.id, att.id, f"not merged ({', '.join(v['defects'])}); continue from this diff. {notes}")
        row = db.ticket(ticket.id)
        if row["attempts"] >= cfg.escalation.get("max_attempts", 6):
            db.set_ticket(ticket.id, status="needs-architect", reason="ran out of attempts while revising")
        else:
            db.set_ticket(ticket.id, status="open", salvage_attempt=att.id)
    else:
        # Faking is evidence the ticket is beyond this tier: escalate at once.
        weight = cfg.escalation.get("strikes_per_tier", 2) if v.get("fakery") and \
            cfg.escalation.get("fakery_escalates", True) else 1
        strike(cfg, db, ticket, att.id,
               f"rejected ({', '.join(v['defects'])}): {v.get('summary', '')[:300]} {notes}", weight=weight)
    db.set_attempt(att.id, finished=time.time())


def build_tools(cfg):
    """Builds the trusted runner from the main checkout into work/bin/t262."""
    env = dict(os.environ, CARGO_TARGET_DIR=str(WORK / "orch-target"))
    run(["cargo", "build", "-q", "--release", "-p", "t262"], cwd=ROOT, env=env)
    (WORK / "bin").mkdir(parents=True, exist_ok=True)
    shutil.copy2(WORK / "orch-target" / "release" / "t262", gates.T262)
