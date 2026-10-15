# Status and handoff

Where kahvi stands and how to pick the work up in a new session. Update it
when handing off.

## As of 2026-10-14

- **test262:** 71,374 passing runs at the switch to stacked PRs (87.9% of
  those in scope; `conformance/skip.txt` excludes intl402, staging and
  proposals not in the pinned spec; Annex B is in scope). The ratchet has
  never regressed.
- **Landing:** through stacked PRs (docs/STACKS.md). Accepted work is pushed
  as layers `stack/NNNN`, each a PR on the layer below; sharded PR CI runs
  the full suite (and GC stress for the `gc-stress` label) against the
  layer below's results; Samuel merges from the bottom. The stack sat on
  `m1-spine` (PR #3) at the switch; once #3 merges it sits on `main`.
  `m1-spine` and `harness/land.sh` are retired.
- **Branches:** `stack` is local (the stack top workers start from; the
  main checkout stays on it). `arch` is the architect branch (worktree
  `../kahvi-arch`); start it from `stack` and land with `./orch land arch`.
  `orch-runs` holds the run state, including the stack's layers.
- **GC stress:** until 2026-10-14 the runner cleared the engine's
  environment, so `KAHVI_GC_STRESS=1 ./t262` never stressed anything. Fixed
  (the runner passes that variable through); the first real stress runs
  found nothing.
- **Actions minutes:** the repo is private; Samuel makes it public when the
  minutes run out. A layer whose CI cannot start shows as `stuck` in
  `./orch stack`: tell him.

## Resuming on a fresh machine

```
git clone <repo> kahvi && cd kahvi
git submodule update --init --depth 1
# Restore the run state (ticket statuses, attempt history) before `orch run`,
# or every finished ticket starts over as open:
git fetch origin orch-runs:orch-runs
git worktree add work/runs-repo orch-runs
cp work/runs-repo/state.db work/state.db
# The architect worktree:
git fetch origin arch:arch && git worktree add ../kahvi-arch arch
# Per-machine orchestrator settings (work/ is not committed):
cat > work/orch.local.toml <<'TOML'
branch = "stack"      # local: the stack top
push = true
slots = 4             # no local full-suite runs any more

[escalation]
tiers = ["haiku", "sonnet"]   # Opus-tier work comes to the architect instead
max_attempts = 4

[stack]
repo = "mozzius/kahvi"
trunk = "main"
required_checks = ["test262", "fmt, clippy, test"]
TOML
# The stacker rebuilds `stack` from the newest open layer. Attempts that
# were queued but not yet in a layer were local commits: they are lost and
# their tickets start over.
git checkout -B stack origin/main
nohup ./orch run > work/orch.log 2>&1 & echo $! > work/orch.pid
```

## Landing architect work

In `../kahvi-arch`, on `arch` started from `stack`: run the affected
test262 directories (and `KAHVI_GC_STRESS=1` for GC changes) locally, then
`./orch land arch` from the main checkout. The running orchestrator merges
it onto the stack as its own layer; CI runs the full suite on it. See it
with `./orch stack`. Then regenerate tickets on the next `arch`
(`python3 harness/gen/tickets.py --counts counts.json`, counts from the
newest `work/layers/*/results.jsonl`).

## Pacing

The plan has a 5-hour and a 7-day usage window (`./orch usage`). The swarm
pauses itself at the limit; architect sessions should stop well before it
(the 7-day window is the binding one when the swarm runs all day).

## Open architect work

- Done: the A-gc-* tickets (`docs/GC.md`), binding caches (name
  instructions remember the (hops, slot) of declarative bindings; off in
  code with direct eval or `with`), `String.prototype.normalize` (UAX #15
  tables in kahvi-unicode), Annex B block-level functions (B.3.2), catch
  `var` redeclaration (B.3.4) and call assignment targets. Annex B's
  built-ins (escape, substr, the HTML methods, getYear/setYear,
  RegExp.prototype.compile) are swarm tickets.
- Scope still skipped: intl402.
- M4 started: the `kahvi` embedding crate, `kahvi-run` (console, timers,
  event loop, ES modules from files) and `./e2e`; see `docs/RUNTIME.md` for
  what is next there.
- Triage the `partial` tickets whose remaining failures are not in their own
  code (the failures name the missing piece).
- M5 started: host objects and native state in kahvi-core, the C ABI
  (`crates/kahvi-ffi`, header `include/kahvi.h`) and a `jsi::Runtime` over it
  (`react-native/cpp/KahviJSI.cpp`). `harness/jsi/run.sh` builds it against
  RN v0.76.0's JSI and runs `harness/jsi/test.cpp` (also run it with
  `KAHVI_GC_STRESS=1` and under ASan after changes). ArrayBuffers over
  host memory (`createArrayBuffer`) use external data blocks
  (`DataBlock::External`, `kahvi_array_buffer_new_external`). Missing: the
  RN integration package, and running a real Metro bundle.

## Standing decisions from the user

- ESM only; no CommonJS.
- Commit and push as you go.
- Implement the GC before stopping (done; follow-ups above).
- Times for the user in EEST (UTC+3).
