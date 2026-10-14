# Ticket B-typed-array-prototype-values: TypedArray.prototype.values

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

- `crates/kahvi-builtins/src/typed_array/prototype_values.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.values ( )
  id: sec-%typedarray%.prototype.values   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Return CreateArrayIterator(_obj_, ~value~).

--- referenced: ValidateTypedArray ( _obj_: an ECMAScript language value, _order_: ~seq-cst~ or ~unordered~, ): either a normal completion containing a TypedArray With Buffer Witness Record or a throw completion
  id: sec-validatetypedarray

1. Perform ? RequireInternalSlot(_obj_, [[TypedArrayName]]).
2. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
3. Return ? ValidateTypedArrayBounds(_obj_, _order_).

--- referenced: CreateArrayIterator ( _array_: an Object, _kind_: ~key+value~, ~key~, or ~value~, ): an Object
  id: sec-createarrayiterator

description: It is used to create iterator objects for Array methods that return such iterators.

1. Let _iterator_ be OrdinaryObjectCreate(%ArrayIteratorPrototype%, « [[IteratedArrayLike]], [[ArrayLikeNextIndex]], [[ArrayLikeIterationKind]] »).
2. Set _iterator_.[[IteratedArrayLike]] to _array_.
3. Set _iterator_.[[ArrayLikeNextIndex]] to 0.
4. Set _iterator_.[[ArrayLikeIterationKind]] to _kind_.
5. Return _iterator_.


## Done when

- Every test262 run under built-ins/TypedArray/prototype/values/ passes (check with `./t262 built-ins/TypedArray/prototype/values/`). 8 pass on the main branch right now.

