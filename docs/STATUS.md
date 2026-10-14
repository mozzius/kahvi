# Status and handoff

Where kahvi stands and how to pick the work up in a new session. Update it
when handing off.

## As of 2026-10-14

- **test262:** 69,294 passing runs (86.7% of those in scope;
  `conformance/skip.txt` excludes intl402, staging and proposals not in
  the pinned spec; Annex B is in scope). The ratchet (`conformance/expected-pass.txt`)
  has never regressed.
- **Branches:** `m1-spine` is the integration branch (the swarm's merge bot
  lands on it; architect work lands with `harness/land.sh`). `arch` is the
  architect branch (worktree `../kahvi-arch`). `orch-runs` holds the run
  state. `main` gets `m1-spine` by PR (#3 is the first); keep it current
  after each landing.
- **Swarm:** ~330 open built-in tickets (Haiku), ~60 partial. Stopped at
  handoff; restart it in the new session (below).

## Resuming on a fresh machine

```
git clone <repo> kahvi && cd kahvi && git checkout m1-spine
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
branch = "m1-spine"   # the integration branch (main gets it by PR)
push = true           # containers are ephemeral: keep every merge on origin
# slots = 4           # more concurrent workers on a bigger box

[escalation]
tiers = ["haiku", "sonnet"]   # Opus-tier work comes to the architect instead
max_attempts = 4
TOML
nohup ./orch run > work/orch.log 2>&1 & echo $! > work/orch.pid
```

At handoff one ticket (`B-array-prototype-to-spliced`) was killed mid-merge;
`orch run` reopens it on start.

## Landing architect work

From `../kahvi-arch`, after a clean full run against
`conformance/expected-pass.txt`:

```
KAHVI_ROOT=<main checkout> setsid nohup bash harness/land.sh arch > land.log 2>&1 < /dev/null &
```

It stops the swarm (draining in-flight attempts), merges, runs the full suite
on the release build, refuses on any regression, pushes and restarts the
swarm. It takes 20-40 minutes: run it detached and watch `land.log` for the
`land:` line (or "regressions; undoing"). Then merge `m1-spine` back into
`arch`, and regenerate tickets there (`python3 harness/gen/tickets.py
--counts counts.json`, counts from the latest `work/attempts/*/full.jsonl`)
so stubs and priorities reflect the swarm's merges.

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
  `KAHVI_GC_STRESS=1` and under ASan after changes). Missing: ArrayBuffers
  over host memory (`createArrayBuffer` throws not-implemented), the RN
  integration package, and running a real Metro bundle.

## Standing decisions from the user

- ESM only; no CommonJS.
- Commit and push as you go.
- Implement the GC before stopping (done; follow-ups above).
- Times for the user in EEST (UTC+3).
