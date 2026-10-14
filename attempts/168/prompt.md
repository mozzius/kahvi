# Ticket B-object-get-own-property-symbols: Object.getOwnPropertySymbols

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

- `crates/kahvi-builtins/src/object/get_own_property_symbols.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.getOwnPropertySymbols ( _obj_ )
  id: sec-object.getownpropertysymbols   type: built-in function

This function performs the following steps when called:

1. Return CreateArrayFromList(? GetOwnPropertyKeys(_obj_, ~symbol~)).

[subclause sec-getownpropertykeys: GetOwnPropertyKeys (
_value_: an ECMAScript language value,
_type_: ~string~ or ~symbol~,
): either a normal completion containing a List of property keys or a throw completion]

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.

--- referenced: GetOwnPropertyKeys ( _value_: an ECMAScript language value, _type_: ~string~ or ~symbol~, ): either a normal completion containing a List of property keys or a throw completion
  id: sec-getownpropertykeys

1. Let _obj_ be ? ToObject(_value_).
2. Let _keys_ be ? _obj_.[[OwnPropertyKeys]]().
3. Let _nameList_ be a new empty List.
4. For each element _nextKey_ of _keys_, do
   a. If _nextKey_ is a Symbol and _type_ is ~symbol~, or if _nextKey_ is a String and _type_ is ~string~, then
      i. Append _nextKey_ to _nameList_.
5. Return _nameList_.


## Done when

- Every test262 run under built-ins/Object/getOwnPropertySymbols/ passes (check with `./t262 built-ins/Object/getOwnPropertySymbols/`). 6 pass on the main branch right now.

