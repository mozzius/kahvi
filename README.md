# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93]
```

## Latest run: 2026-10-20T07:39:19Z

Kahvi git 34ce57b, 3 runs each, on Linux-6.17.0-1022-azure-x86_64-with-glibc2.39, Linux-7.0.0-1017-azure-x86_64-with-glibc2.39.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.003s (2.545x) | 0.018s (13.538x) | 0.016s (12.538x) | 0.001s | 0.008s (6.231x) | 0.002s (1.692x) |
| octane/richards | 10.728s (3.66x) | 0.071s (0.036x) | 1.149s (0.392x) | 2.931s | 9.463s (4.756x) | 0.970s (0.488x) |
| octane/deltablue | 9.053s (4.153x) | 0.071s (0.036x) | 0.914s (0.419x) | 2.180s | 10.203s (5.099x) | 1.074s (0.537x) |
| octane/crypto | 23.689s (1.722x) | 0.209s (0.015x) | 4.874s (0.354x) | 13.753s | 33.768s (2.443x) | 3.708s (0.268x) |
| octane/raytrace | 12.988s (4.329x) | 0.136s (0.048x) | 1.075s (0.358x) | 3.001s | 14.867s (5.222x) | 1.844s (0.648x) |
| octane/earley-boyer | 32.358s (3.871x) | 0.268s (0.049x) | 2.464s (0.295x) | 8.359s | 33.400s (6.151x) | 2.814s (0.518x) |
| octane/regexp | 20.138s (2.925x) | 0.347s (0.051x) | 0.900s (0.131x) | 6.885s | 43.951s (6.435x) | 3.455s (0.506x) |
| octane/splay | 0.925s (2.067x) | 0.145s (0.283x) | 0.218s (0.424x) | 0.513s | 1.373s (2.678x) | 0.234s (0.456x) |
| octane/navier-stokes | 3.835s (2.378x) | 0.083s (0.049x) | 0.916s (0.543x) | 1.686s | 5.208s (3.089x) | 0.886s (0.526x) |
| realworld/acorn | 2.635s | 0.142s | 0.383s | – | – | – |
| realworld/marked | 0.788s | 0.086s | 0.087s | – | – | – |
| realworld/prettier | 8.546s | 0.399s | 0.912s | – | – | – |
| realworld/terser | 7.380s | 0.497s | 0.793s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git 34ce57b |
| node | v22.23.3 |
| node-jitless | v22.23.3 |
| quickjs | 0.17.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-10-20T07:39:19Z | git 34ce57b | 2.93x | 9 | [json](results/20261020T073919Z-34ce57be6c.json) |
| 2026-10-18T07:16:59Z | git e8d4c59 | 3.11x | 9 | [json](results/20261018T071659Z-e8d4c59b8e.json) |
| 2026-10-16T07:15:56Z | git 36694ea | 3.80x | 9 | [json](results/20261016T071556Z-36694eab43.json) |
