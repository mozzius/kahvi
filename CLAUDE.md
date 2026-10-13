# kahvi

A JavaScript engine in Rust, built mostly by a swarm of cheap agents under a
harness. Plan: docs/PLAN.md. Operating the harness: docs/HARNESS.md.

- **test262 is the only measure of correctness.** `./t262 <path prefix>` runs
  tests under `vendor/test262/test/`, e.g. `./t262 built-ins/Array/prototype/at/`.
- **The spec is the design.** `./spec <clause-id>` prints an algorithm with
  the published step numbers; `./spec --find <text>` searches titles.
- **Engine code mirrors the spec.** Abstract operations keep their spec names
  in snake_case (`to_property_key`, `ordinary_get`). Each algorithm step is a
  comment with the spec's text, directly above the code for it.
- **Never fake progress.** No special-casing tests or inputs, no hardcoded
  results, no swallowed errors, no made-up return values. Anything
  unimplemented throws the engine's not-implemented error
  (docs/ENGINE-CLI.md).
- **No unit tests for engine behaviour.** test262 covers it. The only Rust
  tests are architect-owned checks of internals that test262 cannot reach.
- **Protected, architect-only paths:** `vendor/`, `harness/`, `conformance/`,
  `tickets/`, `docs/`, Cargo manifests, this file. Worker tickets list the
  only files they may change.
