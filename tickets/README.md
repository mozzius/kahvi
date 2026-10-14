# Tickets

One file per ticket, `T-0001.md`, written by the architect (or drafted by
triage and reviewed by the architect). Run state (open / claimed / done /
blocked …) lives in the orchestrator's database, not here.

```
+++
id = "T-0001"
title = "Array.prototype.flatMap"
kind = "builtin"            # builtin | language | parser | infra
tier = "haiku"              # starting tier: haiku | sonnet | opus | architect
scope = ["crates/kahvi-builtins/src/array/flat_map.rs"]   # the only files the worker may change
read = ["docs/API.md", "crates/kahvi-builtins/src/array/flat.rs"]
spec = ["sec-array.prototype.flatmap", "sec-flattenintoarray"]
target = ["built-ins/Array/prototype/flatMap/"]   # test262 prefixes that define "done"
check = []                  # shell commands that must exit 0 ($KAHVI_BIN is the engine)
needs = []                  # ticket ids that must be done first
priority = 0                # higher runs first
+++
Notes for the worker: which helpers exist, traps to avoid, what is out of
scope. Keep it short; the spec text is added automatically.
```

Good tickets:

- **One stub, one scope.** Ideally one function in one file. Parallel tickets
  never share files.
- **Objective target.** A test262 directory, ending in `/` (prefixes match
  literally: `flat` would also match `flatMap`).
- **Dependencies explicit.** If it needs an abstract operation that doesn't
  exist yet, that is another ticket in `needs`.
- **Sized for Haiku.** If two attempts come back garbage, the ticket is
  probably too big or under-specified. Split it rather than only escalating.

Tickets with `tier = "architect"` (named `A-*`) are tracked here but never
dispatched to the swarm: they are core work the architect does by hand.
