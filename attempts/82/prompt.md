# Ticket C-proxy-construct: Proxy [[Construct]]

Implement this Proxy internal method in its file, following the spec steps
already in the stub: keep each step as a comment directly above the code.

- `p.validate_non_revoked(agent)?` is ValidateNonRevokedProxy and gives
  (target, handler). Call the target's internal methods directly
  (`target.internal_get(agent, key, receiver)`); a property key becomes a
  value for the trap call with `agent.key_to_value(key)`.
- Property descriptors: `ops::from_property_descriptor`,
  `ops::to_property_descriptor`, `ops::complete_property_descriptor`; the
  invariant checks use IsCompatiblePropertyDescriptor, which is
  `kahvi_core::object::internal::validate_and_apply_property_descriptor`
  called with `None` as the object.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))`. No new Rust tests.

## Files you may change

- `crates/kahvi-core/src/object/proxy/construct.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-core/src/object/proxy/mod.rs`
- `crates/kahvi-core/src/object/internal.rs`

## Spec

[[Construct]] ( _argList_: a List of ECMAScript language values, _newTarget_: a constructor, ): either a normal completion containing an Object or a throw completion
  id: sec-proxy-object-internal-methods-and-internal-slots-construct-argumentslist-newtarget   type: internal method

for: a Proxy exotic object _obj_

1. Perform ? ValidateNonRevokedProxy(_obj_).
2. Let _target_ be _obj_.[[ProxyTarget]].
3. Assert: IsConstructor(_target_) is *true*.
4. Let _handler_ be _obj_.[[ProxyHandler]].
5. Assert: _handler_ is an Object.
6. Let _trap_ be ? GetMethod(_handler_, *"construct"*).
7. If _trap_ is *undefined*, then
   a. Return ? Construct(_target_, _argList_, _newTarget_).
8. Let _argArray_ be CreateArrayFromList(_argList_).
9. Let _newObj_ be ? Call(_trap_, _handler_, « _target_, _argArray_, _newTarget_ »).
10. If _newObj_ is not an Object, throw a *TypeError* exception.
11. Return _newObj_.

NOTE:
A Proxy exotic object only has a [[Construct]] internal method if the initial value of its [[ProxyTarget]] internal slot is an object that has a [[Construct]] internal method.
NOTE:
[[Construct]] for Proxy objects enforces the following invariants:
-
The result of [[Construct]] must be an Object.

--- referenced: ValidateNonRevokedProxy ( _proxy_: a Proxy exotic object, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-validatenonrevokedproxy

description: It throws a *TypeError* exception if _proxy_ has been revoked.

1. If _proxy_.[[ProxyTarget]] is *null*, throw a *TypeError* exception.
2. Assert: _proxy_.[[ProxyHandler]] is not *null*.
3. Return ~unused~.

--- referenced: IsConstructor ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-isconstructor

description: It determines if _arg_ is a function object with a [[Construct]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Construct]] internal method, return *true*.
3. Return *false*.

--- referenced: GetMethod ( _value_: an ECMAScript language value, _propertyKey_: a property key, ): either a normal completion containing either a function object or *undefined*, or a throw completion
  id: sec-getmethod

description: It is used to get the value of a specific property of an ECMAScript language value when the value of the property is expected to be a function.

1. Let _func_ be ? GetV(_value_, _propertyKey_).
2. If _func_ is either *undefined* or *null*, return *undefined*.
3. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
4. Return _func_.

--- referenced: Construct ( _ctor_: a constructor, optional _argList_: a List of ECMAScript language values, optional _newTarget_: a constructor, ): either a normal completion containing an Object or a throw completion
  id: sec-construct

description: It is used to call the [[Construct]] internal method of a function object. _argList_ and _newTarget_ are the values to be passed as the corresponding arguments of the internal method. If _argList_ is not present, a new empty List is used as its value. If _newTarget_ is not present, _ctor_ is used as its value.

1. If _newTarget_ is not present, set _newTarget_ to _ctor_.
2. If _argList_ is not present, set _argList_ to a new empty List.
3. Return ? _ctor_.[[Construct]](_argList_, _newTarget_).

NOTE:
If _newTarget_ is not present, this operation is equivalent to: `new F(...argumentsList)`

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).


## Done when

- Every test262 run under built-ins/Proxy/construct/ passes (check with `./t262 built-ins/Proxy/construct/`). 0 pass on the main branch right now.

