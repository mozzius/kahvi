# kahvi — plan

A JavaScript engine in Rust, written mostly by cheap models (Haiku) under a
harness that makes cheating unprofitable and measures progress against the
official conformance suite.

## Thesis

1. **test262 is the only judge.** It is external, pinned (`vendor/test262`)
   and unmodifiable by agents. The pass count is the score and it only goes up.
2. **The spec is pseudocode.** Every builtin is a numbered algorithm. If our
   code mirrors spec naming, a ticket is "translate these 12 steps", not
   "design something". Spec `?` (ReturnIfAbrupt) ≈ Rust `?`.
3. **test262's directory tree is the ticket breakdown.**
   `built-ins/Array/prototype/flatMap/` is one ticket with an objective
   done-condition.

## Pipeline

```
architect (Opus) ─┐
triage (Haiku)  ──┴─▶ ticket pool ─▶ Haiku workers ×N   (own worktree, file-scoped)
                                          │ diff only
                                          ▼
                   mechanical gates: scope · lint · build · test262 delta
                                          │ diff + numbers (no agent narrative)
                                          ▼
                                   blind verifier
       good / incomplete ─▶ merge bot ─▶ main, ratchet baseline
       salvageable ───────▶ back to pool with diff + verifier notes
       garbage ───────────▶ discard; 2 strikes → escalate to Sonnet or split
                                          │
      failing test262 on main ─▶ clustered by error ─▶ new tickets (loop)
```

| Role | Model | When | Job |
|---|---|---|---|
| Architect | Opus | phase starts, every ~50 merges, escalations | spine code, docs, skeleton, writes/splits tickets |
| Triage | Haiku | continuously | clusters failures by error → drafts tickets from a template |
| Worker | Haiku | the bulk | one ticket, one scope, fresh session |
| Verifier | Haiku screens every submission; Sonnet confirms anything that would merge | every submission | blind verdict, binding checklist |
| Auditor | Opus | every ~50 merges | samples merged code for rot, tightens lints |
| Merge bot | script | every verdict | rebase, full suite, ratchet, merge |

## Engine architecture (fixed up front; workers never make design calls)

- **Bytecode VM, stack-based.** Generators/async are suspended frames.
- **Spec-mirroring code.** Abstract ops named after the spec
  (`to_property_key`, `ordinary_define_own_property`, …), every step commented
  with its spec text.
- **Handles, not references.** `ObjectId(u32)` into an arena. No lifetimes or
  `unsafe` in any API a worker touches. Leak-everything heap first; real GC is
  a later architect-tier ticket.
- **UTF-16 strings** from day one.
- **Skeleton-first.** A generator emits every builtin as a stub from
  `spec.html` with steps as comments and correct `name`/`length`/attributes.
  Stubs throw `InternalError: not implemented: <name>`, which the runner
  counts separately. A ticket = fill one stub.
- **Crates:** `ast`, `parser`, `compiler`, `vm`, `builtins`, `regexp`, `host`,
  `cli`; `runtime` later.
- **Parser scoreboard from day one** (test262 parse-only + negative syntax
  tests), since nothing in test262 proper passes until `assert.js` runs.

## Anti-faking

Deterministic gates run first; an LLM looks only at what survives.

- Diff confined to the ticket's `scope`; protected paths untouched
  (`vendor/`, `harness/`, `conformance/`, `.claude/`, `Cargo.toml`/`Cargo.lock`,
  CI).
- Added-line lint: `todo!`/`unimplemented!`/`panic!`, test262/harness strings
  outside the host crate, `TODO|for now|simplified|stub|hack|placeholder`,
  new `#[test]`, `#[allow]`, `unsafe`, `std::process`.
- Build + clippy `-D warnings`; target tests before/after; full suite at merge
  with zero regressions (regressions re-run once).
- All numbers computed by the orchestrator in a clean checkout. The worker's
  own claims are never read.
- Runner hardening: tests arrive at random temp paths; positive tests get a
  random completion nonce appended that must be printed; randomized should-fail
  canaries invalidate the run if they "pass"; the engine runs with an empty
  environment; crashes/timeouts are failures.
- **Blind verifier** sees ticket + spec text + diff + gate report, never the
  worker's transcript. Verdicts: `good` (merge), `incomplete` (honest partial:
  merge + follow-up), `salvageable` (back to pool with notes), `garbage`
  (discard, strike).
- **Verifier calibration:** a red-team set of planted cheats and good diffs
  (`harness/redteam/`). Use the cheapest verifier config that rejects 100% of
  plants.
- **Honest escape hatch:** `BLOCKED: <reason>` is not penalised and files a
  dependency ticket.

## Test policy

- Workers cannot write tests. Conformance is test262's job.
- Architect-owned invariant tests only (heap, interning), plus later a small
  end-to-end JS suite for runtime features test262 doesn't cover. Each must
  state the bug it catches that test262 wouldn't.

## Cost levers

- Haiku 5.5 bills 5× above 100K-token prompts → worker contexts stay under
  100K: fresh session per ticket, `read` hints in tickets, `docs/API.md` cheat
  sheet, small files, turn cap.
- Identical static prompt prefix across workers → cache hits.
- Track $ per newly passing test, per ticket kind; route kinds with high
  Haiku garbage rates straight to Sonnet.
- Compile time is the real bottleneck: persistent slots with warm `target/`.

## Milestones

- **M0 Harness** — workspace, pinned test262 + spec, runner + ratchet, spec
  lookup, orchestrator + gates + verifier + merge bot, prompts, red-team set.
  Exit: a ticket flows end to end; every planted cheat is rejected.
- **M1 Spine** (architect, serial) — values/objects/realm, core abstract ops,
  parser + compiler + VM for what `assert.js`/`sta.js` need, `$262` host,
  skeleton generator, `docs/API.md` + golden builtins. Exit: harness loads,
  first few % pass, ~300 tickets queued.
- **M2 Fan-out** (Haiku swarm) — builtins, parser track, simple language
  features; triage keeps the pool full.
- **M3 Hard tail** (Sonnet/Opus) — generators/async, Proxy/Reflect, RegExp
  engine, modules, Date, number→string, Annex B, GC.
- **M4 Runtime** — event loop, console, timers, module loading, own e2e suite.

## Standing decisions (defaults; revisit any time)

- **From scratch:** we write everything JS-semantic (parser, VM, regex,
  builtins). Small math/data crates are allowed in engine crates only with
  architect sign-off (bigint arithmetic, Unicode tables, shortest float
  formatting). Harness code may use whatever it likes.
- **Engine first**, thin runtime in M4. No Node compat goal.
- **Model access:** the orchestrator shells out to `claude -p`, so it works
  with either an API key or a subscription. API key recommended for the swarm.
