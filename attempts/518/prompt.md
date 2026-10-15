# Ticket B-atomics-compare-exchange: Atomics.compareExchange

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

- `crates/kahvi-builtins/src/atomics/compare_exchange.rs`

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

Atomics.compareExchange ( _ta_, _index_, _expectedValue_, _replacementValue_ )
  id: sec-atomics.compareexchange   type: built-in function

This function performs the following steps when called:

1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
2. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
3. Let _block_ be _buffer_.[[ArrayBufferData]].
4. If _ta_.[[ContentType]] is ~bigint~, then
   a. Let _expected_ be ? ToBigInt(_expectedValue_).
   b. Let _replacement_ be ? ToBigInt(_replacementValue_).
5. Else,
   a. Let _expected_ be 𝔽(? ToIntegerOrInfinity(_expectedValue_)).
   b. Let _replacement_ be 𝔽(? ToIntegerOrInfinity(_replacementValue_)).
6. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
7. Let _elementType_ be TypedArrayElementType(_ta_).
8. Let _elementSize_ be TypedArrayElementSize(_ta_).
9. Let _agentRecord_ be the Agent Record of the surrounding agent.
10. Let _isLittleEndian_ be _agentRecord_.[[LittleEndian]].
11. Let _expectedBytes_ be NumericToRawBytes(_elementType_, _expected_, _isLittleEndian_).
12. Let _replacementBytes_ be NumericToRawBytes(_elementType_, _replacement_, _isLittleEndian_).
13. If IsSharedArrayBuffer(_buffer_) is *true*, then
   a. Let _rawBytesRead_ be AtomicCompareExchangeInSharedBlock(_block_, _byteIndexInBuffer_, _elementSize_, _expectedBytes_, _replacementBytes_).
14. Else,
   a. Let _rawBytesRead_ be a List of length _elementSize_ whose elements are the sequence of _elementSize_ bytes starting with _block_[_byteIndexInBuffer_].
   b. If ByteListEqual(_rawBytesRead_, _expectedBytes_) is *true*, then
      i. Store the individual bytes of _replacementBytes_ into _block_, starting at _block_[_byteIndexInBuffer_].
15. Return RawBytesToNumeric(_elementType_, _rawBytesRead_, _isLittleEndian_).

--- referenced: ValidateAtomicAccessOnIntegerTypedArray ( _ta_: an ECMAScript language value, _requestIndex_: an ECMAScript language value, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-validateatomicaccessonintegertypedarray

1. Let _taRecord_ be ? ValidateIntegerTypedArray(_ta_, *false*).
2. Return ? ValidateAtomicAccess(_taRecord_, _requestIndex_).

--- referenced: ToBigInt ( _arg_: an ECMAScript language value, ): either a normal completion containing a BigInt or a throw completion
  id: sec-tobigint

description: It converts _arg_ to a BigInt value, or throws if an implicit conversion from Number would be required.

1. Let _primitive_ be ? ToPrimitive(_arg_, ~number~).
2. Return the value that _primitive_ corresponds to in [table-tobigint].

Argument Type
|
Result
|
Undefined
|
Throw a *TypeError* exception.
|
Null
|
Throw a *TypeError* exception.
|
Boolean
|
Return `1n` if _primitive_ is *true* and `0n` if _primitive_ is *false*.
|
BigInt
|
Return _primitive_.
|
Number
|
Throw a *TypeError* exception.
|
String
|

1. Let _n_ be StringToBigInt(_primitive_).
2. If _n_ is *undefined*, throw a *SyntaxError* exception.
3. Return _n_.

|
Symbol
|
Throw a *TypeError* exception.
|

--- referenced: ToIntegerOrInfinity ( _arg_: an ECMAScript language value, ): either a normal completion containing either an integer, +∞, or -∞, or a throw completion
  id: sec-tointegerorinfinity

description: It converts _arg_ to an integer representing its Number value with fractional part truncated, or to +∞ or -∞ when that Number value is infinite.

1. Let _number_ be ? ToNumber(_arg_).
2. If _number_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return 0.
3. If _number_ is *+∞*_𝔽, return +∞.
4. If _number_ is *-∞*_𝔽, return -∞.
5. Return truncate(ℝ(_number_)).

NOTE:
𝔽(ToIntegerOrInfinity(_x_)) never returns *-0*_𝔽 for any value of _x_. The truncation of the fractional part is performed after converting _x_ to a mathematical value.

--- referenced: RevalidateAtomicAccess ( _ta_: a TypedArray, _byteIndexInBuffer_: a non-negative integer, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-revalidateatomicaccess

description: This operation revalidates the index within the backing buffer for atomic operations after all argument coercions are performed in Atomics methods, as argument coercions can have arbitrary side effects, which could cause the buffer to become out of bounds. This operation does not throw when _ta_'s backing buffer is a SharedArrayBuffer.

1. NOTE: Bounds checking is not a synchronizing operation when _ta_'s backing buffer is a growable SharedArrayBuffer.
2. Let _taRecord_ be ? ValidateTypedArrayBounds(_ta_, ~unordered~).
3. Assert: _byteIndexInBuffer_ ≥ _ta_.[[ByteOffset]].
4. Let _elementSize_ be TypedArrayElementSize(_ta_).
5. Let _elementLastByteIndex_ be _byteIndexInBuffer_ + _elementSize_ - 1.
6. If _elementLastByteIndex_ ≥ _taRecord_.[[CachedBufferByteLength]], throw a *RangeError* exception.
7. Return ~unused~.

--- referenced: TypedArrayElementType ( _obj_: a TypedArray, ): a TypedArray element type
  id: sec-typedarrayelementtype

1. Return the Element Type value specified in [table-the-typedarray-constructors] for _obj_.[[TypedArrayName]].

--- referenced: TypedArrayElementSize ( _obj_: a TypedArray, ): a non-negative integer
  id: sec-typedarrayelementsize

1. Return the Element Size value specified in [table-the-typedarray-constructors] for _obj_.[[TypedArrayName]].

--- referenced: IsSharedArrayBuffer ( _obj_: an ArrayBuffer or a SharedArrayBuffer, ): a Boolean
  id: sec-issharedarraybuffer

description: It tests whether an object is a SharedArrayBuffer.

1. If _obj_.[[ArrayBufferData]] is a Shared Data Block, return *true*.
2. Return *false*.

--- referenced: ByteListEqual ( _xBytes_: a List of byte values, _yBytes_: a List of byte values, ): a Boolean
  id: sec-bytelistequal

1. If _xBytes_ and _yBytes_ do not have the same number of elements, return *false*.
2. Let _i_ be 0.
3. For each element _xByte_ of _xBytes_, do
   a. Let _yByte_ be _yBytes_[_i_].
   b. If _xByte_ ≠ _yByte_, return *false*.
   c. Set _i_ to _i_ + 1.
4. Return *true*.


## Done when

- Every test262 run under built-ins/Atomics/compareExchange/ passes (check with `./t262 built-ins/Atomics/compareExchange/`). 0 pass on the main branch right now.

