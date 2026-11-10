# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26", "10-27", "10-28", "10-30", "10-31", "11-01", "11-03", "11-04", "11-05", "11-06", "11-07", "11-09", "11-10"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93, 3.07, 2.71, 2.89, 2.88, 2.91, 3.03, 2.95, 2.99, 2.95, 2.84, 2.85, 3.77, 3.35, 3.48, 3.65, 3.71, 3.44, 3.70]
```

## Latest run: 2026-11-10T09:56:08Z

Kahvi git d130077, 3 runs each, on Linux-7.0.0-1019-azure-x86_64-with-glibc2.43.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (2.8x) | 0.022s (15.643x) | 0.025s (18.071x) | 0.001s | 0.007s (5.214x) | 0.003s (1.786x) |
| octane/richards | 11.178s (4.994x) | 0.083s (0.036x) | 0.970s (0.433x) | 2.238s | 11.882s (5.17x) | 1.258s (0.547x) |
| octane/deltablue | 9.704s (4.168x) | 0.100s (0.043x) | 1.007s (0.433x) | 2.328s | 13.169s (5.653x) | 1.379s (0.592x) |
| octane/crypto | 12.717s (3.835x) | 0.178s (0.026x) | 2.547s (0.768x) | 3.316s | 33.841s (5.0x) | 3.668s (0.542x) |
| octane/raytrace | 14.418s (4.069x) | 0.162s (0.045x) | 1.266s (0.357x) | 3.543s | 18.789s (5.233x) | 2.812s (0.783x) |
| octane/earley-boyer | 31.177s (4.245x) | 0.274s (0.05x) | 2.396s (0.326x) | 7.345s | 39.243s (7.168x) | 3.462s (0.632x) |
| octane/regexp | 21.228s (3.286x) | 0.289s (0.051x) | 0.874s (0.135x) | 6.459s | 34.581s (6.137x) | 2.800s (0.497x) |
| octane/splay | 1.474s (2.488x) | 0.131s (0.26x) | 0.220s (0.437x) | 0.503s | 1.608s (3.2x) | 0.391s (0.778x) |
| octane/navier-stokes | 5.836s (4.101x) | 0.096s (0.101x) | 1.132s (1.194x) | 0.948s | 5.375s (5.671x) | 0.892s (0.941x) |
| realworld/acorn | 4.749s | 0.136s | 0.465s | – | – | – |
| realworld/marked | 1.216s | 0.081s | 0.097s | – | – | – |
| realworld/prettier | 10.134s | 0.363s | 1.029s | – | – | – |
| realworld/terser | 8.874s | 0.422s | 0.880s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git d130077 |
| node | v24.22.2 |
| node-jitless | v24.22.2 |
| quickjs | 0.18.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-11-10T09:56:08Z | git d130077 | 3.70x | 9 | [json](results/20261110T095608Z-d13007748a.json) |
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
