#!/usr/bin/env python3
"""End-to-end tests of the runtime (kahvi-run): what test262 cannot see
(console output, the event loop, module loading from files, exit statuses).

Each harness/e2e/tests/NAME.mjs runs as `kahvi-run NAME.mjs`; its stdout
must equal NAME.out, its stderr NAME.err (empty if absent), and its exit
status the `// exit: N` comment (0 if absent). Files in tests/lib/ are
imported by the tests, not run.

  ./e2e [--engine PATH] [--gc-stress] [NAME...]
"""

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TESTS = Path(__file__).resolve().parent / "tests"


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--engine", help="kahvi-run binary (default: build target/debug/kahvi-run)")
    ap.add_argument("--gc-stress", action="store_true", help="collect at every safe point")
    ap.add_argument("names", nargs="*")
    args = ap.parse_args()
    engine = args.engine
    if not engine:
        subprocess.run(["cargo", "build", "-q", "-p", "kahvi-runtime"], cwd=ROOT, check=True)
        engine = str(ROOT / "target" / "debug" / "kahvi-run")
    env = {"PATH": os.environ.get("PATH", "")}
    if args.gc_stress:
        env["KAHVI_GC_STRESS"] = "1"
    tests = sorted(TESTS.glob("*.mjs"))
    if args.names:
        tests = [t for t in tests if t.stem in args.names]
    failed = []
    for t in tests:
        src = t.read_text()
        m = re.search(r"^// exit: (\d+)$", src, re.M)
        want_status = int(m.group(1)) if m else 0
        want_out = t.with_suffix(".out").read_text() if t.with_suffix(".out").exists() else ""
        want_err = t.with_suffix(".err").read_text() if t.with_suffix(".err").exists() else ""
        try:
            p = subprocess.run([engine, t.name], cwd=TESTS, env=env, capture_output=True, text=True, timeout=60)
            got = (p.stdout, p.stderr, p.returncode)
        except subprocess.TimeoutExpired:
            got = ("", "timeout", None)
        if got != (want_out, want_err, want_status):
            failed.append(t.stem)
            print(f"FAIL {t.stem}")
            for label, g, w in (("stdout", got[0], want_out), ("stderr", got[1], want_err),
                                ("status", got[2], want_status)):
                if g != w:
                    print(f"  {label}: expected {w!r}\n  {label}: got      {g!r}")
        else:
            print(f"ok   {t.stem}")
    print(f"e2e: {len(tests) - len(failed)}/{len(tests)} passed")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
