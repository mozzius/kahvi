//! String.prototype.substr ( _start_, _length_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.substr

use kahvi_core::prelude::*;

/// String.prototype.substr
pub(crate) fn prototype_substr(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a substring of the result of converting the *this* value to a String, starting from index _start_ and running for _length_ code units (or through the end of the String if _length_ is *undefined*). If _start_ is negative, it is treated as _sourceLength_ + _start_ where _sourceLength_ is the length of the String. The result is a String value, not a String object.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Perform ? RequireObjectCoercible(_obj_).
    ops::require_object_coercible(agent, obj)?;
    // 3. Let _string_ be ? ToString(_obj_).
    let string = ops::to_string(agent, obj)?;
    // 4. Let _size_ be the length of _string_.
    let size = agent.str_len(string) as f64;
    // 5. Let _intStart_ be ? ToClampedIndex(_start_, _size_).
    let int_start = to_clamped_index(agent, args.get_or_undefined(0), size)?;
    // 6. If _length_ is *undefined*, let _intLength_ be _size_; else let _intLength_ be the result of clamping ? ToIntegerOrInfinity(_length_) between 0 and _size_.
    let length = args.get_or_undefined(1);
    let int_length = if length.is_undefined() {
        size
    } else {
        ops::to_integer_or_infinity(agent, length)?.clamp(0.0, size)
    };
    // 7. Let _intEnd_ be min(_intStart_ + _intLength_, _size_).
    let int_end = (int_start + int_length).min(size);
    // 8. Return the substring of _string_ from _intStart_ to _intEnd_.
    if int_start >= int_end {
        return Ok(Value::String(agent.names.empty));
    }
    Ok(Value::String(agent.substring(
        string,
        int_start as usize,
        int_end as usize,
    )))
}

/// ToClampedIndex ( _value_, _length_ )
///
/// Converts _value_ to an integer (interpreting negative values relative to
/// _length_) and clamps the result to the inclusive interval from 0 to _length_.
fn to_clamped_index(agent: &mut Agent, value: Value, length: f64) -> JsResult<f64> {
    // 1. Let _index_ be ? ToAbsoluteIndex(_value_, _length_).
    // ToAbsoluteIndex ( _relativeIndex_: a Number, _length_: a non-negative integer, ): a non-negative integer
    //   1. If relativeIndex = -∞, return 0.
    //   2. If relativeIndex < 0, return max(length + relativeIndex, 0).
    //   3. Return min(relativeIndex, length).
    let relative_index = ops::to_integer_or_infinity(agent, value)?;
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
