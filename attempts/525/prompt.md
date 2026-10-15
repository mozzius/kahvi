# Ticket B-atomics-sub: Atomics.sub

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

- `crates/kahvi-builtins/src/atomics/sub.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/atomics.rs`
- `crates/kahvi-builtins/src/atomics/add.rs`
- `crates/kahvi-builtins/src/atomics/and.rs`
- `crates/kahvi-core/src/object/typed_array.rs`

## Spec

Atomics.sub ( _ta_, _index_, _value_ )
  id: sec-atomics.sub   type: built-in function

This function performs the following steps when called:

1. Let _subtract_ be a new read-modify-write modification function with parameters (_xBytes_, _yBytes_) that captures _ta_ and performs the following steps atomically when called:
   a. Let _type_ be TypedArrayElementType(_ta_).
   b. Let _agentRecord_ be the Agent Record of the surrounding agent.
   c. Let _isLittleEndian_ be _agentRecord_.[[LittleEndian]].
   d. Let _x_ be RawBytesToNumeric(_type_, _xBytes_, _isLittleEndian_).
   e. Let _y_ be RawBytesToNumeric(_type_, _yBytes_, _isLittleEndian_).
   f. If _x_ is a Number, then
      i. Let _difference_ be Number::subtract(_x_, _y_).
   g. Else,
      i. Assert: _x_ is a BigInt.
      ii. Let _difference_ be BigInt::subtract(_x_, _y_).
   h. Let _differenceBytes_ be NumericToRawBytes(_type_, _difference_, _isLittleEndian_).
   i. Assert: _differenceBytes_, _xBytes_, and _yBytes_ have the same number of elements.
   j. Return _differenceBytes_.
2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _subtract_).

--- referenced: TypedArrayElementType ( _obj_: a TypedArray, ): a TypedArray element type
  id: sec-typedarrayelementtype

1. Return the Element Type value specified in [table-the-typedarray-constructors] for _obj_.[[TypedArrayName]].

--- referenced: AtomicReadModifyWrite ( _ta_: an ECMAScript language value, _index_: an ECMAScript language value, _value_: an ECMAScript language value, _op_: a read-modify-write modification function, ): either a normal completion containing either a Number or a BigInt, or a throw completion
  id: sec-atomicreadmodifywrite

description: _op_ takes two List of byte values arguments and returns a List of byte values. This operation atomically loads a value, combines it with another value, and stores the combination. It returns the loaded value.

1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
2. If _ta_.[[ContentType]] is ~bigint~, let _coerced_ be ? ToBigInt(_value_).
3. Else, let _coerced_ be 𝔽(? ToIntegerOrInfinity(_value_)).
4. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
5. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
6. Let _elementType_ be TypedArrayElementType(_ta_).
7. Return GetModifySetValueInBuffer(_buffer_, _byteIndexInBuffer_, _elementType_, _coerced_, _op_).


## Done when

- Every test262 run under built-ins/Atomics/sub/ passes (check with `./t262 built-ins/Atomics/sub/`). 8 pass on the main branch right now.

