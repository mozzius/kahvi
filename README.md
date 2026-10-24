# kahvi

A JavaScript engine written from scratch in Rust: parser, bytecode VM, garbage
collector, regex engine and every built-in. It passes ~~all~~ *almost all* of
[test262](https://github.com/tc39/test262) that is in scope, runs real npm
packages, and embeds in Rust, C and React Native.

Most of the code was written by a swarm of cheap models (mostly Haiku, with
harder tickets escalated to Sonnet) working one ticket at a time under a
harness that makes cheating unprofitable, with an architect model doing the
design and the hard parts.

## Results

Each suite's CI job on main rewrites its own row.

<!-- results:start -->

| Suite | Passing | Pass rate | |
|---|--:|--:|---|
| [test262](https://github.com/tc39/test262) | 84,067 / 84,983 | 98.92% | 52,074 files, 864 not implemented |
| [mjsunit](mjsunit/run.py) | 5,237 / 5,534 | 94.63% | V8 [758f9aba526b](https://chromium.googlesource.com/v8/v8/+/758f9aba526bfbd1183e18f420096e5b4cf9df07/test/mjsunit); 4,319 skipped (need d8 or V8 internals) |
| [realworld](realworld/README.md) | 17 / 17 | 100.00% | npm packages and programs, checked byte for byte against Node |

<!-- results:end -->

- **test262** is the only judge of correctness. Pinned in `vendor/test262`,
  run in strict, sloppy, module and raw modes; `conformance/skip.txt` leaves
  out intl402, staging and proposals beyond the pinned spec. Passing tests
  are a ratchet (`conformance/expected-pass.txt`): CI fails on any
  regression, including under GC stress.
- **mjsunit** is V8's own test suite, run under the `kahvi` CLI. Tests that
  need d8 or V8 internals are skipped with a reason.
- **realworld** runs npm packages (acorn, prettier, terser, zod, lodash and
  more) and a few self-contained programs under `kahvi-run`, unbundled and
  unpatched, and checks their output byte for byte against Node.

<!-- benchmarks:start -->

Benchmarks from the last run on main (2026-10-24, Kahvi 965eed51be, GitHub-hosted runners):
median wall time per workload, with each engine's ratio to the baseline in brackets; lower is faster.
History and engine versions are on the [bench-results](https://github.com/mozzius/kahvi/tree/bench-results) branch; see [bench/](bench/README.md).

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (2.4x) | 0.022s (14.4x) | 0.022s (14.867x) | 0.002s | 0.010s (6.8x) | 0.003s (1.867x) |
| octane/richards | 6.096s (4.06x) | 0.083s (0.028x) | 0.612s (0.408x) | 1.501s | 11.340s (3.847x) | 1.278s (0.433x) |
| octane/deltablue | 9.596s (3.574x) | 0.095s (0.035x) | 1.048s (0.39x) | 2.685s | 12.464s (4.666x) | 1.370s (0.513x) |
| octane/crypto | 23.711s (1.726x) | 0.203s (0.015x) | 4.837s (0.352x) | 13.735s | 33.429s (2.432x) | 3.675s (0.267x) |
| octane/raytrace | 14.489s (3.684x) | 0.171s (0.044x) | 1.326s (0.337x) | 3.933s | 18.976s (4.858x) | 2.833s (0.725x) |
| octane/earley-boyer | 30.475s (3.795x) | 0.349s (0.052x) | 2.412s (0.3x) | 8.031s | 41.432s (6.196x) | 3.587s (0.536x) |
| octane/regexp | 11.735s (3.583x) | 0.203s (0.055x) | 0.483s (0.147x) | 3.275s | 27.057s (7.268x) | 2.247s (0.604x) |
| octane/splay | 1.483s (1.986x) | 0.187s (0.274x) | 0.320s (0.469x) | 0.681s | 2.007s (2.946x) | 0.378s (0.555x) |
| octane/navier-stokes | 3.664s (2.308x) | 0.134s (0.051x) | 1.685s (0.646x) | 2.609s | 8.331s (3.193x) | 1.468s (0.563x) |
| realworld/acorn | 4.682s | 0.207s | 0.607s | – | – | – |
| realworld/marked | 1.260s | 0.121s | 0.132s | – | – | – |
| realworld/prettier | 8.617s | 0.582s | 1.527s | – | – | – |
| realworld/terser | 7.133s | 0.698s | 1.253s | – | – | – |

<!-- benchmarks:end -->

## Using it

```sh
git clone https://github.com/mozzius/kahvi && cd kahvi
git submodule update --init --depth 1     # vendor/test262 and vendor/ecma262
cargo build --release

echo 'console.log([1, 2, 3].map(x => x * 2))' > hello.mjs
target/release/kahvi-run hello.mjs        # an ES module, with node_modules lookup
```

- **`kahvi-run FILE`** runs an ES module with `console`, timers,
  `queueMicrotask`, `structuredClone` and `process.env`, resolving bare
  specifiers through `node_modules` and `package.json` `"exports"`. ESM
  only: no CommonJS. See [docs/RUNTIME.md](docs/RUNTIME.md).
- **`kahvi [--module] FILE`** is the bare engine the test runners drive, with
  only `print` and `$262` as host bindings
  ([docs/ENGINE-CLI.md](docs/ENGINE-CLI.md)).
- **Embedding:** the `kahvi` crate is the Rust API (a `Runtime`, host
  functions, modules, the job queue); `kahvi-ffi` is a C ABI over it, and
  `react-native/cpp` is a JSI `Runtime` on top of that for React Native.
  The engine has no process-global state and never touches files, clocks or
  processes itself; the host supplies them.

## Testing

```sh
./t262 built-ins/Array/prototype/at/      # test262 under a path prefix
./mjs                                     # V8 mjsunit
./rw                                      # real-world packages vs Node
./e2e                                     # kahvi-run end-to-end tests
bench/run.py                              # benchmarks vs other engines
./spec sec-array.prototype.at             # a spec clause with its step numbers
```

Set `KAHVI_GC_STRESS=1` (or pass `--gc-stress` to `./mjs`, `./rw` and `./e2e`) to
collect at every safe point. Engine behaviour is tested by these suites only; the few
Rust tests check internals that test262 cannot reach.

## How it is built

- **The spec is the design.** Abstract operations keep their ECMA-262 names
  in snake_case (`to_property_key`, `ordinary_get`), and each algorithm step
  is a comment with the spec's text above the code for it.
- **test262's tree is the ticket breakdown.** A directory such as
  `built-ins/Array/prototype/flatMap/` is one ticket with an objective
  done-condition. Workers get one ticket and the only files they may change.
- **Never fake progress.** Anything unimplemented throws
  `InternalError: not implemented: <name>`. Special-cased tests, hardcoded
  results and swallowed errors are caught by mechanical gates and a blind
  verifier.
- **Landing.** Accepted work goes onto a stack of pull requests; sharded CI
  runs the full test262 suite on each layer against the one below it, and
  layers merge into `main` from the bottom
  ([docs/STACKS.md](docs/STACKS.md)).

```sh
./orch run       # work through the ticket pool (needs the claude CLI)
./orch status    # tickets, outcomes and cost
./orch stack     # the stack of PRs and their CI
```

[docs/PLAN.md](docs/PLAN.md) has the pipeline, engine architecture and
milestones, [docs/HARNESS.md](docs/HARNESS.md) how to operate the harness,
and [docs/STATUS.md](docs/STATUS.md) where things stand.

## Layout

| Path | Contents |
|---|---|
| `crates/kahvi-core` | Values, objects, realms, the bytecode compiler and VM, the GC |
| `crates/kahvi-parser`, `kahvi-regexp`, `kahvi-unicode` | Parser, regex engine, Unicode tables |
| `crates/kahvi-builtins` | One file per built-in function ([docs/API.md](docs/API.md)) |
| `crates/kahvi` | The embedding API |
| `crates/kahvi-cli`, `kahvi-runtime` | The `kahvi` and `kahvi-run` binaries |
| `crates/kahvi-ffi`, `react-native/` | C ABI and the React Native JSI shim |
| `harness/` | test262 runner, orchestrator, gates, verifier, spec lookup, e2e tests |
| `mjsunit/`, `realworld/`, `bench/` | The other suites and the benchmarks |
| `tickets/`, `conformance/` | The ticket pool; the test262 ratchet and skip list |
| `vendor/` | Pinned test262 and ECMA-262 (submodules) |

## Contributing

Read [CLAUDE.md](CLAUDE.md) and [docs/HARNESS.md](docs/HARNESS.md) first.
`vendor/`, `harness/`, `conformance/`, `tickets/`, `docs/`, the Cargo
manifests and `CLAUDE.md` are architect-only.
