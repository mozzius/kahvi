# Ticket C-proxy-get-prototype-of: Proxy [[GetPrototypeOf]]

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

- `crates/kahvi-core/src/object/proxy/get_prototype_of.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-core/src/object/proxy/mod.rs`
- `crates/kahvi-core/src/object/internal.rs`

## Spec

[[GetPrototypeOf]] ( ): either a normal completion containing either an Object or *null*, or a throw completion
  id: sec-proxy-object-internal-methods-and-internal-slots-getprototypeof   type: internal method

for: a Proxy exotic object _obj_

1. Perform ? ValidateNonRevokedProxy(_obj_).
2. Let _target_ be _obj_.[[ProxyTarget]].
3. Let _handler_ be _obj_.[[ProxyHandler]].
4. Assert: _handler_ is an Object.
5. Let _trap_ be ? GetMethod(_handler_, *"getPrototypeOf"*).
6. If _trap_ is *undefined*, then
   a. Return ? _target_.[[GetPrototypeOf]]().
7. Let _handlerProto_ be ? Call(_trap_, _handler_, « _target_ »).
8. If _handlerProto_ is not an Object and _handlerProto_ is not *null*, throw a *TypeError* exception.
9. Let _extensibleTarget_ be ? IsExtensible(_target_).
10. If _extensibleTarget_ is *true*, return _handlerProto_.
11. Let _targetProto_ be ? _target_.[[GetPrototypeOf]]().
12. If SameValue(_handlerProto_, _targetProto_) is *false*, throw a *TypeError* exception.
13. Return _handlerProto_.

NOTE:
[[GetPrototypeOf]] for Proxy objects enforces the following invariants:
-
The result of [[GetPrototypeOf]] must be either an Object or *null*.
-
If the target object is not extensible, [[GetPrototypeOf]] applied to the Proxy object must return the same value as [[GetPrototypeOf]] applied to the Proxy object's target object.

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

--- referenced: IsExtensible ( _obj_: an Object, ): either a normal completion containing a Boolean or a throw completion
  id: sec-isextensible-o

description: It is used to determine whether additional properties can be added to _obj_.

1. Return ? _obj_.[[IsExtensible]]().

--- referenced: SameValue ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, ): a Boolean
  id: sec-samevalue

description: It determines whether or not the two arguments are the same value.

1. If SameType(_x_, _y_) is *false*, return *false*.
2. If _x_ is a Number, then
   a. Return Number::sameValue(_x_, _y_).
3. Return SameValueNonNumber(_x_, _y_).

NOTE:
This algorithm differs from the IsStrictlyEqual Algorithm by treating all *NaN* values as equivalent and by differentiating *+0*_𝔽 from *-0*_𝔽.


## Done when

- Every test262 run under built-ins/Proxy/getPrototypeOf/ passes (check with `./t262 built-ins/Proxy/getPrototypeOf/`). 0 pass on the main branch right now.

