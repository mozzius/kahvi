# Garbage collection

kahvi's heap is a set of arenas indexed by handles: `Object`, `JsString`,
`Symbol`, `BigInt` and `EnvId` are `u32` indices into vectors on the
`Agent`. Until now nothing was ever freed. This document is the design of
the collector that frees them; it is architect-owned.

## Shape

- **Mark-sweep, non-moving.** A handle stays valid for as long as its
  target is alive; collection never relocates. Dead slots go on a free list
  per arena and are reused by later allocations.
- **Precise.** Every handle the engine holds is found by tracing from roots;
  nothing is scanned conservatively.
- **Stop-the-world, at safe points only** (below). There is no incremental
  or concurrent marking yet.
- **Triggered by allocation volume.** Allocations since the last
  collection are counted; a safe point collects once the count passes a
  threshold that grows with the live heap (`max(MIN, live * 1.0)`).
  `KAHVI_GC_STRESS=1` collects at every safe point: the test262 results
  under stress must equal the normal results, which is how rooting bugs
  are found.

## When it is safe to collect

Rust code that holds a handle in a local while JavaScript runs is invisible
to the tracer. The rule that makes this manageable:

> A collection happens only at a **safe point**, and only while no Rust code
> above it on the call stack holds unrooted handles (`agent.gc_holds == 0`).

Safe points:

1. **Between jobs** in the job loop (and before a host runs a script):
   nothing is on the Rust stack.
2. **In the interpreter**, at a backward jump and at function entry. The
   live state of every running function is in its `Frame`; frames register
   themselves on the agent (`FrameRegistry`) for the duration of
   `run_frame`, so outer frames suspended in a call are traced too.

Who holds:

- **Rust calling into JavaScript holds.** `ops::call`, `ops::construct` and
  every accessor or trap invocation made from Rust increment `gc_holds`
  around the call: whatever the caller keeps in locals stays valid, and
  JavaScript run from there (a getter, a `forEach` callback) simply does not
  collect until control is back in code that does not hold.
- **The interpreter's own calls do not hold.** `Op::Call`, `Op::New`,
  `Op::SuperCall` and their spread forms keep the callee, `this` and the
  arguments on the caller's operand stack until the callee returns, so the
  caller's frame roots them; the callee's frame roots its own state.
- **Tail calls do not hold.** A built-in whose last action is a call whose
  result it returns (Function.prototype.call/apply, Reflect.apply/construct,
  bound functions) uses `ops::call_tail`, which does not hold: nothing of
  the built-in's is used after it. (Its `this` and arguments are rooted by
  the caller's operand stack, as above.)
- **Resuming suspended code** (generators, async functions, async
  generators, module evaluation) runs `run_frame` from Rust without holding:
  the code that resumes keeps the generator or context object rooted (it is
  `this` on the caller's stack, or the current job's data), and the
  resumed frame is registered like any other.

Built-ins that call back into JavaScript therefore do not need rooting
discipline: while their callback runs, no collection can free their
locals. The cost is that garbage made inside such a callback waits until
the built-in returns. A built-in can opt out of holding by rooting its
locals explicitly (`agent.root_scope()`), which matters for long-running
callbacks; that is follow-up work, done built-in by built-in.

## Roots

- Registered frames: operand stack, reference stack, iterator stack,
  LexicalEnvironment, VariableEnvironment, PrivateEnvironment, function,
  arguments, completion value, pending yield/await state.
- Execution contexts (function objects), realms (intrinsics, global object,
  global environment, template registry), modules (environments,
  namespaces, cycle roots, async capabilities), the job queue, the job
  being run, the symbol registry, well-known symbols and common names, the
  rejection tracker, explicit roots (`root_scope`).
- Compiled code is reached through the functions, scripts and modules that
  own it: each `FunctionCode` lists the string, symbol and BigInt handles in
  its constants (`FunctionCode::handles`, filled when compiling) and its
  nested functions.

## Tracing

`trait Trace { fn trace(&self, t: &mut Tracer); }` is implemented for every
type that can hold a handle. `NativeData` (the slots of built-in objects in
`ObjectKind::Native`) requires `Trace`; each support type implements it.
The `Tracer` marks with an explicit worklist (no recursion), so deep object
graphs cannot overflow the Rust stack.

Jobs are data, not closures: a `Job` is an enum (`PromiseReaction`,
`PromiseResolveThenable`, ...) whose fields are traced.

## Weak references

- **WeakMap / WeakSet** entries are ephemerons: `Tracer::ephemeron(key,
  value)` marks the value only once the key is marked, iterating to a fixed
  point after the strong marking. Sweeping removes entries whose key died.
- **WeakRef**: `[[WeakRefTarget]]` is a weak edge, cleared when the target
  dies. ClearKeptObjects / AddToKeptObjects keep targets alive until the end
  of the current job (the kept list is a root).
- **FinalizationRegistry**: cells whose target dies are removed and the
  registry is scheduled with HostEnqueueFinalizationRegistryCleanupJob.
- **Strings** are interned: the intern table is weak, and sweeping a string
  removes its entry.

`NativeData` types with weak edges implement `sweep_weak`, called after
marking with a liveness test.

## Sweeping

Each arena keeps mark bits. Sweep frees every unmarked slot: the slot's
data is dropped (freeing property maps and native boxes) and its index
pushed on the arena's free list. Realms and compiled scripts are not
collected yet.
