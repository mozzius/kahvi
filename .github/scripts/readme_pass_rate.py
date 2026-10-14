#!/usr/bin/env python3
"""Rewrite the test262 results table in README.md from a t262 summary.

Usage: readme_pass_rate.py SUMMARY_JSON README_MD

The table lives between the <!-- test262:start --> and <!-- test262:end -->
markers. Exits 2 without touching the README if the run was invalid
(canaries failed), so a broken runner never publishes a number.
"""
import json
import re
import sys

START, END = "<!-- test262:start -->", "<!-- test262:end -->"


def main() -> int:
    summary_path, readme_path = sys.argv[1:3]
    with open(summary_path) as f:
        s = json.load(f)
    if not s["canaries_ok"]:
        print("canaries failed: not publishing a pass rate", file=sys.stderr)
        return 2

    runs, passed = s["runs"], s["pass"]
    pct = 100.0 * passed / runs if runs else 0.0
    table = "\n".join([
        "| test262 | |",
        "|---|--:|",
        f"| **Pass rate** | **{pct:.2f}%** |",
        f"| Passing runs | {passed:,} / {runs:,} |",
        f"| Not implemented | {s['not_implemented']:,} |",
        f"| Test files | {s['files']:,} |",
    ])

    with open(readme_path) as f:
        text = f.read()
    pattern = re.compile(re.escape(START) + r".*?" + re.escape(END), re.S)
    if not pattern.search(text):
        print(f"{readme_path}: markers not found", file=sys.stderr)
        return 1
    new = pattern.sub(lambda _: f"{START}\n\n{table}\n\n{END}", text, count=1)
    with open(readme_path, "w") as f:
        f.write(new)
    print(table)
    return 0


if __name__ == "__main__":
    sys.exit(main())
