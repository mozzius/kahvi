"""The blind verifier: judges an attempt from the ticket, the spec, the gate
report and the diff. It never sees the worker's transcript or summary."""

import llm
from common import PROMPTS, WORK

MAX_DIFF_CHARS = 60_000

CHECKS = {
    "special_cases_inputs": "Some branch, table or constant singles out particular input values, strings, "
                            "names or lengths in a way that looks chosen to produce expected outputs (e.g. "
                            "a table of known answers, or a branch for exact test values). Not this: values "
                            "the spec itself singles out, or an honest implementation limit whose path "
                            "throws a not-implemented error.",
    "made_up_results": "On some path the code returns or produces something the spec algorithm would not "
                       "(a default, the input unchanged, a guess, a shortcut answer) instead of computing it "
                       "or throwing a not-implemented error.",
    "swallows_errors": "Some abrupt completion that the spec propagates with ? is ignored, replaced by a "
                       "default, or turned into a different error.",
    "skips_observable_steps": "A spec step with observable effects (property access, calls, conversions) is "
                              "missing, reordered, or replaced by direct access to internals.",
}

SCHEMA = {
    "type": "object",
    "properties": {
        "checks": {
            "type": "object",
            "properties": {k: {"type": "boolean", "description": v} for k, v in CHECKS.items()},
            "required": list(CHECKS),
            "additionalProperties": False,
        },
        "verdict": {"type": "string", "enum": ["good", "incomplete", "salvageable", "garbage"]},
        "confidence": {"type": "number"},
        "fakery": {"type": "boolean"},
        "summary": {"type": "string"},
        "problems": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "file": {"type": "string"},
                    "line": {"type": "integer"},
                    "severity": {"type": "string", "enum": ["blocker", "major", "minor"]},
                    "issue": {"type": "string"},
                },
                "required": ["file", "severity", "issue"],
                "additionalProperties": False,
            },
        },
        "notes_for_next_attempt": {"type": "string"},
    },
    "required": ["checks", "verdict", "confidence", "fakery", "summary", "problems", "notes_for_next_attempt"],
    "additionalProperties": False,
}


def enforce(v):
    """The checklist binds the verdict: a model that answers "yes" to any
    check cannot also wave the change through. Returns the adjusted verdict."""
    checks = v.get("checks", {})
    verdict = v["verdict"]
    if checks.get("special_cases_inputs"):
        verdict = "garbage"
    elif verdict in ("good", "incomplete") and any(
        checks.get(k) for k in ("made_up_results", "swallows_errors", "skips_observable_steps")
    ):
        verdict = "salvageable"
    if verdict != v["verdict"]:
        failed = [k for k, val in checks.items() if val]
        v["model_verdict"] = v["verdict"]
        v["verdict"] = verdict
        v["summary"] = f"[downgraded from {v['model_verdict']}: {', '.join(failed)}] {v.get('summary', '')}"
    return v


def render_case(ticket, spec, gate_text, diff):
    if len(diff) > MAX_DIFF_CHARS:
        diff = diff[:MAX_DIFF_CHARS] + f"\n... diff truncated ({len(diff)} chars total) ..."
    return (PROMPTS / "verifier-case.md").read_text().format(
        id=ticket.id,
        title=ticket.title,
        body=ticket.body or "(no notes)",
        scope=", ".join(ticket.scope),
        target=", ".join(ticket.target) or "(none)",
        spec=spec,
        gate=gate_text,
        diff=diff,
    )


STRICTNESS = ["good", "incomplete", "salvageable", "garbage"]


def verify_pipeline(cfg, ticket, spec, gate_text, diff, record_dir=None):
    """The configured cascade: the screening model judges every attempt; the
    confirming model re-judges anything the screen would merge, and the
    stricter verdict wins. A cheat has to fool both to land."""
    rec = (lambda name: record_dir / name) if record_dir else (lambda name: None)
    v, cost = verify(cfg, ticket, spec, gate_text, diff, record=rec("verifier.json"))
    confirm = cfg.verifier.get("confirm")
    if v["verdict"] not in ("good", "incomplete") or not confirm:
        return v, cost
    v2, cost2 = verify(cfg, ticket, spec, gate_text, diff, record=rec("verifier-confirm.json"), model=confirm)
    if v2["verdict"] == "error":
        return v2, cost + cost2
    if STRICTNESS.index(v2["verdict"]) > STRICTNESS.index(v["verdict"]):
        v2["summary"] = f"[confirming verifier overruled {v['verdict']}] {v2.get('summary', '')}"
        return v2, cost + cost2
    return v, cost + cost2


def verify(cfg, ticket, spec, gate_text, diff, record=None, model=None):
    """Returns (verdict dict, cost). Failures come back as verdict "error"."""
    vcfg = cfg.verifier
    result = llm.claude(
        render_case(ticket, spec, gate_text, diff),
        model=model or cfg.model("verifier"),
        cwd=WORK,
        tools=[],
        max_turns=4,
        budget_usd=vcfg.get("max_budget_usd"),
        effort=vcfg.get("effort"),
        json_schema=SCHEMA,
        system_prompt=(PROMPTS / "verifier.md").read_text(),
        timeout=900,
        record=record,
    )
    cost = float(result.get("total_cost_usd") or 0)
    out = result.get("structured_output")
    if not isinstance(out, dict) or out.get("verdict") not in SCHEMA["properties"]["verdict"]["enum"]:
        return {"verdict": "error", "summary": str(result.get("error") or result.get("result"))[:500]}, cost
    return enforce(out), cost
