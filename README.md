# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20", "10-21", "10-22"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93, 3.07, 2.71]
```

## Latest run: 2026-10-22T07:39:32Z

Kahvi git f9066d9, 3 runs each, on Linux-6.17.0-1022-azure-x86_64-with-glibc2.39, Linux-7.0.0-1017-azure-x86_64-with-glibc2.39.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.003s (2.077x) | 0.019s (11.875x) | 0.018s (11.0x) | 0.002s | 0.007s (4.438x) | 0.002s (1.5x) |
| octane/richards | 10.248s (3.492x) | 0.069s (0.035x) | 1.125s (0.383x) | 2.935s | 8.503s (4.292x) | 0.972s (0.49x) |
| octane/deltablue | 9.653s (3.592x) | 0.074s (0.037x) | 1.047s (0.39x) | 2.688s | 9.779s (4.933x) | 1.091s (0.55x) |
| octane/crypto | 11.859s (1.461x) | 0.194s (0.014x) | 2.360s (0.291x) | 8.116s | 35.101s (2.545x) | 3.673s (0.266x) |
| octane/raytrace | 13.777s (3.442x) | 0.134s (0.048x) | 1.317s (0.329x) | 4.003s | 14.546s (5.217x) | 1.850s (0.664x) |
| octane/earley-boyer | 32.090s (3.888x) | 0.347s (0.042x) | 2.450s (0.297x) | 8.254s | 51.770s (6.229x) | 4.640s (0.558x) |
| octane/regexp | 20.232s (2.973x) | 0.346s (0.051x) | 0.887s (0.13x) | 6.805s | 43.754s (6.435x) | 3.406s (0.501x) |
| octane/splay | 1.079s (1.972x) | 0.148s (0.286x) | 0.253s (0.49x) | 0.517s | 1.582s (3.06x) | 0.364s (0.705x) |
| octane/navier-stokes | 6.959s (2.663x) | 0.108s (0.056x) | 1.286s (0.672x) | 1.915s | 6.005s (3.136x) | 0.910s (0.475x) |
| realworld/acorn | 3.044s | 0.159s | 0.493s | – | – | – |
| realworld/marked | 0.858s | 0.097s | 0.103s | – | – | – |
| realworld/prettier | 12.167s | 0.440s | 1.103s | – | – | – |
| realworld/terser | 10.716s | 0.513s | 0.900s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git f9066d9 |
| node | v22.23.3 |
| node-jitless | v22.23.3 |
| quickjs | 0.17.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-10-22T07:39:32Z | git f9066d9 | 2.71x | 9 | [json](results/20261022T073932Z-f9066d9e0c.json) |
| 2026-10-21T07:43:58Z | git 6195d71 | 3.07x | 9 | [json](results/20261021T074358Z-6195d71ab0.json) |
| 2026-10-20T07:39:19Z | git 34ce57b | 2.93x | 9 | [json](results/20261020T073919Z-34ce57be6c.json) |
| 2026-10-18T07:16:59Z | git e8d4c59 | 3.11x | 9 | [json](results/20261018T071659Z-e8d4c59b8e.json) |
| 2026-10-16T07:15:56Z | git 36694ea | 3.80x | 9 | [json](results/20261016T071556Z-36694eab43.json) |
