# Ticket C-proxy-own-property-keys: Proxy [[OwnPropertyKeys]]

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

- `crates/kahvi-core/src/object/proxy/own_property_keys.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-core/src/object/proxy/mod.rs`
- `crates/kahvi-core/src/object/internal.rs`

## Spec

[[OwnPropertyKeys]] ( ): either a normal completion containing a List of property keys or a throw completion
  id: sec-proxy-object-internal-methods-and-internal-slots-ownpropertykeys   type: internal method

for: a Proxy exotic object _obj_

1. Perform ? ValidateNonRevokedProxy(_obj_).
2. Let _target_ be _obj_.[[ProxyTarget]].
3. Let _handler_ be _obj_.[[ProxyHandler]].
4. Assert: _handler_ is an Object.
5. Let _trap_ be ? GetMethod(_handler_, *"ownKeys"*).
6. If _trap_ is *undefined*, then
   a. Return ? _target_.[[OwnPropertyKeys]]().
7. Let _trapResultArray_ be ? Call(_trap_, _handler_, « _target_ »).
8. Let _trapResult_ be ? CreateListFromArrayLike(_trapResultArray_, ~property-key~).
9. If _trapResult_ contains any duplicate entries, throw a *TypeError* exception.
10. Let _extensibleTarget_ be ? IsExtensible(_target_).
11. Let _targetKeys_ be ? _target_.[[OwnPropertyKeys]]().
12. Assert: _targetKeys_ is a List of property keys.
13. Assert: _targetKeys_ contains no duplicate entries.
14. Let _targetConfigurableKeys_ be a new empty List.
15. Let _targetNonconfigurableKeys_ be a new empty List.
16. For each element _key_ of _targetKeys_, do
   a. Let _propertyDesc_ be ? _target_.[[GetOwnProperty]](_key_).
   b. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Configurable]] is *false*, then
      i. Append _key_ to _targetNonconfigurableKeys_.
   c. Else,
      i. Append _key_ to _targetConfigurableKeys_.
17. If _extensibleTarget_ is *true* and _targetNonconfigurableKeys_ is empty, then
   a. Return _trapResult_.
18. Let _uncheckedResultKeys_ be a List whose elements are the elements of _trapResult_.
19. For each element _key_ of _targetNonconfigurableKeys_, do
   a. If _uncheckedResultKeys_ does not contain _key_, throw a *TypeError* exception.
   b. Remove _key_ from _uncheckedResultKeys_.
20. If _extensibleTarget_ is *true*, return _trapResult_.
21. For each element _key_ of _targetConfigurableKeys_, do
   a. If _uncheckedResultKeys_ does not contain _key_, throw a *TypeError* exception.
   b. Remove _key_ from _uncheckedResultKeys_.
22. If _uncheckedResultKeys_ is not empty, throw a *TypeError* exception.
23. Return _trapResult_.

NOTE:
[[OwnPropertyKeys]] for Proxy objects enforces the following invariants:
-
The result of [[OwnPropertyKeys]] is a List.
-
The returned List contains no duplicate entries.
-
Each element of the returned List is a property key.
-
The result List must contain the keys of all non-configurable own properties of the target object.
-
If the target object is not extensible, then the result List must contain all the keys of the own properties of the target object and no other values.

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

--- referenced: CreateListFromArrayLike ( _obj_: an ECMAScript language value, optional _validElementTypes_: ~all~ or ~property-key~, ): either a normal completion containing a List of ECMAScript language values or a throw completion
  id: sec-createlistfromarraylike

description: It is used to create a List value whose elements are provided by the indexed properties of _obj_. _validElementTypes_ indicates the types of values that are allowed as elements.

1. If _validElementTypes_ is not present, set _validElementTypes_ to ~all~.
2. If _obj_ is not an Object, throw a *TypeError* exception.
3. Let _length_ be ? LengthOfArrayLike(_obj_).
4. Let _list_ be a new empty List.
5. Let _index_ be 0.
6. Repeat, while _index_ < _length_,
   a. Let _indexName_ be ! ToString(𝔽(_index_)).
   b. Let _next_ be ? Get(_obj_, _indexName_).
   c. If _validElementTypes_ is ~property-key~ and _next_ is not a property key, throw a *TypeError* exception.
   d. Append _next_ to _list_.
   e. Set _index_ to _index_ + 1.
7. Return _list_.

--- referenced: IsExtensible ( _obj_: an Object, ): either a normal completion containing a Boolean or a throw completion
  id: sec-isextensible-o

description: It is used to determine whether additional properties can be added to _obj_.

1. Return ? _obj_.[[IsExtensible]]().


## Done when

- Every test262 run under built-ins/Proxy/ownKeys/ passes (check with `./t262 built-ins/Proxy/ownKeys/`). 0 pass on the main branch right now.

