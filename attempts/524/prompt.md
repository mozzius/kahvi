# Ticket B-atomics-store: Atomics.store

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

- `crates/kahvi-builtins/src/atomics/store.rs`

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

Atomics.store ( _ta_, _index_, _value_ )
  id: sec-atomics.store   type: built-in function

This function performs the following steps when called:

1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
2. If _ta_.[[ContentType]] is ~bigint~, let _coerced_ be ? ToBigInt(_value_).
3. Else, let _coerced_ be 𝔽(? ToIntegerOrInfinity(_value_)).
4. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
5. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
6. Let _elementType_ be TypedArrayElementType(_ta_).
7. Perform SetValueInBuffer(_buffer_, _byteIndexInBuffer_, _elementType_, _coerced_, *true*, ~seq-cst~).
8. Return _coerced_.

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

--- referenced: SetValueInBuffer ( _arrayBuffer_: an ArrayBuffer or SharedArrayBuffer, _byteIndex_: a non-negative integer, _type_: a TypedArray element type, _value_: a Number or a BigInt, _isTypedArray_: a Boolean, _order_: ~seq-cst~, ~unordered~, or ~init~, optional _isLittleEndian_: a Boolean, ): ~unused~
  id: sec-setvalueinbuffer

1. Assert: IsDetachedBuffer(_arrayBuffer_) is *false*.
2. Assert: There are sufficient bytes in _arrayBuffer_ starting at _byteIndex_ to represent a value of _type_.
3. Assert: _value_ is a BigInt if IsBigIntElementType(_type_) is *true*; else, _value_ is a Number.
4. Let _block_ be _arrayBuffer_.[[ArrayBufferData]].
5. Let _elementSize_ be the Element Size value specified in [table-the-typedarray-constructors] for Element Type _type_.
6. Let _agentRecord_ be the Agent Record of the surrounding agent.
7. If _isLittleEndian_ is not present, then
   a. Set _isLittleEndian_ to _agentRecord_.[[LittleEndian]].
8. Let _rawBytes_ be NumericToRawBytes(_type_, _value_, _isLittleEndian_).
9. If IsSharedArrayBuffer(_arrayBuffer_) is *true*, then
   a. Let _execution_ be _agentRecord_.[[CandidateExecution]].
   b. Let _eventsRecord_ be the Agent Events Record of _execution_.[[EventsRecords]] whose [[AgentSignifier]] is AgentSignifier().
   c. If _isTypedArray_ is *true* and IsNoTearConfiguration(_type_, _order_) is *true*, let _noTear_ be *true*; else let _noTear_ be *false*.
   d. Append WriteSharedMemory { [[Order]]: _order_, [[NoTear]]: _noTear_, [[Block]]: _block_, [[ByteIndex]]: _byteIndex_, [[ElementSize]]: _elementSize_, [[Payload]]: _rawBytes_ } to _eventsRecord_.[[EventList]].
10. Else,
   a. Store the individual bytes of _rawBytes_ into _block_, starting at _block_[_byteIndex_].
11. Return ~unused~.


## Done when

- Every test262 run under built-ins/Atomics/store/ passes (check with `./t262 built-ins/Atomics/store/`). 8 pass on the main branch right now.

