//! %TypedArray%.prototype.slice ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.slice

use kahvi_core::prelude::*;

/// TypedArray.prototype.slice
pub(crate) fn prototype_slice(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.slice` as defined in [sec-array.prototype.slice].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _sourceArrayLength_ be TypedArrayLength(_taRecord_).
    // 4. Let _startIndex_ be ? ToClampedIndex(_start_, _sourceArrayLength_).
    // 5. If _end_ is *undefined*, let _endIndex_ be _sourceArrayLength_; else let _endIndex_ be ? ToClampedIndex(_end_, _sourceArrayLength_).
    // 6. Let _countBytes_ be max(_endIndex_ - _startIndex_, 0).
    // 7. Let _resultArray_ be ? TypedArraySpeciesCreate(_obj_, « 𝔽(_countBytes_) »).
    // 8. If _countBytes_ > 0, then
    //    a. Set _taRecord_ to ? ValidateTypedArrayBounds(_obj_, ~seq-cst~).
    //    b. Set _endIndex_ to min(_endIndex_, TypedArrayLength(_taRecord_)).
    //    c. Set _countBytes_ to max(_endIndex_ - _startIndex_, 0).
    //    d. Let _sourceType_ be TypedArrayElementType(_obj_).
    //    e. Let _targetType_ be TypedArrayElementType(_resultArray_).
    //    f. If _sourceType_ is _targetType_, then
    //       i. NOTE: The transfer must be performed in a manner that preserves the bit-level encoding of the source data.
    //       ii. Let _sourceBuffer_ be _obj_.[[ViewedArrayBuffer]].
    //       iii. Let _targetBuffer_ be _resultArray_.[[ViewedArrayBuffer]].
    //       iv. Let _elementSize_ be TypedArrayElementSize(_obj_).
    //       v. Let _sourceByteOffset_ be _obj_.[[ByteOffset]].
    //       vi. Let _sourceByteIndex_ be (_startIndex_ × _elementSize_) + _sourceByteOffset_.
    //       vii. Let _targetByteIndex_ be _resultArray_.[[ByteOffset]].
    //       viii. Let _endByteIndex_ be _targetByteIndex_ + (_countBytes_ × _elementSize_).
    //       ix. Repeat, while _targetByteIndex_ < _endByteIndex_,
    //          1. Let _value_ be GetValueFromBuffer(_sourceBuffer_, _sourceByteIndex_, ~uint8~, *true*, ~unordered~).
    //          2. Perform SetValueInBuffer(_targetBuffer_, _targetByteIndex_, ~uint8~, _value_, *true*, ~unordered~).
    //          3. Set _sourceByteIndex_ to _sourceByteIndex_ + 1.
    //          4. Set _targetByteIndex_ to _targetByteIndex_ + 1.
    //    g. Else,
    //       i. Let _n_ be 0.
    //       ii. Let _k_ be _startIndex_.
    //       iii. Repeat, while _k_ < _endIndex_,
    //          1. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //          2. Perform ! TypedArraySetElement(_resultArray_, 𝔽(_n_), _kValue_).
    //          3. Set _k_ to _k_ + 1.
    //          4. Set _n_ to _n_ + 1.
    // 9. Return _resultArray_.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.slice"))
}
