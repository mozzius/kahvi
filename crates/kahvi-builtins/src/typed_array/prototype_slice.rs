//! %TypedArray%.prototype.slice ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.slice

use kahvi_core::object::typed_array::{
    ElementType, get_value_from_buffer, set_value_in_buffer, typed_array_data, typed_array_length,
};
use kahvi_core::prelude::*;

use crate::support::typed_array::{
    typed_array_get_element, typed_array_set_element, typed_array_species_create,
    validate_typed_array,
};

/// ToClampedIndex(_value_, _length_)
///
/// Spec: https://tc39.es/ecma262/#sec-toclampedindex
///
/// Not yet in `ops`; only the TypedArray and Array built-ins need it so far.
fn to_clamped_index(agent: &mut Agent, value: Value, length: f64) -> JsResult<f64> {
    // 1. Let _index_ be ? ToAbsoluteIndex(_value_, _length_).
    //    ToAbsoluteIndex ( _start_, _length_ ):
    //    1. Let _relativeIndex_ be ? ToIntegerOrInfinity(_start_).
    let relative_index = ops::to_integer_or_infinity(agent, value)?;
    //    2. If _relativeIndex_ is -∞, return +0𝔽.
    //    3. Else if _relativeIndex_ < 0, return max(_length_ + _relativeIndex_, 0).
    //    4. Else, return min(_relativeIndex_, _length_).
    let index = if relative_index == f64::NEG_INFINITY {
        0.0
    } else if relative_index < 0.0 {
        (length + relative_index).max(0.0)
    } else {
        relative_index.min(length)
    };
    // 2. Return the result of clamping _index_ between 0 and _length_.
    Ok(index.clamp(0.0, length))
}

/// %TypedArray%.prototype.slice
pub(crate) fn prototype_slice(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, obj)?;
    let Value::Object(obj) = obj else {
        unreachable!("ValidateTypedArray accepted a non-object")
    };
    // 3. Let _sourceArrayLength_ be TypedArrayLength(_taRecord_).
    let source_array_length = typed_array_length(agent, ta_record) as f64;
    // 4. Let _startIndex_ be ? ToClampedIndex(_start_, _sourceArrayLength_).
    let start_index = to_clamped_index(agent, args.get_or_undefined(0), source_array_length)?;
    // 5. If _end_ is *undefined*, let _endIndex_ be _sourceArrayLength_; else let _endIndex_ be ? ToClampedIndex(_end_, _sourceArrayLength_).
    let end = args.get_or_undefined(1);
    let mut end_index = if end.is_undefined() {
        source_array_length
    } else {
        to_clamped_index(agent, end, source_array_length)?
    };
    // 6. Let _countBytes_ be max(_endIndex_ - _startIndex_, 0).
    let mut count_bytes = (end_index - start_index).max(0.0);
    // 7. Let _resultArray_ be ? TypedArraySpeciesCreate(_obj_, « 𝔽(_countBytes_) »).
    let result_array = typed_array_species_create(agent, obj, &[Value::Number(count_bytes)])?;
    // 8. If _countBytes_ > 0, then
    if count_bytes > 0.0 {
        // a. Set _taRecord_ to ? ValidateTypedArrayBounds(_obj_, ~seq-cst~).
        let ta_record = validate_typed_array(agent, Value::Object(obj))?;
        // b. Set _endIndex_ to min(_endIndex_, TypedArrayLength(_taRecord_)).
        end_index = end_index.min(typed_array_length(agent, ta_record) as f64);
        // c. Set _countBytes_ to max(_endIndex_ - _startIndex_, 0).
        count_bytes = (end_index - start_index).max(0.0);
        // d. Let _sourceType_ be TypedArrayElementType(_obj_).
        let source_data = typed_array_data(agent, obj).expect("a TypedArray");
        let source_type = source_data.kind;
        // e. Let _targetType_ be TypedArrayElementType(_resultArray_).
        let target_data = typed_array_data(agent, result_array).expect("a TypedArray");
        let target_type = target_data.kind;
        // f. If _sourceType_ is _targetType_, then
        if source_type == target_type {
            // i. NOTE: The transfer must be performed in a manner that preserves the bit-level encoding of the source data.
            // ii. Let _sourceBuffer_ be _obj_.[[ViewedArrayBuffer]].
            let source_buffer = source_data.buffer.expect("initialized");
            // iii. Let _targetBuffer_ be _resultArray_.[[ViewedArrayBuffer]].
            let target_buffer = target_data.buffer.expect("initialized");
            // iv. Let _elementSize_ be TypedArrayElementSize(_obj_).
            let element_size = source_type.size();
            // v. Let _sourceByteOffset_ be _obj_.[[ByteOffset]].
            let source_byte_offset = source_data.byte_offset;
            // vi. Let _sourceByteIndex_ be (_startIndex_ × _elementSize_) + _sourceByteOffset_.
            let mut source_byte_index = start_index as usize * element_size + source_byte_offset;
            // vii. Let _targetByteIndex_ be _resultArray_.[[ByteOffset]].
            let mut target_byte_index = target_data.byte_offset;
            // viii. Let _endByteIndex_ be _targetByteIndex_ + (_countBytes_ × _elementSize_).
            let end_byte_index = target_byte_index + count_bytes as usize * element_size;
            // ix. Repeat, while _targetByteIndex_ < _endByteIndex_,
            while target_byte_index < end_byte_index {
                // 1. Let _value_ be GetValueFromBuffer(_sourceBuffer_, _sourceByteIndex_, ~uint8~, *true*, ~unordered~).
                let value = get_value_from_buffer(
                    agent,
                    source_buffer,
                    source_byte_index,
                    ElementType::Uint8,
                    true,
                );
                // 2. Perform SetValueInBuffer(_targetBuffer_, _targetByteIndex_, ~uint8~, _value_, *true*, ~unordered~).
                set_value_in_buffer(
                    agent,
                    target_buffer,
                    target_byte_index,
                    ElementType::Uint8,
                    value,
                    true,
                );
                // 3. Set _sourceByteIndex_ to _sourceByteIndex_ + 1.
                source_byte_index += 1;
                // 4. Set _targetByteIndex_ to _targetByteIndex_ + 1.
                target_byte_index += 1;
            }
        } else {
            // g. Else,
            // i. Let _n_ be 0.
            let mut n = 0.0;
            // ii. Let _k_ be _startIndex_.
            let mut k = start_index;
            // iii. Repeat, while _k_ < _endIndex_,
            while k < end_index {
                // 1. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
                let k_value = typed_array_get_element(agent, obj, k);
                // 2. Perform ! TypedArraySetElement(_resultArray_, 𝔽(_n_), _kValue_).
                typed_array_set_element(agent, result_array, n, k_value)?;
                // 3. Set _k_ to _k_ + 1.
                k += 1.0;
                // 4. Set _n_ to _n_ + 1.
                n += 1.0;
            }
        }
    }
    // 9. Return _resultArray_.
    Ok(Value::Object(result_array))
}
