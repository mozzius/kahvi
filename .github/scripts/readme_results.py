#!/usr/bin/env python3
"""Update one suite's row in the README's results table.

Usage: readme_results.py SUITE RESULTS_JSON README_MD

The table lives between <!-- results:start --> and <!-- results:end -->,
one row per suite; this rewrites SUITE's row from that suite's own results
file and leaves the other rows alone, so each suite's CI job keeps its row
current:

  test262   t262-results/summary.json (from .github/scripts/t262_merge.py)
  mjsunit   ./mjs --json FILE
  realworld ./rw --json FILE

Exits 2 without touching the README when the run must not be published: the
test262 canaries failed, or the real-world run was under GC stress.
"""
import json
import re
import sys

START, END = "<!-- results:start -->", "<!-- results:end -->"
HEADER = ["| Suite | Passing | Pass rate | |", "|---|--:|--:|---|"]
ORDER = ["test262", "mjsunit", "realworld"]
LINKS = {
    "test262": "https://github.com/tc39/test262",
    "mjsunit": "mjsunit/run.py",
    "realworld": "realworld/README.md",
}


def test262(s):
    if not s["canaries_ok"]:
        raise Unpublishable("test262 canaries failed")
    return s["pass"], s["runs"], f"{s['files']:,} files, {s['not_implemented']:,} not implemented"


def mjsunit(s):
    passed = sum(1 for r in s["results"] if r["pass"])
    commit = s["v8"]
    return passed, len(s["results"]), (
        f"V8 [{commit[:12]}](https://chromium.googlesource.com/v8/v8/+/{commit}/test/mjsunit); "
        f"{len(s['skipped']):,} skipped (need d8 or V8 internals)")


def realworld(s):
    if s["gc_stress"]:
        raise Unpublishable("real-world run was under GC stress")
    failed = [r["name"] for r in s["results"] if not r["pass"]]
    note = "npm packages and programs, checked byte for byte against Node"
    if failed:
        note += "; failing: " + ", ".join(failed)
    return len(s["results"]) - len(failed), len(s["results"]), note


SUITES = {"test262": test262, "mjsunit": mjsunit, "realworld": realworld}


class Unpublishable(Exception):
    pass


def row(suite, passed, total, note):
    pct = 100.0 * passed / total if total else 0.0
    return f"| [{suite}]({LINKS[suite]}) | {passed:,} / {total:,} | {pct:.2f}% | {note.replace('|', '/')} |"


def main() -> int:
    suite, results_path, readme_path = sys.argv[1:4]
    if suite not in SUITES:
        print(f"unknown suite {suite!r}; known: {', '.join(SUITES)}", file=sys.stderr)
        return 1
    with open(results_path) as f:
        try:
            new_row = row(suite, *SUITES[suite](json.load(f)))
        except Unpublishable as e:
            print(f"{e}: not publishing", file=sys.stderr)
            return 2

    with open(readme_path) as f:
        text = f.read()
    pattern = re.compile(re.escape(START) + r"(.*?)" + re.escape(END), re.S)
    m = pattern.search(text)
    if not m:
        print(f"{readme_path}: results markers not found", file=sys.stderr)
        return 1
    rows = {}
    for line in m.group(1).splitlines():
        name = re.match(r"\| \[([a-z0-9-]+)\]", line)
        if name:
            rows[name.group(1)] = line
    rows[suite] = new_row
    ordered = [rows[s] for s in ORDER if s in rows] + [rows[s] for s in sorted(rows) if s not in ORDER]
    table = "\n".join(HEADER + ordered)
    with open(readme_path, "w") as f:
        f.write(text[:m.start()] + f"{START}\n\n{table}\n\n{END}" + text[m.end():])
    print(new_row)
    return 0


if __name__ == "__main__":
    sys.exit(main())
