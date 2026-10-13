"""The blind verifier: judges an attempt from the ticket, the spec, the gate
report and the diff. It never sees the worker's transcript or summary."""

import llm
from common import PROMPTS, WORK

MAX_DIFF_CHARS = 60_000

SCHEMA = {
    "type": "object",
    "properties": {
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
    "required": ["verdict", "confidence", "fakery", "summary", "problems", "notes_for_next_attempt"],
    "additionalProperties": False,
}


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
    return out, cost
