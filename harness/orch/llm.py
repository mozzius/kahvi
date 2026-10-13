"""Running Claude headlessly (`claude -p`). Works with an API key or a
subscription login, whichever the `claude` CLI is configured with."""

import json
import subprocess
import uuid
from pathlib import Path

from common import write_json


def claude(
    prompt,
    *,
    model,
    cwd,
    tools,
    allowed=(),
    max_turns=30,
    budget_usd=None,
    json_schema=None,
    settings=None,
    effort=None,
    system_append=None,
    system_prompt=None,
    add_dirs=(),
    env=None,
    timeout=3600,
    record=None,
):
    """Runs one headless session. Returns the CLI's JSON result (with an
    `is_error` key either way). `record` is a path to save the raw result."""
    cmd = [
        "claude", "-p",
        "--model", model,
        "--output-format", "json",
        "--max-turns", str(max_turns),
        "--session-id", str(uuid.uuid4()),
        "--no-session-persistence",
        "--strict-mcp-config",
        "--setting-sources", "project",
        "--exclude-dynamic-system-prompt-sections",
        "--permission-mode", "dontAsk",
        "--permission-prompts", "none",
        "--tools", ",".join(tools),
    ]
    if allowed:
        cmd += ["--allowedTools", " ".join(allowed)]
    if budget_usd:
        cmd += ["--max-budget-usd", str(budget_usd)]
    if json_schema:
        cmd += ["--json-schema", json.dumps(json_schema)]
    if settings:
        cmd += ["--settings", json.dumps(settings)]
    if effort:
        cmd += ["--effort", effort]
    if system_prompt:
        cmd += ["--system-prompt", system_prompt]
    if system_append:
        cmd += ["--append-system-prompt", system_append]
    for d in add_dirs:
        cmd += ["--add-dir", str(d)]
    try:
        proc = subprocess.run(
            cmd, input=prompt, cwd=cwd, env=env, timeout=timeout, text=True,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        )
        try:
            result = json.loads(proc.stdout)
        except json.JSONDecodeError:
            result = {"is_error": True, "error": f"exit {proc.returncode}: {proc.stdout[-2000:]}{proc.stderr[-2000:]}"}
    except subprocess.TimeoutExpired:
        result = {"is_error": True, "error": f"timed out after {timeout}s"}
    result.setdefault("total_cost_usd", 0.0)
    result.setdefault("num_turns", 0)
    if record:
        slim = {k: v for k, v in result.items() if k not in ("modelUsage", "subagent_stats")}
        write_json(Path(record), slim)
    return result
