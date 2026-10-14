//! %TypedArray%.prototype.copyWithin ( _target_, _start_ [ , _end_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.copywithin

use kahvi_core::prelude::*;

/// TypedArray.prototype.copyWithin
pub(crate) fn prototype_copy_within(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.copyWithin` as defined in [sec-array.prototype.copywithin].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. Let _targetIndex_ be ? ToClampedIndex(_target_, _length_).
    // 5. Let _startIndex_ be ? ToClampedIndex(_start_, _length_).
    // 6. If _end_ is *undefined*, let _endIndex_ be _length_; else let _endIndex_ be ? ToClampedIndex(_end_, _length_).
    // 7. Let _count_ be min(_endIndex_ - _startIndex_, _length_ - _targetIndex_).
    // 8. If _count_ > 0, then
    //    a. NOTE: The copying must be performed in a manner that preserves the bit-level encoding of the source data.
    //    b. Let _buffer_ be _obj_.[[ViewedArrayBuffer]].
    //    c. Set _taRecord_ to ? ValidateTypedArrayBounds(_obj_, ~seq-cst~).
    //    d. Set _length_ to TypedArrayLength(_taRecord_).
    //    e. NOTE: Side-effects of the above steps may have reduced the size of _obj_, in which case copying should proceed with the longest still-applicable prefix.
    //    f. Set _count_ to min(_count_, _length_ - _startIndex_, _length_ - _targetIndex_).
    //    g. Let _elementSize_ be TypedArrayElementSize(_obj_).
    //    h. Let _byteOffset_ be _obj_.[[ByteOffset]].
    //    i. Let _toByteIndex_ be (_targetIndex_ × _elementSize_) + _byteOffset_.
    //    j. Let _fromByteIndex_ be (_startIndex_ × _elementSize_) + _byteOffset_.
    //    k. Let _countBytes_ be _count_ × _elementSize_.
    //    l. If _fromByteIndex_ < _toByteIndex_ and _toByteIndex_ < _fromByteIndex_ + _countBytes_, then
    //       i. Let _direction_ be -1.
    //       ii. Set _fromByteIndex_ to _fromByteIndex_ + _countBytes_ - 1.
    //       iii. Set _toByteIndex_ to _toByteIndex_ + _countBytes_ - 1.
    //    m. Else,
    //       i. Let _direction_ be 1.
    //    n. Repeat, while _countBytes_ > 0,
    //       i. Assert: _fromByteIndex_ ≥ 0 and _toByteIndex_ ≥ 0.
    //       ii. Let _value_ be GetValueFromBuffer(_buffer_, _fromByteIndex_, ~uint8~, *true*, ~unordered~).
    //       iii. Perform SetValueInBuffer(_buffer_, _toByteIndex_, ~uint8~, _value_, *true*, ~unordered~).
    //       iv. Set _fromByteIndex_ to _fromByteIndex_ + _direction_.
    //       v. Set _toByteIndex_ to _toByteIndex_ + _direction_.
    //       vi. Set _countBytes_ to _countBytes_ - 1.
    // 9. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.copyWithin"))
}
