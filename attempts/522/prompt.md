# Ticket B-atomics-or: Atomics.or

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

- `crates/kahvi-builtins/src/atomics/or.rs`

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

Atomics.or ( _ta_, _index_, _value_ )
  id: sec-atomics.or   type: built-in function

This function performs the following steps when called:

1. Let _or_ be a new read-modify-write modification function with parameters (_xBytes_, _yBytes_) that captures nothing and performs the following steps atomically when called:
   a. Return ByteListBitwiseOp(`|`, _xBytes_, _yBytes_).
2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _or_).

--- referenced: ByteListBitwiseOp ( _op_: `&`, `^`, or `|`, _xBytes_: a List of byte values, _yBytes_: a List of byte values, ): a List of byte values
  id: sec-bytelistbitwiseop

description: The operation atomically performs a bitwise operation on all byte values of the arguments and returns a List of byte values.

1. Assert: _xBytes_ and _yBytes_ have the same number of elements.
2. Let _result_ be a new empty List.
3. Let _i_ be 0.
4. For each element _xByte_ of _xBytes_, do
   a. Let _yByte_ be _yBytes_[_i_].
   b. If _op_ is `&`, then
      i. Let _resultByte_ be the result of applying the bitwise AND operation to _xByte_ and _yByte_.
   c. Else if _op_ is `^`, then
      i. Let _resultByte_ be the result of applying the bitwise exclusive OR (XOR) operation to _xByte_ and _yByte_.
   d. Else,
      i. Assert: _op_ is `|`.
      ii. Let _resultByte_ be the result of applying the bitwise inclusive OR operation to _xByte_ and _yByte_.
   e. Set _i_ to _i_ + 1.
   f. Append _resultByte_ to _result_.
5. Return _result_.

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

- Every test262 run under built-ins/Atomics/or/ passes (check with `./t262 built-ins/Atomics/or/`). 8 pass on the main branch right now.

