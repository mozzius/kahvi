# Ticket C-proxy-get-own-property: Proxy [[GetOwnProperty]]

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

- `crates/kahvi-core/src/object/proxy/get_own_property.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-core/src/object/proxy/mod.rs`
- `crates/kahvi-core/src/object/internal.rs`

## Spec

[[GetOwnProperty]] ( _propertyKey_: a property key, ): either a normal completion containing either a Property Descriptor or *undefined*, or a throw completion
  id: sec-proxy-object-internal-methods-and-internal-slots-getownproperty-p   type: internal method

for: a Proxy exotic object _obj_

1. Perform ? ValidateNonRevokedProxy(_obj_).
2. Let _target_ be _obj_.[[ProxyTarget]].
3. Let _handler_ be _obj_.[[ProxyHandler]].
4. Assert: _handler_ is an Object.
5. Let _trap_ be ? GetMethod(_handler_, *"getOwnPropertyDescriptor"*).
6. If _trap_ is *undefined*, then
   a. Return ? _target_.[[GetOwnProperty]](_propertyKey_).
7. Let _trapResultObj_ be ? Call(_trap_, _handler_, « _target_, _propertyKey_ »).
8. If _trapResultObj_ is not an Object and _trapResultObj_ is not *undefined*, throw a *TypeError* exception.
9. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
10. If _trapResultObj_ is *undefined*, then
   a. If _targetDesc_ is *undefined*, return *undefined*.
   b. If _targetDesc_.[[Configurable]] is *false*, throw a *TypeError* exception.
   c. Let _extensibleTarget_ be ? IsExtensible(_target_).
   d. If _extensibleTarget_ is *false*, throw a *TypeError* exception.
   e. Return *undefined*.
11. Let _extensibleTarget_ be ? IsExtensible(_target_).
12. Let _resultDesc_ be ? ToPropertyDescriptor(_trapResultObj_).
13. Perform CompletePropertyDescriptor(_resultDesc_).
14. Let _valid_ be IsCompatiblePropertyDescriptor(_extensibleTarget_, _resultDesc_, _targetDesc_).
15. If _valid_ is *false*, throw a *TypeError* exception.
16. If _resultDesc_.[[Configurable]] is *false*, then
   a. If _targetDesc_ is *undefined* or _targetDesc_.[[Configurable]] is *true*, then
      i. Throw a *TypeError* exception.
   b. If _resultDesc_ has a [[Writable]] field and _resultDesc_.[[Writable]] is *false*, then
      i. Assert: _targetDesc_ has a [[Writable]] field.
      ii. If _targetDesc_.[[Writable]] is *true*, throw a *TypeError* exception.
17. Return _resultDesc_.

NOTE:
[[GetOwnProperty]] for Proxy objects enforces the following invariants:
-
The result of [[GetOwnProperty]] must be either a Property Descriptor or *undefined*.
-
A property cannot be reported as non-existent, if it exists as a non-configurable own property of the target object.
-
A property cannot be reported as non-existent, if it exists as an own property of a non-extensible target object.
-
A property cannot be reported as existent, if it does not exist as an own property of the target object and the target object is not extensible.
-
A property cannot be reported as non-configurable, unless it exists as a non-configurable own property of the target object.
-
A property cannot be reported as both non-configurable and non-writable, unless it exists as a non-configurable, non-writable own property of the target object.

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

--- referenced: CompletePropertyDescriptor ( _propertyDesc_: a Property Descriptor, ): ~unused~
  id: sec-completepropertydescriptor

1. Let _like_ be the Record { [[Value]]: *undefined*, [[Writable]]: *false*, [[Getter]]: *undefined*, [[Setter]]: *undefined*, [[Enumerable]]: *false*, [[Configurable]]: *false* }.
2. If IsGenericDescriptor(_propertyDesc_) is *true* or IsDataDescriptor(_propertyDesc_) is *true*, then
   a. If _propertyDesc_ does not have a [[Value]] field, set _propertyDesc_.[[Value]] to _like_.[[Value]].
   b. If _propertyDesc_ does not have a [[Writable]] field, set _propertyDesc_.[[Writable]] to _like_.[[Writable]].
3. Else,
   a. If _propertyDesc_ does not have a [[Getter]] field, set _propertyDesc_.[[Getter]] to _like_.[[Getter]].
   b. If _propertyDesc_ does not have a [[Setter]] field, set _propertyDesc_.[[Setter]] to _like_.[[Setter]].
4. If _propertyDesc_ does not have an [[Enumerable]] field, set _propertyDesc_.[[Enumerable]] to _like_.[[Enumerable]].
5. If _propertyDesc_ does not have a [[Configurable]] field, set _propertyDesc_.[[Configurable]] to _like_.[[Configurable]].
6. Return ~unused~.

--- referenced: IsCompatiblePropertyDescriptor ( _extensible_: a Boolean, _propertyDesc_: a Property Descriptor, _current_: a Property Descriptor or *undefined*, ): a Boolean
  id: sec-iscompatiblepropertydescriptor

1. Return ValidateAndApplyPropertyDescriptor(*undefined*, *""*, _extensible_, _propertyDesc_, _current_).


## Done when

- Every test262 run under built-ins/Proxy/getOwnPropertyDescriptor/ passes (check with `./t262 built-ins/Proxy/getOwnPropertyDescriptor/`). 0 pass on the main branch right now.

