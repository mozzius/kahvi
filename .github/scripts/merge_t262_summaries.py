#!/usr/bin/env python3
"""Merge the per-shard t262 summaries of one sharded run into one.

Usage: merge_t262_summaries.py [--allow-timeouts] LABEL EXPECTED_SHARDS OUT_JSON SHARD_PREFIX...

Each SHARD_PREFIX names a shard's PREFIX-summary.json and
PREFIX-results.jsonl (see t262_shard.sh). Writes the combined summary (same shape as `t262 run --summary-json`, so
readme_pass_rate.py reads it) and prints the regressions. Exits 1 on any
regression or a missing shard, 3 if any shard's canaries failed. A shard
whose summary is missing (killed, timed out, crashed) fails the run rather
than shrinking it.

--allow-timeouts is for the GC-stress run: collecting at every safe point
makes the heaviest tests (RegExp property escapes, exhaustive decodeURI)
too slow to finish, so a baseline pass that times out is listed but does not
fail the run. A baseline pass that fails or crashes under stress still does:
that is the rooting-bug signal.
"""
import json
import sys

COUNTS = ("files", "skipped_files", "runs", "pass", "fail", "timeout", "crash", "not_implemented")


def main() -> int:
    args = sys.argv[1:]
    allow_timeouts = args[:1] == ["--allow-timeouts"]
    if allow_timeouts:
        args = args[1:]
    label, expected, out_path, *prefixes = args
    shards, status = [], {}
    for p in prefixes:
        with open(f"{p}-summary.json") as f:
            shards.append(json.load(f))
        with open(f"{p}-results.jsonl") as f:
            for line in f:
                r = json.loads(line)
                status[r["id"]] = r["status"]
    merged = {k: sum(s[k] for s in shards) for k in COUNTS}
    merged["engine"] = shards[0]["engine"] if shards else ""
    merged["canaries_ok"] = bool(shards) and all(s["canaries_ok"] for s in shards)
    merged["elapsed_ms"] = max((s["elapsed_ms"] for s in shards), default=0)
    merged["regressions"] = sorted(r for s in shards for r in s["regressions"])
    merged["new_passes"] = sorted(r for s in shards for r in s["new_passes"])
    merged["shards"] = len(shards)
    timed_out = [r for r in merged["regressions"] if status.get(r) == "timeout"]
    if allow_timeouts:
        merged["regressions"] = [r for r in merged["regressions"] if status.get(r) != "timeout"]
        merged["timed_out"] = timed_out
    with open(out_path, "w") as f:
        json.dump(merged, f)
        f.write("\n")

    print(
        f"{label}: {merged['pass']}/{merged['runs']} runs pass across {len(shards)} shards; "
        f"{len(merged['regressions'])} regressions, {len(merged['new_passes'])} new passes"
    )
    for r in merged["regressions"][:200]:
        print(f"  REGRESSED {r}  ({status.get(r, '?')})")
    if allow_timeouts and timed_out:
        print(f"{label}: {len(timed_out)} baseline passes timed out (not counted):")
        for r in timed_out[:200]:
            print(f"  TIMEOUT {r}")
    code = 0
    if len(shards) != int(expected):
        print(f"{label}: only {len(shards)} of {expected} shards reported", file=sys.stderr)
        code = 1
    if merged["regressions"]:
        code = 1
    if not merged["canaries_ok"]:
        print(f"{label}: canaries failed in at least one shard; run invalid", file=sys.stderr)
        code = 3
    return code


if __name__ == "__main__":
    sys.exit(main())
