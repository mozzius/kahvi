# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20", "10-21"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93, 3.07]
```

## Latest run: 2026-10-21T07:43:58Z

Kahvi git 6195d71, 3 runs each, on Linux-6.17.0-1022-azure-x86_64-with-glibc2.39, Linux-7.0.0-1017-azure-x86_64-with-glibc2.39.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (3.385x) | 0.020s (15.308x) | 0.020s (15.154x) | 0.001s | 0.010s (7.385x) | 0.002s (1.615x) |
| octane/richards | 10.508s (3.559x) | 0.093s (0.032x) | 1.149s (0.389x) | 2.953s | 11.166s (3.807x) | 1.265s (0.431x) |
| octane/deltablue | 5.982s (4.144x) | 0.096s (0.036x) | 0.573s (0.397x) | 1.444s | 12.160s (4.529x) | 1.388s (0.517x) |
| octane/crypto | 12.097s (1.494x) | 0.203s (0.015x) | 2.369s (0.293x) | 8.095s | 34.503s (2.501x) | 3.705s (0.269x) |
| octane/raytrace | 8.877s (4.31x) | 0.186s (0.047x) | 0.683s (0.332x) | 2.060s | 18.870s (4.788x) | 2.879s (0.73x) |
| octane/earley-boyer | 27.169s (4.532x) | 0.260s (0.043x) | 1.824s (0.304x) | 5.995s | 39.305s (6.496x) | 3.466s (0.573x) |
| octane/regexp | 20.082s (2.944x) | 0.238s (0.046x) | 0.893s (0.131x) | 6.821s | 34.602s (6.635x) | 2.548s (0.489x) |
| octane/splay | 1.486s (2.191x) | 0.171s (0.289x) | 0.267s (0.451x) | 0.592s | 1.931s (3.263x) | 0.347s (0.587x) |
| octane/navier-stokes | 7.373s (2.56x) | 0.111s (0.05x) | 1.271s (0.575x) | 2.212s | 6.559s (2.966x) | 1.205s (0.545x) |
| realworld/acorn | 4.745s | 0.162s | 0.465s | – | – | – |
| realworld/marked | 1.271s | 0.093s | 0.102s | – | – | – |
| realworld/prettier | 13.085s | 0.434s | 1.052s | – | – | – |
| realworld/terser | 11.175s | 0.532s | 0.914s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git 6195d71 |
| node | v22.23.3 |
| node-jitless | v22.23.3 |
| quickjs | 0.17.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-10-21T07:43:58Z | git 6195d71 | 3.07x | 9 | [json](results/20261021T074358Z-6195d71ab0.json) |
| 2026-10-20T07:39:19Z | git 34ce57b | 2.93x | 9 | [json](results/20261020T073919Z-34ce57be6c.json) |
| 2026-10-18T07:16:59Z | git e8d4c59 | 3.11x | 9 | [json](results/20261018T071659Z-e8d4c59b8e.json) |
| 2026-10-16T07:15:56Z | git 36694ea | 3.80x | 9 | [json](results/20261016T071556Z-36694eab43.json) |
