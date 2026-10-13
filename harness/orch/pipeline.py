"""One ticket through the pipeline: worker → gates → blind verifier → merge,
and the bookkeeping that decides what happens to the ticket next."""

import threading
from pathlib import Path

import gates
import merge
import verifier
import worker
from common import log, run, write_json

MERGE_LOCK = threading.Lock()


def next_tier(cfg, tier):
    tiers = cfg.escalation.get("tiers", ["haiku", "sonnet", "opus"])
    i = tiers.index(tier) if tier in tiers else len(tiers) - 1
    return tiers[i + 1] if i + 1 < len(tiers) else None


def strike(cfg, db, ticket, aid, reason):
    row = db.ticket(ticket.id)
    db.note(ticket.id, aid, reason)
    strikes = row["strikes"] + 1
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


def process(cfg, db, ticket, slot):
    row = db.ticket(ticket.id)
    tier = row["tier"]
    log(f"{ticket.id}: attempt on slot {slot} with {tier}")
    att = worker.attempt(cfg, db, ticket, slot, tier, salvage_attempt=row["salvage_attempt"])
    log(f"{ticket.id}: attempt {att.id} finished (${att.cost:.3f}, {att.turns} turns)")

    if att.error and not att.head:
        db.set_attempt(att.id, outcome="error", finished=_now())
        db.note(ticket.id, att.id, f"worker run failed: {att.error[:300]}")
        db.set_ticket(ticket.id, status="open")
        return
    if att.blocked and not att.head:
        db.set_attempt(att.id, outcome="blocked", finished=_now())
        db.note(ticket.id, att.id, f"BLOCKED: {att.blocked}")
        db.set_ticket(ticket.id, status="blocked", reason=att.blocked)
        log(f"{ticket.id}: blocked: {att.blocked}")
        return
    if att.blocked:
        # A diff still gets judged on its merits; the reason is kept for context.
        db.note(ticket.id, att.id, f"worker reported BLOCKED but changed files: {att.blocked}")
    if not att.head:
        db.set_attempt(att.id, outcome="no-diff", finished=_now())
        strike(cfg, db, ticket, att.id, "produced no changes")
        return

    rep = gates.check_attempt(cfg, ticket, slot, att.slot_dir, att.base, att.head, att.dir)
    gate_text = gates.render(rep)
    if rep["decision"] == "reject":
        db.set_attempt(att.id, outcome="gate-reject", finished=_now())
        log(f"{ticket.id}: gate rejected attempt {att.id}: {'; '.join(rep['reject'])[:200]}")
        strike(cfg, db, ticket, att.id, "rejected by gates: " + "; ".join(rep["reject"])[:600])
        return

    diff = Path(att.dir, "diff.patch").read_text()
    v, vcost = verifier.verify(cfg, ticket, worker.spec_text(ticket.spec), gate_text, diff,
                               record=Path(att.dir) / "verifier.json")
    write_json(Path(att.dir) / "verdict.json", v)
    verdict = v["verdict"]
    db.set_attempt(att.id, outcome=verdict, verifier_cost=vcost)
    log(f"{ticket.id}: verifier says {verdict} ({v.get('summary', '')[:160]})")

    if verdict == "error":
        db.note(ticket.id, att.id, "verifier failed to produce a verdict; retrying")
        db.set_ticket(ticket.id, status="open")
    elif verdict in ("good", "incomplete"):
        db.set_ticket(ticket.id, status="merging")
        with MERGE_LOCK:
            m = merge.merge(cfg, ticket, att.id, att.head, verdict, att.dir)
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
                db.note(ticket.id, att.id,
                        f"partial progress merged ({verdict}). Still failing:\n{left}\n"
                        f"Reviewer: {v.get('notes_for_next_attempt', '')}")
                db.set_ticket(ticket.id, status="open", strikes=0, salvage_attempt=None)
    elif verdict == "salvageable":
        db.note(ticket.id, att.id, f"salvageable, not merged. Reviewer: {v.get('notes_for_next_attempt', '')}")
        row = db.ticket(ticket.id)
        if row["attempts"] >= cfg.escalation.get("max_attempts", 6):
            db.set_ticket(ticket.id, status="needs-architect", reason="ran out of attempts while salvageable")
        else:
            db.set_ticket(ticket.id, status="open", salvage_attempt=att.id)
    else:
        why = "garbage" + (" (fakery)" if v.get("fakery") else "")
        strike(cfg, db, ticket, att.id, f"{why}: {v.get('summary', '')} {v.get('notes_for_next_attempt', '')}")
    db.set_attempt(att.id, finished=_now())


def _now():
    import time
    return time.time()


def build_tools(cfg):
    """Builds the trusted runner from the main checkout into work/bin/t262."""
    from common import ROOT, WORK
    import os
    import shutil
    env = dict(os.environ, CARGO_TARGET_DIR=str(WORK / "orch-target"))
    run(["cargo", "build", "-q", "--release", "-p", "t262"], cwd=ROOT, env=env)
    (WORK / "bin").mkdir(parents=True, exist_ok=True)
    shutil.copy2(WORK / "orch-target" / "release" / "t262", gates.T262)
