#!/usr/bin/env python3
"""Merge the shards of one CI test262 run into the `t262-results` artifact.

Usage: t262_merge.py OUT_DIR BASELINE SHARD_DIR NORMAL_SHARDS GC_SHARDS [--build-failed]

SHARD_DIR holds one directory per shard, t262-normal-K/ and t262-gc-K/, each
with the shard-summary.json and shard-results.jsonl that t262_shard.sh
writes. OUT_DIR gets what the stacker reads (docs/STACKS.md):

  results.jsonl     every run of the normal suite, as `t262 run --json`
  summary.json      as `t262 run --summary-json`, merged across shards, plus
                    "valid" (canaries ok and no shard lost) and the GC-stress
                    result under "gc_stress"
  expected-pass.txt BASELINE updated as `t262 run --write-baseline` would

Regressions are runs listed in BASELINE that did not pass. Under GC stress
(collect at every safe point) the heaviest tests are too slow to finish, so
a baseline pass that times out there is reported but not counted; one that
fails or crashes under stress is a regression like any other.

Exit 0 when valid with no regressions, 1 on regressions, a lost shard or a
failed build, 3 when canaries say a run is invalid.
"""
import json
import os
import sys

COUNTS = ("files", "skipped_files", "runs", "pass", "fail", "timeout", "crash", "not_implemented")
HEADER = (
    "# test262 runs that kahvi passes. Written by the merge bot via `t262 run --write-baseline`.\n"
    "# Do not edit by hand: merges may not remove lines from this file.\n"
)


def read_baseline(path):
    with open(path) as f:
        return {l.strip() for l in f if l.strip() and not l.startswith("#")}


def load(shard_dir, mode, count):
    """The summaries and results of one mode's shards, and how many were lost."""
    summaries, results, lost = [], [], 0
    for k in range(count):
        prefix = os.path.join(shard_dir, f"t262-{mode}-{k}", "shard")
        try:
            with open(f"{prefix}-summary.json") as f:
                s = json.load(f)
            with open(f"{prefix}-results.jsonl") as f:
                rs = [json.loads(l) for l in f if l.strip()]
        except (OSError, ValueError) as e:
            print(f"{mode} shard {k}: no results ({e})", file=sys.stderr)
            lost += 1
            continue
        summaries.append(s)
        results.extend(rs)
    results.sort(key=lambda r: r["id"])
    return summaries, results, lost


def merge(summaries, results, lost, base, allow_timeouts):
    m = {k: sum(s[k] for s in summaries) for k in COUNTS}
    m["engine"] = summaries[0]["engine"] if summaries else ""
    m["canaries_ok"] = bool(summaries) and all(s["canaries_ok"] for s in summaries)
    m["elapsed_ms"] = max((s["elapsed_ms"] for s in summaries), default=0)
    m["shards_lost"] = lost
    m["valid"] = m["canaries_ok"] and lost == 0
    regressions = [r for r in results if r["id"] in base and r["status"] != "pass"]
    timed_out = [r["id"] for r in regressions if r["status"] == "timeout"]
    if allow_timeouts:
        m["timed_out"] = timed_out
        regressions = [r for r in regressions if r["status"] != "timeout"]
    m["regressions"] = [r["id"] for r in regressions]
    m["new_passes"] = [r["id"] for r in results if r["id"] not in base and r["status"] == "pass"]
    return m, regressions


def report(label, m, regressions):
    print(
        f"{label}: {m['pass']}/{m['runs']} runs pass; {len(m['regressions'])} regressions, "
        f"{len(m['new_passes'])} new passes; canaries {'ok' if m['canaries_ok'] else 'FAILED'}, "
        f"{m['shards_lost']} shards lost"
    )
    for r in regressions[:200]:
        print(f"  REGRESSED {r['id']}  {r['status']}: {r.get('reason', '')[:160]}")
    for i in m.get("timed_out", [])[:200]:
        print(f"  TIMEOUT {i}  (not counted under GC stress)")


def main() -> int:
    args = sys.argv[1:]
    build_failed = "--build-failed" in args
    args = [a for a in args if a != "--build-failed"]
    out, baseline, shard_dir, n_normal, n_gc = args[0], args[1], args[2], int(args[3]), int(args[4])
    os.makedirs(out, exist_ok=True)

    if build_failed:
        # The stacker treats this as the layer's fault, not CI's.
        summary = {"valid": True, "build_failed": True, "regressions": [], "new_passes": []}
        with open(os.path.join(out, "summary.json"), "w") as f:
            json.dump(summary, f)
        print("test262: the engine did not build", file=sys.stderr)
        return 1

    base = read_baseline(baseline)
    sums, results, lost = load(shard_dir, "normal", n_normal)
    normal, normal_regs = merge(sums, results, lost, base, False)
    report("test262", normal, normal_regs)
    summary = dict(normal)
    if n_gc:
        gc, gc_regs = merge(*load(shard_dir, "gc", n_gc), base, True)
        report("test262 gc-stress", gc, gc_regs)
        summary["gc_stress"] = {k: gc[k] for k in ("pass", "runs", "canaries_ok", "shards_lost", "valid",
                                                  "regressions", "timed_out")}
        summary["valid"] = normal["valid"] and gc["valid"]
        summary["regressions"] = sorted(set(normal["regressions"]) | set(gc["regressions"]))

    with open(os.path.join(out, "results.jsonl"), "w") as f:
        for r in results:
            f.write(json.dumps(r) + "\n")
    with open(os.path.join(out, "summary.json"), "w") as f:
        json.dump(summary, f)
        f.write("\n")
    # As --write-baseline: drop every id that ran, then add back what passed.
    if normal["valid"]:
        ran = {r["id"] for r in results}
        passed = {r["id"] for r in results if r["status"] == "pass"}
        with open(os.path.join(out, "expected-pass.txt"), "w") as f:
            f.write(HEADER)
            for i in sorted((base - ran) | passed):
                f.write(i + "\n")

    canaries_ok = normal["canaries_ok"] and (not n_gc or summary["gc_stress"]["canaries_ok"])
    if not canaries_ok:
        return 3
    if not summary["valid"] or summary["regressions"]:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
