# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26", "10-27", "10-28", "10-30", "10-31", "11-01", "11-03", "11-04", "11-05", "11-06", "11-07", "11-09"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93, 3.07, 2.71, 2.89, 2.88, 2.91, 3.03, 2.95, 2.99, 2.95, 2.84, 2.85, 3.77, 3.35, 3.48, 3.65, 3.71, 3.44]
```

## Latest run: 2026-11-09T09:53:46Z

Kahvi git 0ee82a5, 3 runs each, on Linux-7.0.0-1019-azure-x86_64-with-glibc2.43.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (1.483x) | 0.023s (14.437x) | 0.025s (15.375x) | 0.002s | 0.009s (5.688x) | 0.002s (1.438x) |
| octane/richards | 6.325s (4.774x) | 0.082s (0.035x) | 0.582s (0.439x) | 1.325s | 11.246s (4.818x) | 1.278s (0.547x) |
| octane/deltablue | 9.326s (3.949x) | 0.093s (0.04x) | 1.018s (0.431x) | 2.362s | 12.128s (5.215x) | 1.377s (0.592x) |
| octane/crypto | 23.776s (3.456x) | 0.179s (0.026x) | 4.863s (0.707x) | 6.879s | 32.697s (4.809x) | 3.689s (0.543x) |
| octane/raytrace | 14.617s (4.077x) | 0.158s (0.044x) | 1.290s (0.36x) | 3.586s | 18.513s (5.197x) | 2.830s (0.794x) |
| octane/earley-boyer | 30.814s (4.212x) | 0.349s (0.056x) | 2.386s (0.326x) | 7.316s | 44.942s (7.225x) | 4.070s (0.654x) |
| octane/regexp | 13.054s (3.913x) | 0.280s (0.05x) | 0.496s (0.149x) | 3.336s | 34.829s (6.169x) | 2.877s (0.51x) |
| octane/splay | 1.451s (2.523x) | 0.159s (0.261x) | 0.275s (0.452x) | 0.608s | 2.155s (3.543x) | 0.459s (0.754x) |
| octane/navier-stokes | 7.130s (4.154x) | 0.139s (0.127x) | 1.465s (1.334x) | 1.098s | 7.790s (7.095x) | 1.326s (1.208x) |
| realworld/acorn | 4.529s | 0.183s | 0.571s | – | – | – |
| realworld/marked | 1.211s | 0.101s | 0.122s | – | – | – |
| realworld/prettier | 12.342s | 0.477s | 1.354s | – | – | – |
| realworld/terser | 10.700s | 0.538s | 1.169s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git 0ee82a5 |
| node | v24.22.2 |
| node-jitless | v24.22.2 |
| quickjs | 0.18.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-11-09T09:53:46Z | git 0ee82a5 | 3.44x | 9 | [json](results/20261109T095346Z-0ee82a5dcc.json) |
| 2026-11-07T08:51:32Z | git bee1b4e | 3.71x | 9 | [json](results/20261107T085132Z-bee1b4ebd9.json) |
| 2026-11-06T09:28:43Z | git c91b8e8 | 3.65x | 9 | [json](results/20261106T092843Z-c91b8e8d8b.json) |
| 2026-11-05T09:34:11Z | git 663f862 | 3.48x | 9 | [json](results/20261105T093411Z-663f862552.json) |
| 2026-11-04T09:22:01Z | git 98ed7da | 3.35x | 9 | [json](results/20261104T092201Z-98ed7da385.json) |
| 2026-11-03T09:38:44Z | git b236288 | 3.77x | 9 | [json](results/20261103T093844Z-b236288e30.json) |
| 2026-11-01T09:20:36Z | git 100ced3 | 2.85x | 9 | [json](results/20261101T092036Z-100ced3d4d.json) |
| 2026-10-31T08:29:36Z | git a3dc6de | 2.84x | 9 | [json](results/20261031T082936Z-a3dc6de88e.json) |
| 2026-10-30T08:43:18Z | git b75ecfe | 2.95x | 9 | [json](results/20261030T084318Z-b75ecfe641.json) |
| 2026-10-28T08:39:01Z | git 1a1d9f0 | 2.99x | 9 | [json](results/20261028T083901Z-1a1d9f0872.json) |
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
