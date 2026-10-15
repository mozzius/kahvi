# kahvi

A JavaScript engine written in Rust, built mostly by a swarm of cheap models
(Haiku) working under a harness that makes cheating unprofitable and measures
progress against the official [test262](https://github.com/tc39/test262)
conformance suite.

Every suite's CI job on main rewrites its own row: test262 is the only judge of
correctness, mjsunit is V8's own tests run under the `kahvi` CLI (`./mjs`), and
realworld runs npm packages and programs under `kahvi-run`, checked against
Node (`./rw`).

<!-- results:start -->

| Suite | Passing | Pass rate | |
|---|--:|--:|---|
| [test262](https://github.com/tc39/test262) | 81,183 / 81,183 | 100.00% | 48,751 files, 0 not implemented |
| [mjsunit](mjsunit/run.py) | 5,187 / 5,534 | 93.73% | V8 [758f9aba526b](https://chromium.googlesource.com/v8/v8/+/758f9aba526bfbd1183e18f420096e5b4cf9df07/test/mjsunit); 4,319 skipped (need d8 or V8 internals) |
| [realworld](realworld/README.md) | 17 / 17 | 100.00% | npm packages and programs, checked byte for byte against Node |

<!-- results:end -->

## How it works

- **test262 is the only judge.** It is pinned in `vendor/test262`, agents
  cannot modify it, and the pass count is a ratchet that only goes up
  (`conformance/expected-pass.txt`).
- **The spec is the design.** Engine code mirrors ECMA-262: abstract operations
  keep their spec names in snake_case (`to_property_key`, `ordinary_get`), and
  every algorithm step is a comment with the spec's text above its code.
- **test262's directory tree is the ticket breakdown.** One directory such as
  `built-ins/Array/prototype/flatMap/` is one ticket with an objective
  done-condition.
- **Never fake progress.** Anything unimplemented throws
  `InternalError: not implemented: <name>`. Special-casing tests, hardcoded
  results and swallowed errors are rejected by mechanical gates and a blind
  verifier.

Workers get a single ticket and a file scope; a script merges what passes the
gates and ratchets the baseline. See [docs/PLAN.md](docs/PLAN.md) for the
pipeline, engine architecture, and milestones.

## Getting started

Requirements: Rust (stable), Python 3.11+, git. The `claude` CLI is needed only
to run the swarm.

```sh
git clone <this repo> && cd kahvi
git submodule update --init --depth 1     # vendor/test262 + vendor/ecma262

./t262 built-ins/Array/prototype/at/      # build the engine, run some test262
./spec sec-array.prototype.at             # print a spec clause with step numbers
./spec --find flatMap                     # search clause titles

./orch run                                # grind the ticket pool
./orch status                             # what happened, what it cost
```

`./t262 [PREFIX]... [--show N] [--by-dir] [--json F] [--baseline F]` runs every
test under the given prefixes of `vendor/test262/test/`, in strict, sloppy,
module and raw modes. The engine binary's contract with the runner is in
[docs/ENGINE-CLI.md](docs/ENGINE-CLI.md).

## Layout

| Path | Contents |
|---|---|
| `crates/` | The engine
| `harness/t262` | test262 runner with anti-faking measures and canary tests |
| `harness/orch`, `orch` | Orchestrator: claims tickets, runs workers, gates, verifier, merge bot |
| `harness/spec.py`, `spec` | ECMA-262 clause lookup |
| `tickets/` | The ticket pool |
| `conformance/` | `expected-pass.txt` (ratchet) and `skip.txt` (out of scope) |
| `vendor/` | Pinned test262 and ECMA-262 sources (submodules) |
| `docs/` | [PLAN](docs/PLAN.md), [HARNESS](docs/HARNESS.md), [ENGINE-CLI](docs/ENGINE-CLI.md) |

## Goals

- From scratch: parser, bytecode VM, regex engine and builtins are all written
  here.
- Embeddable: a library first, with no process-global state, host-owned I/O
  and host objects, so a React Native (JSI) integration stays possible.
- Engine first; a thin runtime (event loop, timers, modules) comes later.

## Contributing

Read [CLAUDE.md](CLAUDE.md) and [docs/HARNESS.md](docs/HARNESS.md) first.
`vendor/`, `harness/`, `conformance/`, `tickets/`, `docs/` and the Cargo
manifests are architect-only; worker tickets list the only files they may
change. Engine behaviour is tested by test262, not by Rust unit tests.
