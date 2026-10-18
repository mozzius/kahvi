# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18"]
  y-axis "times quickjs"
  line [3.80, 3.11]
```

## Latest run: 2026-10-18T07:16:59Z

Kahvi git e8d4c59, 3 runs each, on Linux-6.17.0-1022-azure-x86_64-with-glibc2.39.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (2.75x) | 0.023s (19.167x) | 0.020s (16.917x) | 0.001s | 0.009s (7.167x) | 0.003s (2.667x) |
| octane/richards | 5.836s (4.079x) | 0.065s (0.032x) | 0.582s (0.407x) | 1.431s | 8.489s (4.24x) | 0.976s (0.487x) |
| octane/deltablue | 8.182s (4.167x) | 0.073s (0.037x) | 0.834s (0.425x) | 1.964s | 9.591s (4.901x) | 1.075s (0.549x) |
| octane/crypto | 23.981s (1.744x) | 0.120s (0.015x) | 4.850s (0.353x) | 13.749s | 18.576s (2.267x) | 2.268s (0.277x) |
| octane/raytrace | 11.317s (4.063x) | 0.136s (0.048x) | 0.938s (0.337x) | 2.785s | 14.504s (5.174x) | 1.859s (0.663x) |
| octane/earley-boyer | 31.671s (3.87x) | 0.343s (0.042x) | 2.428s (0.297x) | 8.185s | 50.930s (6.163x) | 4.549s (0.55x) |
| octane/regexp | 11.256s (3.573x) | 0.173s (0.056x) | 0.458s (0.145x) | 3.150s | 22.818s (7.333x) | 1.851s (0.595x) |
| octane/splay | 1.434s (2.171x) | 0.172s (0.29x) | 0.287s (0.482x) | 0.595s | 1.687s (2.834x) | 0.352s (0.592x) |
| octane/navier-stokes | 7.093s (2.725x) | 0.121s (0.055x) | 1.469s (0.664x) | 2.211s | 6.870s (3.107x) | 1.060s (0.479x) |
| realworld/acorn | 4.793s | 0.181s | 0.567s | – | – | – |
| realworld/marked | 1.238s | 0.111s | 0.121s | – | – | – |
| realworld/prettier | 12.299s | 0.485s | 1.272s | – | – | – |
| realworld/terser | 10.841s | 0.581s | 1.039s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git e8d4c59 |
| node | v22.23.3 |
| node-jitless | v22.23.3 |
| quickjs | 0.17.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-10-18T07:16:59Z | git e8d4c59 | 3.11x | 9 | [json](results/20261018T071659Z-e8d4c59b8e.json) |
| 2026-10-16T07:15:56Z | git 36694ea | 3.80x | 9 | [json](results/20261016T071556Z-36694eab43.json) |
