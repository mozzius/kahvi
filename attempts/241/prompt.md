# Ticket C-proxy-call: Proxy [[Call]]

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

- `crates/kahvi-core/src/object/proxy/call.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-core/src/object/proxy/mod.rs`
- `crates/kahvi-core/src/object/internal.rs`

## Spec

[[Call]] ( _thisArg_: an ECMAScript language value, _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-proxy-object-internal-methods-and-internal-slots-call-thisargument-argumentslist   type: internal method

for: a Proxy exotic object _obj_

1. Perform ? ValidateNonRevokedProxy(_obj_).
2. Let _target_ be _obj_.[[ProxyTarget]].
3. Let _handler_ be _obj_.[[ProxyHandler]].
4. Assert: _handler_ is an Object.
5. Let _trap_ be ? GetMethod(_handler_, *"apply"*).
6. If _trap_ is *undefined*, then
   a. Return ? Call(_target_, _thisArg_, _argList_).
7. Let _argArray_ be CreateArrayFromList(_argList_).
8. Return ? Call(_trap_, _handler_, « _target_, _thisArg_, _argArray_ »).

NOTE:
A Proxy exotic object only has a [[Call]] internal method if the initial value of its [[ProxyTarget]] internal slot is an object that has a [[Call]] internal method.

--- referenced: ValidateNonRevokedProxy ( _proxy_: a Proxy exotic object, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-validatenonrevokedproxy

description: It throws a *TypeError* exception if _proxy_ has been revoked.

1. If _proxy_.[[ProxyTarget]] is *null*, throw a *TypeError* exception.
2. Assert: _proxy_.[[ProxyHandler]] is not *null*.
3. Return ~unused~.

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

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.


## Done when

- Every test262 run under built-ins/Proxy/apply/ passes (check with `./t262 built-ins/Proxy/apply/`). 0 pass on the main branch right now.

