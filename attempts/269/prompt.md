# Ticket B-object-group-by: Object.groupBy

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

- `crates/kahvi-builtins/src/object/group_by.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.groupBy ( _items_, _callback_ )
  id: sec-object.groupby   type: built-in function

NOTE:
_callback_ should be a function that accepts two arguments. `groupBy` calls _callback_ once for each element in _items_, in ascending order, and constructs a new object. Each value returned by _callback_ is coerced to a property key. For each such property key, the result object has a property whose key is that property key and whose value is an array containing all the elements for which the _callback_ return value coerced to that key.
_callback_ is called with two arguments: the value of the element and the index of the element.
The return value of `groupBy` is an object that does not inherit from %Object.prototype%.
This function performs the following steps when called:

1. Let _groups_ be ? GroupBy(_items_, _callback_, ~property~).
2. Let _obj_ be OrdinaryObjectCreate(*null*).
3. For each Record { [[Key]], [[Elements]] } _group_ of _groups_, do
   a. Let _elements_ be CreateArrayFromList(_group_.[[Elements]]).
   b. Perform ! CreateDataPropertyOrThrow(_obj_, _group_.[[Key]], _elements_).
4. Return _obj_.

--- referenced: GroupBy ( _items_: an ECMAScript language value, _callback_: an ECMAScript language value, _keyCoercion_: ~property~ or ~collection~, ): either a normal completion containing a List of Records with fields [[Key]] (an ECMAScript language value) and [[Elements]] (a List of ECMAScript language values), or a throw completion
  id: sec-groupby

1. Perform ? RequireObjectCoercible(_items_).
2. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
3. Let _groups_ be a new empty List.
4. Let _iteratorRecord_ be ? GetIterator(_items_, ~sync~).
5. Let _k_ be 0.
6. Repeat,
   a. If _k_ ≥ 2^53 - 1, then
      i. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
      ii. Return ? IteratorClose(_iteratorRecord_, _error_).
   b. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
   c. If _next_ is ~done~, then
      i. Return _groups_.
   d. Let _value_ be _next_.
   e. Let _key_ be Completion(Call(_callback_, *undefined*, « _value_, 𝔽(_k_) »)).
   f. IfAbruptCloseIterator(_key_, _iteratorRecord_).
   g. If _keyCoercion_ is ~property~, then
      i. Set _key_ to Completion(ToPropertyKey(_key_)).
      ii. IfAbruptCloseIterator(_key_, _iteratorRecord_).
   h. Else,
      i. Assert: _keyCoercion_ is ~collection~.
      ii. Set _key_ to CanonicalizeKeyedCollectionKey(_key_).
   i. Perform AddValueToKeyedGroup(_groups_, _key_, _value_).
   j. Set _k_ to _k_ + 1.

--- referenced: OrdinaryObjectCreate ( _proto_: an Object or *null*, optional _additionalInternalSlotsList_: a List of names of internal slots, ): an Object
  id: sec-ordinaryobjectcreate

description: It is used to specify the runtime creation of new ordinary objects. _additionalInternalSlotsList_ contains the names of additional internal slots that must be defined as part of the object, beyond [[Prototype]] and [[Extensible]]. If _additionalInternalSlotsList_ is not provided, a new empty List is used.

1. Let _internalSlotsList_ be « [[Prototype]], [[Extensible]] ».
2. If _additionalInternalSlotsList_ is present, set _internalSlotsList_ to the list-concatenation of _internalSlotsList_ and _additionalInternalSlotsList_.
3. Let _obj_ be MakeBasicObject(_internalSlotsList_).
4. Set _obj_.[[Prototype]] to _proto_.
5. Return _obj_.

NOTE:
Although OrdinaryObjectCreate does little more than call MakeBasicObject, its use communicates the intention to create an ordinary object, and not an exotic one. Thus, within this specification, it is not called by any algorithm that subsequently modifies the internal methods of the object in ways that would make the result non-ordinary. Operations that create exotic objects invoke MakeBasicObject directly.

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.


## Done when

- Every test262 run under built-ins/Object/groupBy/ passes (check with `./t262 built-ins/Object/groupBy/`). 4 pass on the main branch right now.

