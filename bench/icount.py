#!/usr/bin/env python3
"""Instruction counts for a few short workloads, under cachegrind.

Wall time on a shared CI runner moves by 10% between runs; the number of
instructions executed barely moves, so a 1-2% regression is visible. Each
workload's output is checked as in bench/run.py: a wrong answer is a failure,
not a count.

  bench/icount.py --kahvi-dir DIR --json out.json
  bench/icount.py --compare base.json head.json [--markdown out.md]
"""

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import run as bench  # noqa: E402

# Short enough to run under valgrind (roughly 50x slower) in a few minutes.
WORKLOADS = ["startup/empty", "octane/richards", "octane/deltablue", "octane/crypto", "octane/raytrace",
             "octane/splay", "octane/regexp", "realworld/zod", "realworld/lodash", "realworld/date-fns"]
SCALE = 0.005


def count(argv, cwd, env):
    with tempfile.TemporaryDirectory() as tmp:
        out_file = os.path.join(tmp, "cg.out")
        p = subprocess.run(["valgrind", "--tool=cachegrind", "--cache-sim=no", f"--cachegrind-out-file={out_file}",
                            *argv], cwd=cwd, env=env, capture_output=True, text=True)
        m = re.search(r"I\s+refs:\s+([\d,]+)", p.stderr)
        return p.returncode, p.stdout, p.stderr, int(m.group(1).replace(",", "")) if m else None


def measure(args):
    kahvi_dir = Path(args.kahvi_dir).resolve()
    env = {"PATH": os.environ.get("PATH", ""), "TZ": "UTC", "HOME": os.environ.get("HOME", "")}
    results = {}
    for name in args.workloads or WORKLOADS:
        w = bench.make_workload(name, SCALE)
        template = bench.ENGINES["kahvi"][w.kind]
        argv = [a.format(file=str(w.file), kahvi=str(kahvi_dir / "kahvi"), kahvi_run=str(kahvi_dir / "kahvi-run"),
                         bin="") for a in template]
        status, out, err, irefs = count(argv, w.file.parent, env)
        if status != 0 or irefs is None or not w.check(out):
            results[name] = {"error": (err.strip().splitlines() or [f"exit {status}"])[-1][:200]}
            print(f"{name:24} FAIL {results[name]['error']}", flush=True)
        else:
            results[name] = {"irefs": irefs}
            print(f"{name:24} {irefs:>16,}", flush=True)
    Path(args.json).write_text(json.dumps({"scale": SCALE, "results": results}, indent=1) + "\n")


def compare(args):
    base = json.loads(Path(args.compare[0]).read_text())["results"]
    head = json.loads(Path(args.compare[1]).read_text())["results"]
    rows = ["| Workload | Base | Head | Change |", "|---|---:|---:|---:|"]
    for name in head:
        b, h = base.get(name, {}), head[name]
        if "irefs" in b and "irefs" in h:
            change = (h["irefs"] - b["irefs"]) / b["irefs"] * 100
            flag = " ⚠️" if change > 2 else " ✅" if change < -2 else ""
            rows.append(f"| {name} | {b['irefs']:,} | {h['irefs']:,} | {change:+.2f}%{flag} |")
        else:
            rows.append(f"| {name} | {b.get('irefs', 'fail')} | {h.get('irefs', 'fail')} | – |")
    table = ("### Instruction counts (cachegrind)\n\nInstructions Kahvi executes per workload, base vs this PR. "
             "Changes beyond ±2% are marked.\n\n" + "\n".join(rows) + "\n")
    print(table)
    if args.markdown:
        Path(args.markdown).write_text(table)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("workloads", nargs="*", help="workloads to count (default: the standard set)")
    ap.add_argument("--kahvi-dir")
    ap.add_argument("--json")
    ap.add_argument("--compare", nargs=2, metavar=("BASE", "HEAD"))
    ap.add_argument("--markdown")
    args = ap.parse_args()
    if args.compare:
        compare(args)
    elif args.kahvi_dir and args.json:
        measure(args)
    else:
        ap.error("give --kahvi-dir and --json, or --compare BASE HEAD")


if __name__ == "__main__":
    main()
