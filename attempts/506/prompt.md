# Ticket B-async-iterator-prototype-symbol-async-dispose: AsyncIteratorPrototype[Symbol.asyncDispose]

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

- `crates/kahvi-builtins/src/async_iterator_prototype/symbol_async_dispose.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

%AsyncIteratorPrototype% [ %Symbol.asyncDispose% ] ( )
  id: sec-%asynciteratorprototype%-%symbol.asyncdispose%   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Let _promiseCapability_ be ! NewPromiseCapability(%Promise%).
3. Let _return_ be Completion(GetMethod(_obj_, *"return"*)).
4. IfAbruptRejectPromise(_return_, _promiseCapability_).
5. If _return_ is *undefined*, then
   a. Perform ! Call(_promiseCapability_.[[Resolve]], *undefined*, « *undefined* »).
6. Else,
   a. Let _result_ be Completion(Call(_return_, _obj_, « »)).
   b. IfAbruptRejectPromise(_result_, _promiseCapability_).
   c. Let _resultWrapper_ be Completion(PromiseResolve(%Promise%, _result_)).
   d. IfAbruptRejectPromise(_resultWrapper_, _promiseCapability_).
   e. Let _unwrap_ be a new Abstract Closure with no parameters that captures nothing and performs the following steps when called:
      i. Return *undefined*.
   f. Let _onFulfilled_ be CreateBuiltinFunction(_unwrap_, 1, *""*, « »).
   g. Perform PerformPromiseThen(_resultWrapper_, _onFulfilled_, *undefined*, _promiseCapability_).
7. Return _promiseCapability_.[[Promise]].

The value of the *"name"* property of this method is *"[Symbol.asyncDispose]"*.

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

--- referenced: GetMethod ( _value_: an ECMAScript language value, _propertyKey_: a property key, ): either a normal completion containing either a function object or *undefined*, or a throw completion
  id: sec-getmethod

description: It is used to get the value of a specific property of an ECMAScript language value when the value of the property is expected to be a function.

1. Let _func_ be ? GetV(_value_, _propertyKey_).
2. If _func_ is either *undefined* or *null*, return *undefined*.
3. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
4. Return _func_.

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

- Every test262 run under built-ins/AsyncIteratorPrototype/Symbol.asyncDispose/ passes (check with `./t262 built-ins/AsyncIteratorPrototype/Symbol.asyncDispose/`). 8 pass on the main branch right now.

