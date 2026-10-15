# Ticket B-atomics-exchange: Atomics.exchange

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

- `crates/kahvi-builtins/src/atomics/exchange.rs`

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

Atomics.exchange ( _ta_, _index_, _value_ )
  id: sec-atomics.exchange   type: built-in function

This function performs the following steps when called:

1. Let _second_ be a new read-modify-write modification function with parameters (_oldBytes_, _newBytes_) that captures nothing and performs the following steps atomically when called:
   a. Return _newBytes_.
2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _second_).

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

- Every test262 run under built-ins/Atomics/exchange/ passes (check with `./t262 built-ins/Atomics/exchange/`). 0 pass on the main branch right now.

