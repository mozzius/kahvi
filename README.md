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

Benchmarks from the last run on main (2026-11-11, Kahvi deec975782, GitHub-hosted runners):
median wall time per workload, with each engine's ratio to the baseline in brackets; lower is faster.
History and engine versions are on the [bench-results](https://github.com/mozzius/kahvi/tree/bench-results) branch; see [bench/](bench/README.md).

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
