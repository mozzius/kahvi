# Ticket C-proxy-get: Proxy [[Get]]

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

- `crates/kahvi-core/src/object/proxy/get.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-core/src/object/proxy/mod.rs`
- `crates/kahvi-core/src/object/internal.rs`

## Spec

[[Get]] ( _propertyKey_: a property key, _receiver_: an ECMAScript language value, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-proxy-object-internal-methods-and-internal-slots-get-p-receiver   type: internal method

for: a Proxy exotic object _obj_

1. Perform ? ValidateNonRevokedProxy(_obj_).
2. Let _target_ be _obj_.[[ProxyTarget]].
3. Let _handler_ be _obj_.[[ProxyHandler]].
4. Assert: _handler_ is an Object.
5. Let _trap_ be ? GetMethod(_handler_, *"get"*).
6. If _trap_ is *undefined*, then
   a. Return ? _target_.[[Get]](_propertyKey_, _receiver_).
7. Let _trapResult_ be ? Call(_trap_, _handler_, « _target_, _propertyKey_, _receiver_ »).
8. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
9. If _targetDesc_ is not *undefined* and _targetDesc_.[[Configurable]] is *false*, then
   a. If IsDataDescriptor(_targetDesc_) is *true* and _targetDesc_.[[Writable]] is *false*, then
      i. If SameValue(_trapResult_, _targetDesc_.[[Value]]) is *false*, throw a *TypeError* exception.
   b. If IsAccessorDescriptor(_targetDesc_) is *true* and _targetDesc_.[[Getter]] is *undefined*, then
      i. If _trapResult_ is not *undefined*, throw a *TypeError* exception.
10. Return _trapResult_.

NOTE:
[[Get]] for Proxy objects enforces the following invariants:
-
The value reported for a property must be the same as the value of the corresponding target object property if the target object property is a non-writable, non-configurable own data property.
-
The value reported for a property must be *undefined* if the corresponding target object property is a non-configurable own accessor property that has *undefined* as its [[Getter]] attribute.

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

--- referenced: IsDataDescriptor ( _propertyDesc_: a Property Descriptor, ): a Boolean
  id: sec-isdatadescriptor

1. If _propertyDesc_ has a [[Value]] field, return *true*.
2. If _propertyDesc_ has a [[Writable]] field, return *true*.
3. Return *false*.

--- referenced: SameValue ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, ): a Boolean
  id: sec-samevalue

description: It determines whether or not the two arguments are the same value.

1. If SameType(_x_, _y_) is *false*, return *false*.
2. If _x_ is a Number, then
   a. Return Number::sameValue(_x_, _y_).
3. Return SameValueNonNumber(_x_, _y_).

NOTE:
This algorithm differs from the IsStrictlyEqual Algorithm by treating all *NaN* values as equivalent and by differentiating *+0*_𝔽 from *-0*_𝔽.

--- referenced: IsAccessorDescriptor ( _propertyDesc_: a Property Descriptor, ): a Boolean
  id: sec-isaccessordescriptor

1. If _propertyDesc_ has a [[Getter]] field, return *true*.
2. If _propertyDesc_ has a [[Setter]] field, return *true*.
3. Return *false*.


## Done when

- Every test262 run under built-ins/Proxy/get/ passes (check with `./t262 built-ins/Proxy/get/`). 0 pass on the main branch right now.

