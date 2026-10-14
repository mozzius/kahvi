# Ticket B-promise-prototype-finally: Promise.prototype.finally

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

- `crates/kahvi-builtins/src/promise/prototype_finally.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-core/src/ops/promise.rs`
- `crates/kahvi-builtins/src/promise/prototype_then.rs`

## Spec

Promise.prototype.finally ( _onFinally_ )
  id: sec-promise.prototype.finally   type: built-in function

This method performs the following steps when called:

1. Let _promise_ be the *this* value.
2. If _promise_ is not an Object, throw a *TypeError* exception.
3. Let _ctor_ be ? SpeciesConstructor(_promise_, %Promise%).
4. Assert: IsConstructor(_ctor_) is *true*.
5. If IsCallable(_onFinally_) is *false*, then
   a. Let _thenFinally_ be _onFinally_.
   b. Let _catchFinally_ be _onFinally_.
6. Else,
   a. Let _thenFinallyClosure_ be a new Abstract Closure with parameters (_value_) that captures _onFinally_ and _ctor_ and performs the following steps when called:
      i. Let _result_ be ? Call(_onFinally_, *undefined*).
      ii. Let _p_ be ? PromiseResolve(_ctor_, _result_).
      iii. Let _returnValue_ be a new Abstract Closure with no parameters that captures _value_ and performs the following steps when called:
         1. Return NormalCompletion(_value_).
      iv. Let _valueThunk_ be CreateBuiltinFunction(_returnValue_, 0, *""*, « »).
      v. Return ? Invoke(_p_, *"then"*, « _valueThunk_ »).
   b. Let _thenFinally_ be CreateBuiltinFunction(_thenFinallyClosure_, 1, *""*, « »).
   c. Let _catchFinallyClosure_ be a new Abstract Closure with parameters (_reason_) that captures _onFinally_ and _ctor_ and performs the following steps when called:
      i. Let _result_ be ? Call(_onFinally_, *undefined*).
      ii. Let _p_ be ? PromiseResolve(_ctor_, _result_).
      iii. Let _throwReason_ be a new Abstract Closure with no parameters that captures _reason_ and performs the following steps when called:
         1. Throw _reason_.
      iv. Let _thrower_ be CreateBuiltinFunction(_throwReason_, 0, *""*, « »).
      v. Return ? Invoke(_p_, *"then"*, « _thrower_ »).
   d. Let _catchFinally_ be CreateBuiltinFunction(_catchFinallyClosure_, 1, *""*, « »).
7. Return ? Invoke(_promise_, *"then"*, « _thenFinally_, _catchFinally_ »).

--- referenced: SpeciesConstructor ( _obj_: an Object, _defaultCtor_: a constructor, ): either a normal completion containing a constructor or a throw completion
  id: sec-speciesconstructor

description: It is used to retrieve the constructor that should be used to create new objects that are derived from _obj_. _defaultCtor_ is the constructor to use if a constructor %Symbol.species% property cannot be found starting from _obj_.

1. Let _ctor_ be ? Get(_obj_, *"constructor"*).
2. If _ctor_ is *undefined*, return _defaultCtor_.
3. If _ctor_ is not an Object, throw a *TypeError* exception.
4. Let _species_ be ? Get(_ctor_, %Symbol.species%).
5. If _species_ is either *undefined* or *null*, return _defaultCtor_.
6. If IsConstructor(_species_) is *true*, return _species_.
7. Throw a *TypeError* exception.

--- referenced: IsConstructor ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-isconstructor

description: It determines if _arg_ is a function object with a [[Construct]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Construct]] internal method, return *true*.
3. Return *false*.

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

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

--- referenced: NormalCompletion ( _value_: any value except a Completion Record, ): a normal completion
  id: sec-normalcompletion

skip return checks: true

1. Return Completion Record { [[Type]]: ~normal~, [[Value]]: _value_, [[Target]]: ~empty~ }.

--- referenced: Invoke ( _value_: an ECMAScript language value, _propertyKey_: a property key, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-invoke

description: It is used to call a method property of an ECMAScript language value. _value_ serves as both the lookup point for the property and the *this* value of the call. _argList_ is the list of arguments values passed to the method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. Let _func_ be ? GetV(_value_, _propertyKey_).
3. Return ? Call(_func_, _value_, _argList_).


## Done when

- Every test262 run under built-ins/Promise/prototype/finally/ passes (check with `./t262 built-ins/Promise/prototype/finally/`). 12 pass on the main branch right now.

