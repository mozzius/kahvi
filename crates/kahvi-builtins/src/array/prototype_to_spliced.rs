//! Array.prototype.toSpliced ( _start_, _skipCount_, ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.tospliced

use kahvi_core::prelude::*;

/// Array.prototype.toSpliced
pub(crate) fn prototype_to_spliced(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    let length = ops::length_of_array_like(agent, obj)?;
    // 3. Let _actualStart_ be ? ToClampedIndex(_start_, _length_).
    let actual_start = to_clamped_index(agent, args.get_or_undefined(0), length)?;
    // 4. Let _insertCount_ be the number of elements in _items_.
    let items = args.get(2..).unwrap_or(&[]);
    let insert_count = items.len() as i128;
    // 5. Let _maxSkipCount_ be _length_ - _actualStart_.
    let max_skip_count = length - actual_start;
    let actual_skip_count = if args.is_empty() {
        // 6. If _start_ is not present, then
        //    a. Let _actualSkipCount_ be 0.
        0.0
    } else if args.len() < 2 {
        // 7. Else if _skipCount_ is not present, then
        //    a. Let _actualSkipCount_ be _maxSkipCount_.
        max_skip_count
    } else {
        // 8. Else,
        //    a. Let _actualSkipCount_ be the result of clamping ? ToIntegerOrInfinity(_skipCount_) between 0 and _maxSkipCount_.
        let skip_count = ops::to_integer_or_infinity(agent, args.get_or_undefined(1))?;
        skip_count.max(0.0).min(max_skip_count)
    };
    // 9. Let _newLength_ be _length_ + _insertCount_ - _actualSkipCount_.
    // These are mathematical values: all three are integers of at most
    // 2^53 - 1, so the sum is exact in i128 (in f64 it can round down to
    // 2^53 - 1 and miss step 11).
    let new_length_exact = length as i128 + insert_count - actual_skip_count as i128;
    let new_length = new_length_exact as f64;
    // 10. Assert: _newLength_ ≥ 0.
    // 11. If _newLength_ > 2^53 - 1, throw a *TypeError* exception.
    if new_length_exact > 9_007_199_254_740_991 {
        return Err(agent.throw_type_error("Array.prototype.toSpliced length exceeds 2^53 - 1"));
    }
    // 12. Let _newArray_ be ? ArrayCreate(_newLength_).
    let new_array = kahvi_core::object::array::array_create(agent, new_length, None)?;
    // 13. Let _writeIndex_ be 0.
    let mut write_index = 0.0;
    // 14. Let _readIndex_ be _actualStart_ + _actualSkipCount_.
    let mut read_index = actual_start + actual_skip_count;
    // 15. Repeat, while _writeIndex_ < _actualStart_,
    while write_index < actual_start {
        // a. Let _propertyKey_ be ! ToString(𝔽(_writeIndex_)).
        let property_key = agent.key_from_index(write_index);
        // b. Let _iValue_ be ? Get(_obj_, _propertyKey_).
        let i_value = ops::get(agent, obj, property_key)?;
        // c. Perform ! CreateDataPropertyOrThrow(_newArray_, _propertyKey_, _iValue_).
        ops::create_data_property_or_throw(agent, new_array, property_key, i_value)?;
        // d. Set _writeIndex_ to _writeIndex_ + 1.
        write_index += 1.0;
    }
    // 16. For each element _item_ of _items_, do
    for item in items {
        // a. Let _propertyKey_ be ! ToString(𝔽(_writeIndex_)).
        let property_key = agent.key_from_index(write_index);
        // b. Perform ! CreateDataPropertyOrThrow(_newArray_, _propertyKey_, _item_).
        ops::create_data_property_or_throw(agent, new_array, property_key, *item)?;
        // c. Set _writeIndex_ to _writeIndex_ + 1.
        write_index += 1.0;
    }
    // 17. Repeat, while _writeIndex_ < _newLength_,
    while write_index < new_length {
        // a. Let _propertyKey_ be ! ToString(𝔽(_writeIndex_)).
        let property_key = agent.key_from_index(write_index);
        // b. Let _from_ be ! ToString(𝔽(_readIndex_)).
        let from = agent.key_from_index(read_index);
        // c. Let _fromValue_ be ? Get(_obj_, _from_).
        let from_value = ops::get(agent, obj, from)?;
        // d. Perform ! CreateDataPropertyOrThrow(_newArray_, _propertyKey_, _fromValue_).
        ops::create_data_property_or_throw(agent, new_array, property_key, from_value)?;
        // e. Set _writeIndex_ to _writeIndex_ + 1.
        write_index += 1.0;
        // f. Set _readIndex_ to _readIndex_ + 1.
        read_index += 1.0;
    }
    // 18. Return _newArray_.
    Ok(Value::Object(new_array))
}

/// ToClampedIndex ( _value_, _length_ )
///
/// Spec: https://tc39.es/ecma262/#sec-toclampedindex
///
/// Not yet in `ops`; only this built-in needs it so far.
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
    Ok(index.max(0.0).min(length))
}
