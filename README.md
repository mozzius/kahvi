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

Benchmarks from the last run on main (2026-10-18, Kahvi e8d4c59b8e, GitHub-hosted runners):
median wall time per workload, with each engine's ratio to the baseline in brackets; lower is faster.
History and engine versions are on the [bench-results](https://github.com/mozzius/kahvi/tree/bench-results) branch; see [bench/](bench/README.md).

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.004s (2.75x) | 0.023s (19.167x) | 0.020s (16.917x) | 0.001s | 0.009s (7.167x) | 0.003s (2.667x) |
| octane/richards | 5.836s (4.079x) | 0.065s (0.032x) | 0.582s (0.407x) | 1.431s | 8.489s (4.24x) | 0.976s (0.487x) |
| octane/deltablue | 8.182s (4.167x) | 0.073s (0.037x) | 0.834s (0.425x) | 1.964s | 9.591s (4.901x) | 1.075s (0.549x) |
| octane/crypto | 23.981s (1.744x) | 0.120s (0.015x) | 4.850s (0.353x) | 13.749s | 18.576s (2.267x) | 2.268s (0.277x) |
| octane/raytrace | 11.317s (4.063x) | 0.136s (0.048x) | 0.938s (0.337x) | 2.785s | 14.504s (5.174x) | 1.859s (0.663x) |
| octane/earley-boyer | 31.671s (3.87x) | 0.343s (0.042x) | 2.428s (0.297x) | 8.185s | 50.930s (6.163x) | 4.549s (0.55x) |
| octane/regexp | 11.256s (3.573x) | 0.173s (0.056x) | 0.458s (0.145x) | 3.150s | 22.818s (7.333x) | 1.851s (0.595x) |
| octane/splay | 1.434s (2.171x) | 0.172s (0.29x) | 0.287s (0.482x) | 0.595s | 1.687s (2.834x) | 0.352s (0.592x) |
| octane/navier-stokes | 7.093s (2.725x) | 0.121s (0.055x) | 1.469s (0.664x) | 2.211s | 6.870s (3.107x) | 1.060s (0.479x) |
| realworld/acorn | 4.793s | 0.181s | 0.567s | – | – | – |
| realworld/marked | 1.238s | 0.111s | 0.121s | – | – | – |
| realworld/prettier | 12.299s | 0.485s | 1.272s | – | – | – |
| realworld/terser | 10.841s | 0.581s | 1.039s | – | – | – |

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
