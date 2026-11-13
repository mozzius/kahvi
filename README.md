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

Benchmarks from the last run on main (2026-11-13, Kahvi 16a110ed67, GitHub-hosted runners):
median wall time per workload, with each engine's ratio to the baseline in brackets; lower is faster.
History and engine versions are on the [bench-results](https://github.com/mozzius/kahvi/tree/bench-results) branch; see [bench/](bench/README.md).

| Workload | kahvi | node | node-jitless | quickjs | boa | hermes |
|---|---|---|---|---|---|---|
| startup/empty | 0.003s (2.727x) | 0.026s (12.619x) | 0.026s (12.524x) | 0.002s | 0.012s (5.905x) | 0.004s (1.952x) |
| octane/richards | 6.093s (4.584x) | 0.059s (0.038x) | 0.565s (0.425x) | 1.329s | 7.179s (4.714x) | 0.793s (0.521x) |
| octane/deltablue | 7.926s (4.805x) | 0.066s (0.047x) | 0.829s (0.502x) | 1.649s | 7.741s (5.479x) | 0.869s (0.615x) |
| octane/crypto | 19.181s (3.52x) | 0.151s (0.028x) | 3.721s (0.683x) | 5.449s | 25.965s (4.825x) | 3.016s (0.561x) |
| octane/raytrace | 11.056s (4.134x) | 0.111s (0.049x) | 0.999s (0.373x) | 2.675s | 11.559s (5.1x) | 1.614s (0.712x) |
| octane/earley-boyer | 27.127s (4.355x) | 0.215s (0.055x) | 2.140s (0.344x) | 6.229s | 28.162s (7.196x) | 2.509s (0.641x) |
| octane/regexp | 12.199s (3.773x) | 0.162s (0.054x) | 0.492s (0.152x) | 3.233s | 22.593s (7.553x) | 1.830s (0.612x) |
| octane/splay | 1.310s (2.588x) | 0.173s (0.263x) | 0.323s (0.489x) | 0.660s | 2.343s (3.55x) | 0.474s (0.718x) |
| octane/navier-stokes | 6.927s (3.976x) | 0.124s (0.073x) | 1.724s (1.016x) | 1.697s | 8.319s (4.902x) | 1.483s (0.874x) |
| realworld/acorn | 3.762s | 0.201s | 0.625s | – | – | – |
| realworld/marked | 1.016s | 0.104s | 0.131s | – | – | – |
| realworld/prettier | 11.934s | 0.532s | 1.548s | – | – | – |
| realworld/terser | 10.372s | 0.611s | 1.280s | – | – | – |

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
