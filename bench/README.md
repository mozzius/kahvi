# Benchmarks

Kahvi timed against other JavaScript engines on the same machine, in the same
run. Correctness first: a run whose output fails its check counts as a
failure, never as a time.

```
bench/run.py                                   default workloads, every engine found
bench/run.py --engines kahvi,quickjs octane/richards
bench/run.py --all                             also the large Octane workloads
bench/run.py --runs 5 --json r.json --markdown r.md
bench/run.py --list                            workloads and engines found
```

## Workloads

| Workload | What it is | Check |
| --- | --- | --- |
| `startup/empty` | An empty classic script | No output, exit 0 |
| `octane/*` | [Octane](https://github.com/chromium/octane) benchmarks (npm `benchmark-octane`, pinned), run as one classic script in Octane's deterministic mode: fixed iteration counts and its seeded `Math.random` | Octane's own self-checks pass and every benchmark reports |
| `realworld/*` | The longer `realworld/cases` modules (acorn, marked, prettier, terser), with packages from `node_modules` | stdout equals the case's Node-recorded `.out` |

The default Octane set is the classic kernels (Richards, DeltaBlue, Crypto,
RayTrace, EarleyBoyer, RegExp, Splay, NavierStokes). `--all` adds PdfJS,
Mandreel, Gameboy, CodeLoad, Box2D, zlib and TypeScript.

## Engines

| Engine | Why | Runs |
| --- | --- | --- |
| `kahvi` | `target/release/kahvi` (scripts) and `kahvi-run` (modules) | all |
| `quickjs` | The reference small bytecode interpreter; the baseline when present | scripts |
| `boa` | The closest peer: a JS engine in Rust | scripts |
| `hermes` | What React Native runs (M5) | scripts |
| `node-jitless` | V8's interpreter alone; the baseline when QuickJS is absent | all |
| `node` | V8 with its JITs, the bar people ask about | all |

Engines are found on `PATH` and in `~/.jsvu/bin` and `~/.esvu/bin`
(`npx jsvu --engines=quickjs,hermes` installs them); missing ones are
skipped. Engines without a package resolver do not run `realworld/*`.

## Method

- Release build of Kahvi; every engine runs each workload `--runs` times
  (default 3) in an environment of only `PATH`, `HOME` and `TZ=UTC`.
- Octane's fixed iteration counts are sized for JIT engines, so every
  engine runs `--scale` of them (default 0.05: Richards runs 410 times, not
  8200). All engines run the same counts, so ratios stay comparable.
- The table reports the median wall time and its ratio to the baseline engine
  measured in the same run, so machine speed and runner noise mostly cancel.
- Peak RSS is the largest of the timed runs', read with `wait4`.
- `--json` keeps everything, including each engine's version and the machine.

## In CI

`.github/workflows/bench.yml` runs nightly on main (when something has
landed), on demand, and on PRs labelled `bench`. The workloads are split over
parallel jobs (`.github/scripts/bench_shards.py`): each job times either
Kahvi or the other engines, always beside the baseline engine on the same
runner, and `bench/run.py --merge` joins their `--json` files into one result.

`.github/workflows/icount.yml` counts Kahvi's instructions on every PR with
`bench/icount.py`, base against head, on two runners at once.
