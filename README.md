# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26", "10-27"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93, 3.07, 2.71, 2.89, 2.88, 2.91, 3.03, 2.95]
```

## Latest run: 2026-10-27T07:57:12Z

Kahvi git ab18b24, 3 runs each, on Linux-6.17.0-1022-azure-x86_64-with-glibc2.39.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (2.056x) | 0.022s (14.933x) | 0.021s (14.333x) | 0.002s | 0.010s (6.933x) | 0.003s (2.267x) |
| octane/richards | 8.633s (4.344x) | 0.082s (0.028x) | 0.809s (0.407x) | 1.987s | 12.036s (4.095x) | 1.280s (0.435x) |
| octane/deltablue | 7.733s (3.946x) | 0.092s (0.034x) | 0.832s (0.424x) | 1.960s | 12.616s (4.722x) | 1.375s (0.515x) |
| octane/crypto | 23.245s (1.693x) | 0.234s (0.017x) | 4.866s (0.354x) | 13.733s | 32.831s (2.383x) | 3.672s (0.267x) |
| octane/raytrace | 11.256s (4.157x) | 0.176s (0.045x) | 0.960s (0.355x) | 2.708s | 18.650s (4.727x) | 2.871s (0.728x) |
| octane/earley-boyer | 31.411s (3.785x) | 0.207s (0.047x) | 2.474s (0.298x) | 8.299s | 27.728s (6.36x) | 2.353s (0.54x) |
| octane/regexp | 16.534s (3.159x) | 0.180s (0.054x) | 0.655s (0.125x) | 5.234s | 24.434s (7.323x) | 2.018s (0.605x) |
| octane/splay | 1.494s (2.111x) | 0.188s (0.272x) | 0.327s (0.473x) | 0.691s | 2.092s (3.027x) | 0.375s (0.543x) |
| octane/navier-stokes | 6.998s (2.685x) | 0.136s (0.052x) | 1.670s (0.638x) | 2.617s | 8.254s (3.154x) | 1.469s (0.561x) |
| realworld/acorn | 4.792s | 0.213s | 0.614s | – | – | – |
| realworld/marked | 1.260s | 0.120s | 0.134s | – | – | – |
| realworld/prettier | 12.608s | 0.589s | 1.542s | – | – | – |
| realworld/terser | 10.736s | 0.710s | 1.258s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git ab18b24 |
| node | v22.23.3 |
| node-jitless | v22.23.3 |
| quickjs | 0.17.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-10-27T07:57:12Z | git ab18b24 | 2.95x | 9 | [json](results/20261027T075712Z-ab18b2476d.json) |
| 2026-10-26T07:52:02Z | git 006e586 | 3.03x | 9 | [json](results/20261026T075202Z-006e58682a.json) |
| 2026-10-25T07:38:52Z | git 0614530 | 2.91x | 9 | [json](results/20261025T073852Z-06145300cb.json) |
| 2026-10-24T07:22:14Z | git 965eed5 | 2.88x | 9 | [json](results/20261024T072214Z-965eed51be.json) |
| 2026-10-23T07:37:01Z | git 7071998 | 2.89x | 9 | [json](results/20261023T073701Z-707199806e.json) |
| 2026-10-22T07:39:32Z | git f9066d9 | 2.71x | 9 | [json](results/20261022T073932Z-f9066d9e0c.json) |
| 2026-10-21T07:43:58Z | git 6195d71 | 3.07x | 9 | [json](results/20261021T074358Z-6195d71ab0.json) |
| 2026-10-20T07:39:19Z | git 34ce57b | 2.93x | 9 | [json](results/20261020T073919Z-34ce57be6c.json) |
| 2026-10-18T07:16:59Z | git e8d4c59 | 3.11x | 9 | [json](results/20261018T071659Z-e8d4c59b8e.json) |
| 2026-10-16T07:15:56Z | git 36694ea | 3.80x | 9 | [json](results/20261016T071556Z-36694eab43.json) |
