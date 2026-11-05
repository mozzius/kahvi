# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26", "10-27", "10-28", "10-30", "10-31", "11-01", "11-03", "11-04", "11-05"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93, 3.07, 2.71, 2.89, 2.88, 2.91, 3.03, 2.95, 2.99, 2.95, 2.84, 2.85, 3.77, 3.35, 3.48]
```

## Latest run: 2026-11-05T09:34:11Z

Kahvi git 663f862, 3 runs each, on Linux-7.0.0-1017-azure-x86_64-with-glibc2.43, Linux-7.0.0-1019-azure-x86_64-with-glibc2.43.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (2.444x) | 0.025s (14.167x) | 0.023s (13.0x) | 0.002s | 0.010s (5.722x) | 0.002s (1.333x) |
| octane/richards | 8.492s (4.563x) | 0.126s (0.054x) | 0.972s (0.522x) | 1.861s | 11.057s (4.741x) | 1.271s (0.545x) |
| octane/deltablue | 9.342s (4.003x) | 0.096s (0.041x) | 1.024s (0.439x) | 2.334s | 12.519s (5.356x) | 1.386s (0.593x) |
| octane/crypto | 23.123s (3.385x) | 0.189s (0.028x) | 4.718s (0.691x) | 6.830s | 34.080s (5.001x) | 3.674s (0.539x) |
| octane/raytrace | 15.024s (4.191x) | 0.160s (0.045x) | 1.286s (0.359x) | 3.585s | 18.520s (5.207x) | 2.856s (0.803x) |
| octane/earley-boyer | 31.269s (4.322x) | 0.378s (0.052x) | 2.374s (0.328x) | 7.235s | 51.211s (7.008x) | 4.624s (0.633x) |
| octane/regexp | 15.028s (2.85x) | 0.286s (0.043x) | 0.719s (0.136x) | 5.274s | 43.865s (6.57x) | 3.451s (0.517x) |
| octane/splay | 1.268s (2.335x) | 0.158s (0.264x) | 0.287s (0.48x) | 0.598s | 2.235s (3.737x) | 0.420s (0.703x) |
| octane/navier-stokes | 6.926s (4.09x) | 0.127s (0.076x) | 1.724s (1.032x) | 1.671s | 8.271s (4.95x) | 1.483s (0.887x) |
| realworld/acorn | 3.896s | 0.196s | 0.617s | – | – | – |
| realworld/marked | 1.007s | 0.103s | 0.126s | – | – | – |
| realworld/prettier | 11.886s | 0.499s | 1.553s | – | – | – |
| realworld/terser | 10.455s | 0.636s | 1.230s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git 663f862 |
| node | v24.22.1 |
| node-jitless | v24.22.1 |
| quickjs | 0.18.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
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
