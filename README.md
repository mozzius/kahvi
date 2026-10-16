# Kahvi benchmark results

Written by `.github/workflows/bench.yml` from `bench/run.py` (see `bench/README.md` on main). Times are medians of wall time on a GitHub-hosted runner; ratios compare each engine with quickjs measured in the same run, so lower is faster.

## Latest run: 2026-10-16T07:15:56Z

Kahvi git 36694ea, 3 runs each, on Linux-6.17.0-1022-azure-x86_64-with-glibc2.39.

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.002s (1.667x) | 0.020s (14.286x) | 0.021s (15.071x) | 0.001s | 0.010s (6.929x) | 0.003s (2.286x) |
| octane/richards | 9.405s (4.709x) | 0.092s (0.031x) | 0.805s (0.403x) | 1.997s | 11.908s (4.042x) | 1.292s (0.439x) |
| octane/deltablue | 11.433s (4.3x) | 0.092s (0.034x) | 1.053s (0.396x) | 2.659s | 12.670s (4.753x) | 1.387s (0.52x) |
| octane/crypto | 27.241s (1.98x) | 0.201s (0.015x) | 4.849s (0.353x) | 13.755s | 34.164s (2.484x) | 3.655s (0.266x) |
| octane/raytrace | 21.987s (5.622x) | 0.177s (0.045x) | 1.324s (0.339x) | 3.911s | 18.931s (4.791x) | 2.885s (0.73x) |
| octane/earley-boyer | 34.948s (7.9x) | 0.360s (0.052x) | 1.381s (0.312x) | 4.424s | 43.568s (6.295x) | 3.949s (0.571x) |
| octane/regexp | 21.095s (4.105x) | 0.302s (0.053x) | 0.655s (0.127x) | 5.138s | 37.216s (6.529x) | 3.037s (0.533x) |
| octane/splay | 2.678s (4.759x) | 0.193s (0.287x) | 0.321s (0.477x) | 0.673s | 2.005s (2.98x) | 0.371s (0.551x) |
| octane/navier-stokes | 6.308s (2.851x) | 0.131s (0.05x) | 1.668s (0.637x) | 2.619s | 8.328s (3.18x) | 1.463s (0.558x) |
| realworld/acorn | 6.048s | 0.204s | 0.604s | – | – | – |
| realworld/marked | 1.104s | 0.125s | 0.133s | – | – | – |
| realworld/prettier | 18.114s | 0.589s | 1.528s | – | – | – |
| realworld/terser | 12.827s | 0.709s | 1.253s | – | – | – |

| Engine | Version |
|---|---|
| kahvi | git 36694ea |
| node | v22.23.3 |
| node-jitless | v22.23.3 |
| quickjs | 0.17.0 |
| boa | boa 0.22.0 |
| hermes | LLVM (http://llvm.org/): |

## History

| Date | Kahvi commit | Kahvi vs quickjs (geomean) | Workloads | File |
|---|---|---|---|---|
| 2026-10-16T07:15:56Z | git 36694ea | 3.80x | 9 | [json](results/20261016T071556Z-36694eab43.json) |
