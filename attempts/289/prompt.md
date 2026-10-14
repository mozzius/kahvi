# Ticket B-promise-try: Promise.try

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

- `crates/kahvi-builtins/src/promise/try.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-core/src/ops/promise.rs`
- `crates/kahvi-builtins/src/promise/prototype_then.rs`

## Spec

Promise.try ( _callback_, ..._args_ )
  id: sec-promise.try   type: built-in function

This function performs the following steps when called:

1. Let _ctor_ be the *this* value.
2. If _ctor_ is not an Object, throw a *TypeError* exception.
3. Let _status_ be Completion(Call(_callback_, *undefined*, _args_)).
4. If _status_ is an abrupt completion, then
   a. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
   b. Perform ? Call(_promiseCapability_.[[Reject]], *undefined*, « _status_.[[Value]] »).
   c. Return _promiseCapability_.[[Promise]].
5. Else,
   a. Return ? PromiseResolve(_ctor_, ! _status_).

NOTE:
This function expects its *this* value to be a constructor function that supports the parameter conventions of the Promise constructor.

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).

--- referenced: PromiseResolve ( _ctor_: an Object, _resolution_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-promise-resolve

description: It returns a new promise resolved with _resolution_.

1. If IsPromise(_resolution_) is *true*, then
   a. Let _resolutionCtor_ be ? Get(_resolution_, *"constructor"*).
   b. If SameValue(_resolutionCtor_, _ctor_) is *true*, return _resolution_.
2. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
3. Perform ? Call(_promiseCapability_.[[Resolve]], *undefined*, « _resolution_ »).
4. Return _promiseCapability_.[[Promise]].


## Done when

- Every test262 run under built-ins/Promise/try/ passes (check with `./t262 built-ins/Promise/try/`). 8 pass on the main branch right now.

