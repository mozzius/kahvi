# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20", "10-21", "10-22", "10-23", "10-24"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93, 3.07, 2.71, 2.89, 2.88]
```

## Latest run: 2026-10-24T07:22:14Z

Kahvi git 965eed5, 3 runs each, on Linux-6.17.0-1022-azure-x86_64-with-glibc2.39.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (2.4x) | 0.022s (14.4x) | 0.022s (14.867x) | 0.002s | 0.010s (6.8x) | 0.003s (1.867x) |
| octane/richards | 6.096s (4.06x) | 0.083s (0.028x) | 0.612s (0.408x) | 1.501s | 11.340s (3.847x) | 1.278s (0.433x) |
| octane/deltablue | 9.596s (3.574x) | 0.095s (0.035x) | 1.048s (0.39x) | 2.685s | 12.464s (4.666x) | 1.370s (0.513x) |
| octane/crypto | 23.711s (1.726x) | 0.203s (0.015x) | 4.837s (0.352x) | 13.735s | 33.429s (2.432x) | 3.675s (0.267x) |
| octane/raytrace | 14.489s (3.684x) | 0.171s (0.044x) | 1.326s (0.337x) | 3.933s | 18.976s (4.858x) | 2.833s (0.725x) |
| octane/earley-boyer | 30.475s (3.795x) | 0.349s (0.052x) | 2.412s (0.3x) | 8.031s | 41.432s (6.196x) | 3.587s (0.536x) |
| octane/regexp | 11.735s (3.583x) | 0.203s (0.055x) | 0.483s (0.147x) | 3.275s | 27.057s (7.268x) | 2.247s (0.604x) |
| octane/splay | 1.483s (1.986x) | 0.187s (0.274x) | 0.320s (0.469x) | 0.681s | 2.007s (2.946x) | 0.378s (0.555x) |
| octane/navier-stokes | 3.664s (2.308x) | 0.134s (0.051x) | 1.685s (0.646x) | 2.609s | 8.331s (3.193x) | 1.468s (0.563x) |
| realworld/acorn | 4.682s | 0.207s | 0.607s | – | – | – |
| realworld/marked | 1.260s | 0.121s | 0.132s | – | – | – |
| realworld/prettier | 8.617s | 0.582s | 1.527s | – | – | – |
| realworld/terser | 7.133s | 0.698s | 1.253s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git 965eed5 |
| node | v22.23.3 |
| node-jitless | v22.23.3 |
| quickjs | 0.17.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-10-24T07:22:14Z | git 965eed5 | 2.88x | 9 | [json](results/20261024T072214Z-965eed51be.json) |
| 2026-10-23T07:37:01Z | git 7071998 | 2.89x | 9 | [json](results/20261023T073701Z-707199806e.json) |
| 2026-10-22T07:39:32Z | git f9066d9 | 2.71x | 9 | [json](results/20261022T073932Z-f9066d9e0c.json) |
| 2026-10-21T07:43:58Z | git 6195d71 | 3.07x | 9 | [json](results/20261021T074358Z-6195d71ab0.json) |
| 2026-10-20T07:39:19Z | git 34ce57b | 2.93x | 9 | [json](results/20261020T073919Z-34ce57be6c.json) |
| 2026-10-18T07:16:59Z | git e8d4c59 | 3.11x | 9 | [json](results/20261018T071659Z-e8d4c59b8e.json) |
| 2026-10-16T07:15:56Z | git 36694ea | 3.80x | 9 | [json](results/20261016T071556Z-36694eab43.json) |
