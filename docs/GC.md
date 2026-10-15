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
  are found. `KAHVI_GC_STRESS=N` collects at one safe point in N on
  average (gaps drawn from 1..2N-1 with a fixed seed, so runs repeat and
  loops are not always interrupted at the same instruction). Each
  collection walks the whole live heap, so every-safe-point stress is
  quadratic on programs with large heaps; the real-world suite runs with
  N = 5000.

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
  bound functions) uses `ops::call_rooted` / `ops::construct_rooted`, which do
  not hold: nothing of the built-in's is used after it. (Its `this` and
  arguments are rooted by the caller's operand stack, as above.) Jobs call
  their handlers the same way; the running job's data is a root.
- **Resuming suspended code** (generators, async functions, async
  generators, module evaluation) runs `run_frame` from Rust without holding:
  the code that resumes keeps the generator or context object rooted (it is
  `this` on the caller's stack, or the current job's data), and the
  resumed frame is registered like any other.

Built-ins that call back into JavaScript therefore do not need rooting
discipline: while their callback runs, no collection can free their
locals. The cost is that garbage made inside such a callback waits until
the built-in returns. A built-in opts out of holding by rooting everything
it uses after a callback and calling with `ops::call_rooted`:

- `agent.with_roots(&[...], |agent| ...)` roots values for the duration of
  the closure, and drops anything pushed inside it too (an early `?`
  return cannot leak roots);
- `agent.push_root(v)` / `agent.push_root_key(k)` / `agent.pop_roots(mark)`
  for per-iteration values (the element passed to the callback, a String
  property key used after it).

Root the callback's arguments as well: a native callee does not root its
own arguments. The callback-heavy built-ins do this (Array forEach, map,
filter, some, every, reduce, reduceRight, sort, Array.from, Set forEach,
String replace and replaceAll, JSON.parse with a reviver, JSON.stringify),
and the interpreter's `for-of` / destructuring steps call `next()` with
`ops::iterator_step_value_rooted` (the record is on the frame's iterator
stack), so a generator driven by `for-of` collects as it runs. Other
built-ins still hold. Code that resumes a suspended
frame from Rust (generators, async functions) roots the object that owns
the frame for the duration.

## Roots

- Registered frames: operand stack, reference stack, iterator stack,
  LexicalEnvironment, VariableEnvironment, PrivateEnvironment, function,
  arguments, completion value, pending yield/await state.
- Execution contexts (function objects), realms (intrinsics, global object,
  global environment, template registry), modules (environments,
  namespaces, cycle roots, async capabilities), the job queue, the job
  being run, the symbol registry, well-known symbols and common names, the
  rejection tracker, explicit roots (`push_root`), the [[KeptAlive]] list.
- Compiled code: each compilation unit (a script, module, eval or dynamic
  function, with every function nested in it) records the strings and
  BigInts the compiler created or looked up while compiling it
  (`CodeConstants`, shared by all of the unit's `FunctionCode`s). Function
  objects, frames and modules trace their code's constants
  (`Tracer::code`, once per unit per collection). The code itself is
  reference-counted (`Arc`) and freed with its last holder. Only the
  agent's common names and the well-known symbols are pinned.
- Script records of eval and dynamic functions are freed once no live code
  belongs to them (their template objects go too), and the index is reused
  by a new record with the same realm and name.

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
- **FinalizationRegistry**: cell targets and unregister tokens are weak,
  held values strong. After a collection, every live registry with a cell
  whose target died gets a cleanup job (`Job::FinalizationRegistryCleanup`,
  HostEnqueueFinalizationRegistryCleanupJob). Dead unregister tokens are
  cleared so a reused handle cannot match them.
- **Strings** are interned: the intern table is weak, and sweeping a string
  removes its entry.

`NativeData` types with weak edges implement `sweep_weak`, called after
marking with a liveness test.

## Sweeping

Each arena keeps mark bits. Sweep frees every unmarked slot: the slot's
data is dropped (freeing property maps and native boxes) and its index
pushed on the arena's free list. Realms are not collected yet.
