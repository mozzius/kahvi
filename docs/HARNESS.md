# Operating the harness

Everything is driven from the repository root. Requirements are Rust (stable),
Python 3.11+, git, and the `claude` CLI logged in with an API key or a
subscription. Node is optional; it's only used to validate the runner.

```
git submodule update --init --depth 1     # vendor/test262 + vendor/ecma262 (pinned)
./t262 built-ins/Array/prototype/at/      # build the engine, run some test262
./spec sec-array.prototype.at             # read a spec clause (--refs: + called ops)
./orch run                                # grind the ticket pool
./orch status                             # what happened, what it cost
```

## test262 runner (`./t262`, `harness/t262`)

`./t262 [PREFIX]... [--show N] [--by-dir] [--json F] [--baseline F]` runs
every test file under the given prefixes of `vendor/test262/test/`. It covers
strict, sloppy, module and raw modes, and runs in parallel with timeouts. The
engine contract is in `docs/ENGINE-CLI.md`.

The runner is built so that faking a pass is hard:

- each run gets a fresh temp directory and a random file name;
- positive tests get a random completion marker that the engine must print;
- the engine runs with an empty environment and a memory cap;
- crashes and timeouts count as failures;
- canary tests with random contents must fail. If any of them passes, the
  run is declared invalid and exits with code 3.

`conformance/skip.txt` lists the features and paths that are out of scope.
`conformance/expected-pass.txt` is the ratchet: every run that passes on the
integration branch. Only the merge bot writes it, and a merge that removes a
line is refused.

To check the runner itself against a mature engine:

```
target/debug/t262 run built-ins/Array/ --mem-limit-mb 0 \
  --engine "node --experimental-vm-modules --no-warnings harness/t262/node-adapter.mjs"
```

## Orchestrator (`./orch`, `harness/orch`)

`./orch run` loops over:

1. **Claim.** Picks the highest-priority open ticket whose `needs` are done.
2. **Work.** Resets a slot worktree (`work/slots/slot-N`, which keeps its warm
   `target/`) to the branch tip and runs `claude -p` on the ticket's tier
   model. The run is limited to file tools plus `cargo`, `./t262`, `./spec`
   and `git diff/status`, with a turn cap and a dollar cap. A PreToolUse hook
   blocks edits outside the ticket's `scope`. The worker's chat output is
   only scanned for a `BLOCKED:` line.
3. **Gates.** Recomputed from the commit:
   - files in scope;
   - no protected path touched;
   - lint on added lines (`harness/orch/policy.py`);
   - vendor checkout untouched;
   - build and clippy, in a separate target directory;
   - `check` commands pass;
   - test262 over the ticket's `target`, using the trusted runner build in
     `work/bin`: no regressions, canaries ok, and at least one new pass.
4. **Verify.** The blind verifier gets the ticket, the spec text, the gate
   report and the diff. It never gets the worker's words. Verdicts:
   `good` / `incomplete` merge; `salvageable` goes back to the pool and the
   next attempt starts from this diff plus the reviewer's notes; `garbage` is
   discarded and counts as a strike.
5. **Merge.** Done serially in `work/merge`:
   - cherry-pick the attempt onto the current tip;
   - build, then run the **full** suite against the baseline (a regression
     gets one re-run to rule out flakes, then the merge is refused);
   - rewrite the baseline and commit;
   - advance the branch (`git merge --ff-only` if the main checkout is on it,
     otherwise `update-ref`);
   - push if `push = true`.

   If the target still has failures after the merge, the ticket goes back to
   the pool with the list of what still fails.

Escalation: after `strikes_per_tier` strikes (gate rejections, garbage
verdicts, empty diffs), the ticket moves up a tier: haiku → sonnet → opus.
After `max_attempts`, or a failure at the top tier, it is marked
`needs-architect`. A `BLOCKED:` reply sets it to `blocked` with the reason.
Both show up in `./orch status` for the architect to split, rewrite or
unblock.

Other commands:

- `./orch status`: tickets by status, spend (workers and verifier), verdicts
  per tier, and dollars per new test262 pass.
- `./orch show N`: an attempt's prompt, gate report, verdict, merge result
  and diff. Everything is kept under `work/attempts/N/`.
- `./orch reset T-0001 [--tier sonnet]`: reopen a ticket.
- `./orch triage [--fresh]`: groups failures on the branch by
  "not implemented: X", by normalised error message and by directory. This is
  the raw material for new tickets.
- `./orch calibrate [--model M] [--repeat N]`: runs the verifier over
  `harness/redteam/cases`. It exits non-zero if any planted cheat or buggy
  diff would have been merged.

Configuration is in `harness/orch.toml`. Per-machine overrides go in
`work/orch.local.toml`, for example `slots = 8` or a different verifier model.
`KAHVI_WORK=/some/dir` runs a separate orchestrator with its own state.

## Roles

- **Architect** (Opus, interactive). Owns everything protected:
  - the engine spine, `docs/API.md`, tickets, skip list, policy, prompts;
  - reviewing `needs-architect` and `blocked` tickets;
  - turning triage output into tickets;
  - commits directly to the branch, outside the pipeline.
- **Workers** (`claude -p`, Haiku by default). One ticket each, scoped files
  only.
- **Verifier** (Sonnet by default). Blind, structured output, calibrated
  against the red-team set.
- **Merge bot.** Plain code, no model.

## Calibration log

| date | verifier | exact | critical misses | good work rejected | $/case |
|---|---|---|---|---|---|
| 2026-10-13 | claude-sonnet-5-5 | 10/12 | 0 | 0 | 0.031 |
| 2026-10-13 | claude-haiku-5-5 (×2) | 20/24 | 0 | 2 | 0.003 |

Haiku's two rejections of good work came from trusting its memory of an older
spec edition over the provided text. After that run, short referenced
operations were added to the spec text (`--refs`), and the verifier prompt
now says the provided spec is authoritative.
