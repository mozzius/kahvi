#!/usr/bin/env python3
"""Add one bench/run.py --json result to a results directory and rewrite its
README.md dashboard: the latest table, and Kahvi's speed relative to the
baseline engine over time.

  bench/publish.py RESULT.json DIR [--sha SHA]

DIR is a checkout of the bench-results branch. Results live in
DIR/results/<date>-<sha>.json; README.md is generated from all of them.
"""

import argparse
import json
import math
import shutil
from pathlib import Path


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs)) if xs else None


def load_all(results_dir):
    runs = []
    for p in sorted(results_dir.glob("*.json")):
        data = json.loads(p.read_text())
        data["file"] = p.name
        runs.append(data)
    return runs


def kahvi_ratio(run):
    """Geometric mean over workloads of Kahvi's time / the baseline's."""
    ratios = [r["ratio_to_baseline"] for r in run["results"]
              if r["engine"] == "kahvi" and "ratio_to_baseline" in r]
    return geomean(ratios), len(ratios)


def latest_table(run):
    results = run["results"]
    engines = list(run["meta"]["engines"])
    workloads = list(dict.fromkeys(r["workload"] for r in results))
    by = {(r["workload"], r["engine"]): r for r in results}
    baseline = run["meta"].get("baseline")
    rows = ["| Workload | " + " | ".join(engines) + " |", "|---" * (len(engines) + 1) + "|"]
    for w in workloads:
        cells = []
        for e in engines:
            r = by.get((w, e))
            if r is None:
                cells.append("–")
            elif "error" in r:
                cells.append("fail")
            else:
                ratio = f" ({r['ratio_to_baseline']}x)" if "ratio_to_baseline" in r and e != baseline else ""
                cells.append(f"{r['median']:.3f}s{ratio}")
        rows.append(f"| {w} | " + " | ".join(cells) + " |")
    return "\n".join(rows)


def readme(runs):
    latest = runs[-1]
    meta = latest["meta"]
    baseline = meta.get("baseline") or "none"
    out = ["# Kahvi benchmark results", "",
           "Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). "
           "Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with "
           f"{baseline} measured in the same run, so lower is faster.", ""]

    history = [(r, *kahvi_ratio(r)) for r in runs]
    history = [(r, g, n) for r, g, n in history if g is not None]
    if len(history) >= 2:
        labels = ", ".join(f'"{r["meta"]["date"][5:10]}"' for r, _, _ in history[-60:])
        values = ", ".join(f"{g:.2f}" for _, g, _ in history[-60:])
        out += ["## Kahvi over time", "",
                f"Geometric mean of Kahvi's time relative to {baseline}, across the workloads both ran. "
                "Lower is faster.", "",
                "```mermaid", "xychart-beta", f'  x-axis [{labels}]', f'  y-axis "times {baseline}"',
                f"  line [{values}]", "```", ""]

    out += [f"## Latest run: {meta['date']}", "",
            f"Kahvi {meta['engines'].get('kahvi', {}).get('version', '?')}, {meta['runs']} runs each, "
            f"on {meta['machine']}.", "", latest_table(latest), "",
            "| Engine | Version |", "|---|---|"]
    out += [f"| {e} | {v['version']} |" for e, v in meta["engines"].items()]
    out += ["", "## History", "", f"| Date | Kahvi commit | Kahvi vs {baseline} (geomean) | Workloads | File |",
            "|---|---|---|---|---|"]
    for r, g, n in reversed(history):
        out.append(f"| {r['meta']['date']} | {r['meta']['engines']['kahvi']['version']} | {g:.2f}x | {n} | "
                   f"[json](results/{r['file']}) |")
    return "\n".join(out) + "\n"


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("result")
    ap.add_argument("dir")
    ap.add_argument("--sha", default="local")
    args = ap.parse_args()
    data = json.loads(Path(args.result).read_text())
    results_dir = Path(args.dir) / "results"
    results_dir.mkdir(parents=True, exist_ok=True)
    stamp = data["meta"]["date"].replace(":", "").replace("-", "")
    shutil.copy(args.result, results_dir / f"{stamp}-{args.sha[:10]}.json")
    (Path(args.dir) / "README.md").write_text(readme(load_all(results_dir)))


if __name__ == "__main__":
    main()
