#!/usr/bin/env python3
"""Real-world tests: npm packages run under kahvi-run, checked against Node.

Each realworld/cases/NAME.mjs imports packages from realworld/node_modules
(pinned in package.json and package-lock.json) and prints plain strings.
Node's stdout for it, recorded with --record, is NAME.out. A case passes when
kahvi-run prints exactly NAME.out and exits 0. Both run with TZ=UTC and no
other environment beyond PATH.

expected-pass.txt is the ratchet: a case listed there that fails is a
regression, and the run exits 1. Other failures are reported, grouped by the
first line of stderr, but do not fail the run.

  ./rw [NAME...]                  run cases (all by default)
  ./rw --engine PATH              use this kahvi-run (default: release build)
  ./rw --gc-stress                collect at every safe point
  ./rw --record [NAME...]         rewrite NAME.out from Node
  ./rw --update-expected          rewrite expected-pass.txt from this run
  ./rw --json FILE                also write per-case results as JSON
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import time
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
CASES = HERE / "cases"
EXPECTED = HERE / "expected-pass.txt"


def ensure_packages():
    if (HERE / "node_modules").is_dir():
        return
    if not shutil.which("npm"):
        sys.exit("rw: realworld/node_modules is missing and npm is not on PATH")
    print("rw: installing pinned packages (npm ci)", flush=True)
    subprocess.run(["npm", "ci", "--ignore-scripts", "--no-audit", "--no-fund"], cwd=HERE, check=True)


def case_env(gc_stress=False):
    env = {"PATH": os.environ.get("PATH", ""), "TZ": "UTC"}
    if gc_stress:
        env["KAHVI_GC_STRESS"] = "1"
    return env


def record(cases):
    node = shutil.which("node")
    if not node:
        sys.exit("rw: --record needs node on PATH")
    for c in cases:
        p = subprocess.run([node, c.name], cwd=CASES, env=case_env(), capture_output=True, text=True)
        if p.returncode != 0:
            sys.exit(f"rw: node failed on {c.stem} (exit {p.returncode}):\n{p.stderr}")
        c.with_suffix(".out").write_text(p.stdout)
        print(f"recorded {c.stem}")


def run_case(engine, case, gc_stress, timeout):
    want = case.with_suffix(".out").read_text()
    start = time.monotonic()
    try:
        p = subprocess.run([engine, case.name], cwd=CASES, env=case_env(gc_stress),
                           capture_output=True, text=True, timeout=timeout)
        out, err, status = p.stdout, p.stderr, p.returncode
    except subprocess.TimeoutExpired as e:
        out = e.stdout.decode(errors="replace") if isinstance(e.stdout, bytes) else (e.stdout or "")
        err, status = f"timeout after {timeout}s", None
    secs = round(time.monotonic() - start, 2)
    if status == 0 and out == want:
        return {"name": case.stem, "pass": True, "secs": secs}
    if status is None:
        reason = err
    elif status != 0:
        reason = (err.strip().splitlines() or [f"exit {status}"])[0]
        if status >= 101:
            reason = f"crash (exit {status}): {reason}"
    else:
        reason = "stdout differs from Node"
    detail = ""
    if status == 0 or status is None:
        got_lines, want_lines = out.splitlines(), want.splitlines()
        for i in range(max(len(got_lines), len(want_lines))):
            g = got_lines[i] if i < len(got_lines) else "<missing>"
            w = want_lines[i] if i < len(want_lines) else "<missing>"
            if g != w:
                detail = f"line {i + 1}:\n    node:  {w[:200]}\n    kahvi: {g[:200]}"
                break
    return {"name": case.stem, "pass": False, "secs": secs, "status": status, "reason": reason, "detail": detail}


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--engine", help="kahvi-run binary (default: build target/release/kahvi-run)")
    ap.add_argument("--gc-stress", action="store_true", help="collect at every safe point")
    ap.add_argument("--timeout", type=int, default=300, help="seconds per case (default 300)")
    ap.add_argument("--record", action="store_true", help="rewrite the .out files from Node")
    ap.add_argument("--update-expected", action="store_true", help="rewrite expected-pass.txt")
    ap.add_argument("--json", metavar="FILE", help="write per-case results to FILE")
    ap.add_argument("names", nargs="*")
    args = ap.parse_args()

    ensure_packages()
    cases = sorted(CASES.glob("*.mjs"))
    if args.names:
        unknown = set(args.names) - {c.stem for c in cases}
        if unknown:
            sys.exit(f"rw: no such case: {', '.join(sorted(unknown))}")
        cases = [c for c in cases if c.stem in args.names]
    if args.record:
        record(cases)
        return

    engine = args.engine
    if not engine:
        subprocess.run(["cargo", "build", "-q", "--release", "-p", "kahvi-runtime"], cwd=ROOT, check=True)
        engine = str(ROOT / "target" / "release" / "kahvi-run")
    engine = str(Path(engine).resolve())

    expected = set(EXPECTED.read_text().split()) if EXPECTED.exists() else set()
    results = []
    for c in cases:
        r = run_case(engine, c, args.gc_stress, args.timeout)
        results.append(r)
        if r["pass"]:
            print(f"ok   {r['name']} ({r['secs']}s)", flush=True)
        else:
            print(f"FAIL {r['name']} ({r['secs']}s): {r['reason']}", flush=True)
            if r["detail"]:
                print("  " + r["detail"], flush=True)

    passed = sorted(r["name"] for r in results if r["pass"])
    failed = [r for r in results if not r["pass"]]
    groups = defaultdict(list)
    for r in failed:
        groups[r["reason"]].append(r["name"])
    if groups:
        print("\nfailures by cause:")
        for reason, names in sorted(groups.items(), key=lambda kv: (-len(kv[1]), kv[0])):
            print(f"  {len(names):3}  {reason}  [{', '.join(names)}]")
    regressions = sorted(r["name"] for r in failed if r["name"] in expected)
    new_passes = sorted(set(passed) - expected)
    print(f"\n{len(passed)}/{len(results)} passed")
    if new_passes:
        print(f"newly passing (not in expected-pass.txt): {', '.join(new_passes)}")
    if regressions:
        print(f"REGRESSIONS (in expected-pass.txt): {', '.join(regressions)}")

    if args.json:
        Path(args.json).write_text(json.dumps({"engine": engine, "gc_stress": args.gc_stress,
                                               "results": results}, indent=1) + "\n")
    if args.update_expected:
        if args.names:
            sys.exit("rw: --update-expected needs a run of every case")
        EXPECTED.write_text("".join(n + "\n" for n in passed))
        print(f"wrote {EXPECTED.relative_to(ROOT)}")
    elif regressions:
        sys.exit(1)


if __name__ == "__main__":
    main()
