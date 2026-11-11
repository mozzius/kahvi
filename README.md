# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Kahvi over time

Geometric mean of Kahvi's time relative to quickjs, across the workloads both ran. Lower is faster.

```mermaid
xychart-beta
  x-axis ["10-16", "10-18", "10-20", "10-21", "10-22", "10-23", "10-24", "10-25", "10-26", "10-27", "10-28", "10-30", "10-31", "11-01", "11-03", "11-04", "11-05", "11-06", "11-07", "11-09", "11-10", "11-11"]
  y-axis "times quickjs"
  line [3.80, 3.11, 2.93, 3.07, 2.71, 2.89, 2.88, 2.91, 3.03, 2.95, 2.99, 2.95, 2.84, 2.85, 3.77, 3.35, 3.48, 3.65, 3.71, 3.44, 3.70, 3.71]
```

## Latest run: 2026-11-11T09:30:33Z

Kahvi git deec975, 3 runs each, on Linux-7.0.0-1019-azure-x86_64-with-glibc2.43.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (3.0x) | 0.023s (11.35x) | 0.023s (11.3x) | 0.002s | 0.011s (5.3x) | 0.003s (1.55x) |
| octane/richards | 6.129s (4.739x) | 0.067s (0.039x) | 0.564s (0.436x) | 1.293s | 8.484s (4.871x) | 0.977s (0.561x) |
| octane/deltablue | 9.199s (3.931x) | 0.076s (0.043x) | 1.005s (0.429x) | 2.340s | 9.784s (5.544x) | 1.107s (0.628x) |
| octane/crypto | 19.773s (3.466x) | 0.116s (0.035x) | 3.643s (0.639x) | 5.705s | 20.181s (6.138x) | 2.370s (0.721x) |
| octane/raytrace | 14.381s (4.005x) | 0.134s (0.053x) | 1.280s (0.356x) | 3.591s | 14.494s (5.764x) | 1.855s (0.738x) |
| octane/earley-boyer | 19.790s (4.657x) | 0.221s (0.055x) | 1.373s (0.323x) | 4.250s | 28.409s (7.004x) | 2.469s (0.609x) |
| octane/regexp | 12.392s (3.838x) | 0.292s (0.043x) | 0.484s (0.15x) | 3.229s | 43.458s (6.442x) | 3.436s (0.509x) |
| octane/splay | 1.503s (2.346x) | 0.156s (0.271x) | 0.279s (0.486x) | 0.574s | 2.208s (3.849x) | 0.429s (0.747x) |
| octane/navier-stokes | 7.456s (4.125x) | 0.123s (0.074x) | 1.727s (1.034x) | 1.669s | 8.315s (4.981x) | 1.481s (0.887x) |
| realworld/acorn | 4.685s | 0.201s | 0.611s | – | – | – |
| realworld/marked | 1.248s | 0.104s | 0.126s | – | – | – |
| realworld/prettier | 12.208s | 0.516s | 1.517s | – | – | – |
| realworld/terser | 10.797s | 0.587s | 1.226s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git deec975 |
| node | v24.22.2 |
| node-jitless | v24.22.2 |
| quickjs | 0.18.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-11-11T09:30:33Z | git deec975 | 3.71x | 9 | [json](results/20261111T093033Z-deec975782.json) |
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
