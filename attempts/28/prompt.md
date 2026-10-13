# Ticket B-promise-race: Promise.race

Implement this built-in in its file, following the spec steps already in
the stub: keep each step as a comment directly above the code for it.

- Abstract operations are in `kahvi_core::ops` (see docs/API.md); use them
  rather than re-implementing. If a helper you need is missing and other
  built-ins would need it too, write it in your file and say so in your
  final report; do not edit other files.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing. Never return
  a made-up value.
- No new Rust tests: the test262 target is the measure.

## Files you may change

- `crates/kahvi-builtins/src/promise/race.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-core/src/ops/promise.rs`
- `crates/kahvi-builtins/src/promise/prototype_then.rs`

## Spec

Promise.race ( _iterable_ )
  id: sec-promise.race   type: built-in function

This function returns a new promise which is settled in the same way as the first passed promise to settle. It resolves all elements of the passed _iterable_ to promises as it runs this algorithm.

1. Let _ctor_ be the *this* value.
2. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
3. Let _promiseResolve_ be Completion(GetPromiseResolve(_ctor_)).
4. IfAbruptRejectPromise(_promiseResolve_, _promiseCapability_).
5. Let _iteratorRecord_ be Completion(GetIterator(_iterable_, ~sync~)).
6. IfAbruptRejectPromise(_iteratorRecord_, _promiseCapability_).
7. Let _result_ be Completion(PerformPromiseRace(_iteratorRecord_, _ctor_, _promiseCapability_, _promiseResolve_)).
8. If _result_ is an abrupt completion, then
   a. If _iteratorRecord_.[[Done]] is *false*, set _result_ to Completion(IteratorClose(_iteratorRecord_, _result_)).
   b. IfAbruptRejectPromise(_result_, _promiseCapability_).
9. Return ! _result_.

NOTE:
If the _iterable_ argument yields no values or if none of the promises yielded by _iterable_ ever settle, then the pending promise returned by this method will never be settled.
NOTE:
This function expects its *this* value to be a constructor function that supports the parameter conventions of the Promise constructor. It also expects that its *this* value provides a `resolve` method.
[subclause sec-performpromiserace: PerformPromiseRace (
_iteratorRecord_: an Iterator Record,
_ctor_: a constructor,
_resultCapability_: a PromiseCapability Record,
_promiseResolve_: a function object,
): either a normal completion containing an Object or a throw completion]

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

--- referenced: GetPromiseResolve ( _promiseCtor_: a constructor, ): either a normal completion containing a function object or a throw completion
  id: sec-getpromiseresolve

1. Let _promiseResolve_ be ? Get(_promiseCtor_, *"resolve"*).
2. If IsCallable(_promiseResolve_) is *false*, throw a *TypeError* exception.
3. Return _promiseResolve_.

--- referenced: GetIterator ( _obj_: an ECMAScript language value, _kind_: ~sync~ or ~async~, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiterator

1. If _kind_ is ~async~, then
   a. Let _method_ be ? GetMethod(_obj_, %Symbol.asyncIterator%).
   b. If _method_ is *undefined*, then
      i. Let _syncMethod_ be ? GetMethod(_obj_, %Symbol.iterator%).
      ii. If _syncMethod_ is *undefined*, throw a *TypeError* exception.
      iii. Let _syncIteratorRecord_ be ? GetIteratorFromMethod(_obj_, _syncMethod_).
      iv. Return CreateAsyncFromSyncIterator(_syncIteratorRecord_).
2. Else,
   a. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
3. If _method_ is *undefined*, throw a *TypeError* exception.
4. Return ? GetIteratorFromMethod(_obj_, _method_).

--- referenced: PerformPromiseRace ( _iteratorRecord_: an Iterator Record, _ctor_: a constructor, _resultCapability_: a PromiseCapability Record, _promiseResolve_: a function object, ): either a normal completion containing an Object or a throw completion
  id: sec-performpromiserace

1. Repeat,
   a. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
   b. If _next_ is ~done~, then
      i. Return _resultCapability_.[[Promise]].
   c. Let _nextPromise_ be ? Call(_promiseResolve_, _ctor_, « _next_ »).
   d. Perform ? Invoke(_nextPromise_, *"then"*, « _resultCapability_.[[Resolve]], _resultCapability_.[[Reject]] »).

--- referenced: IteratorClose ( _iteratorRecord_: an Iterator Record, _completion_: a Completion Record, ): a Completion Record
  id: sec-iteratorclose

description: It is used to notify an iterator that it should perform any actions it would normally perform when it has reached its completed state.

1. Assert: _iteratorRecord_.[[Iterator]] is an Object.
2. Let _iterator_ be _iteratorRecord_.[[Iterator]].
3. Let _innerResult_ be Completion(GetMethod(_iterator_, *"return"*)).
4. If _innerResult_ is a normal completion, then
   a. Let _return_ be _innerResult_.[[Value]].
   b. If _return_ is *undefined*, return ? _completion_.
   c. Set _innerResult_ to Completion(Call(_return_, _iterator_)).
5. If _completion_ is a throw completion, return ? _completion_.
6. If _innerResult_ is a throw completion, return ? _innerResult_.
7. If _innerResult_.[[Value]] is not an Object, throw a *TypeError* exception.
8. Return ? _completion_.


## Done when

- Every test262 run under built-ins/Promise/race/ passes (check with `./t262 built-ins/Promise/race/`). 10 pass on the main branch right now.

