# Ticket B-disposable-stack-prototype-adopt: DisposableStack.prototype.adopt

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

- `crates/kahvi-builtins/src/disposable_stack/prototype_adopt.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/disposable_stack.rs`
- `crates/kahvi-core/src/ops/dispose.rs`
- `crates/kahvi-builtins/src/disposable_stack/prototype_use.rs`

## Spec

DisposableStack.prototype.adopt ( _value_, _onDispose_ )
  id: sec-disposablestack.prototype.adopt   type: built-in function

This method performs the following steps when called:

1. Let _disposableStack_ be the *this* value.
2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
3. If _disposableStack_.[[DisposableState]] is ~disposed~, throw a *ReferenceError* exception.
4. If IsCallable(_onDispose_) is *false*, throw a *TypeError* exception.
5. Let _closure_ be a new Abstract Closure with no parameters that captures _value_ and _onDispose_ and performs the following steps when called:
   a. Return ? Call(_onDispose_, *undefined*, « _value_ »).
6. Let _func_ be CreateBuiltinFunction(_closure_, 0, *""*, « »).
7. Perform ? AddDisposableResource(_disposableStack_.[[DisposableResourceStack]], *undefined*, ~sync-dispose~, _func_).
8. Return _value_.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

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

--- referenced: AddDisposableResource ( _disposableResourceStack_: a List of DisposableResource Records, _value_: an ECMAScript language value, _kind_: ~sync-dispose~ or ~async-dispose~, optional _method_: a function object, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-adddisposableresource

1. If _method_ is present, then
   a. Assert: _value_ is *undefined*.
   b. Let _resource_ be ? CreateDisposableResource(*undefined*, _kind_, _method_).
2. Else,
   a. If _value_ is either *null* or *undefined* and _kind_ is ~sync-dispose~, return ~unused~.
   b. NOTE: When _value_ is either *null* or *undefined* and _kind_ is ~async-dispose~, we record that the resource was evaluated to ensure we will still perform an Await when resources are later disposed.
   c. Let _resource_ be ? CreateDisposableResource(_value_, _kind_).
3. Append _resource_ to _disposableResourceStack_.
4. Return ~unused~.


## Done when

- Every test262 run under built-ins/DisposableStack/prototype/adopt/ passes (check with `./t262 built-ins/DisposableStack/prototype/adopt/`). 8 pass on the main branch right now.

