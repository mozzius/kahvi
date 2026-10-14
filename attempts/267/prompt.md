# Ticket B-map-group-by: Map.groupBy

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

- `crates/kahvi-builtins/src/map/group_by.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Map.groupBy ( _items_, _callback_ )
  id: sec-map.groupby   type: built-in function

NOTE:
_callback_ should be a function that accepts two arguments. `groupBy` calls _callback_ once for each element in _items_, in ascending order, and constructs a new Map. Each value returned by _callback_ is used as a key in the Map. For each such key, the result Map has an entry whose key is that key and whose value is an array containing all the elements for which _callback_ returned that key.
_callback_ is called with two arguments: the value of the element and the index of the element.
The return value of `groupBy` is a Map.
This function performs the following steps when called:

1. Let _groups_ be ? GroupBy(_items_, _callback_, ~collection~).
2. Let _map_ be ! Construct(%Map%).
3. For each Record { [[Key]], [[Elements]] } _group_ of _groups_, do
   a. Let _elements_ be CreateArrayFromList(_group_.[[Elements]]).
   b. Let _entry_ be the Record { [[Key]]: _group_.[[Key]], [[Value]]: _elements_ }.
   c. Append _entry_ to _map_.[[MapData]].
4. Return _map_.

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


## Done when

- Every test262 run under built-ins/Map/groupBy/ passes (check with `./t262 built-ins/Map/groupBy/`). 4 pass on the main branch right now.

