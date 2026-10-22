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

Benchmarks from the last run on main (2026-10-22, Kahvi f9066d9e0c, GitHub-hosted runners):
median wall time per workload, with each engine's ratio to the baseline in brackets; lower is faster.
History and engine versions are on the [bench-results](https://github.com/mozzius/kahvi/tree/bench-results) branch; see [bench/](bench/README.md).

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.003s (2.077x) | 0.019s (11.875x) | 0.018s (11.0x) | 0.002s | 0.007s (4.438x) | 0.002s (1.5x) |
| octane/richards | 10.248s (3.492x) | 0.069s (0.035x) | 1.125s (0.383x) | 2.935s | 8.503s (4.292x) | 0.972s (0.49x) |
| octane/deltablue | 9.653s (3.592x) | 0.074s (0.037x) | 1.047s (0.39x) | 2.688s | 9.779s (4.933x) | 1.091s (0.55x) |
| octane/crypto | 11.859s (1.461x) | 0.194s (0.014x) | 2.360s (0.291x) | 8.116s | 35.101s (2.545x) | 3.673s (0.266x) |
| octane/raytrace | 13.777s (3.442x) | 0.134s (0.048x) | 1.317s (0.329x) | 4.003s | 14.546s (5.217x) | 1.850s (0.664x) |
| octane/earley-boyer | 32.090s (3.888x) | 0.347s (0.042x) | 2.450s (0.297x) | 8.254s | 51.770s (6.229x) | 4.640s (0.558x) |
| octane/regexp | 20.232s (2.973x) | 0.346s (0.051x) | 0.887s (0.13x) | 6.805s | 43.754s (6.435x) | 3.406s (0.501x) |
| octane/splay | 1.079s (1.972x) | 0.148s (0.286x) | 0.253s (0.49x) | 0.517s | 1.582s (3.06x) | 0.364s (0.705x) |
| octane/navier-stokes | 6.959s (2.663x) | 0.108s (0.056x) | 1.286s (0.672x) | 1.915s | 6.005s (3.136x) | 0.910s (0.475x) |
| realworld/acorn | 3.044s | 0.159s | 0.493s | – | – | – |
| realworld/marked | 0.858s | 0.097s | 0.103s | – | – | – |
| realworld/prettier | 12.167s | 0.440s | 1.103s | – | – | – |
| realworld/terser | 10.716s | 0.513s | 0.900s | – | – | – |

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
