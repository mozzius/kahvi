//! String.prototype.lastIndexOf ( _searchString_ [ , _position_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.lastindexof

use kahvi_core::prelude::*;

/// String.prototype.lastIndexOf
pub(crate) fn prototype_last_index_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // If _searchString_ appears as a substring of the result of converting this object to a String at one or more indices that are smaller than or equal to _position_, then the greatest such index is returned; otherwise, *-1*_𝔽 is returned. If _position_ is *undefined*, the length of the String value is assumed, so as to search all of the String.
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 4. Set _searchString_ to ? ToString(_searchString_).
    let search_string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 5. Let _numberPosition_ be ? ToNumber(_position_).
    let number_position = ops::to_number(agent, args.get_or_undefined(1))?;
    // 6. Let _length_ be the length of _string_.
    let length = agent.str_len(string);
    // 7. Let _searchLength_ be the length of _searchString_.
    let search_length = agent.str_len(search_string);
    // 8. Let _maxStart_ be _length_ - _searchLength_.
    // 9. If _maxStart_ < 0, return *-1*_𝔽.
    if search_length > length {
        return Ok(Value::Number(-1.0));
    }
    let max_start = length - search_length;
    // 10. If _numberPosition_ is *NaN*, let _start_ be _maxStart_; else let _start_ be the result of clamping ! ToIntegerOrInfinity(_numberPosition_) between 0 and _maxStart_.
    let start = if number_position.is_nan() {
        max_start
    } else {
        let position = ops::to_integer_or_infinity(agent, Value::Number(number_position))?;
        position.max(0.0).min(max_start as f64) as usize
    };
    // 11. Assert: If _position_ is *undefined*, then _start_ is _maxStart_.
    // 12. Let _result_ be StringLastIndexOf(_string_, _searchString_, _start_).
    let result = string_last_index_of(agent.str(string), agent.str(search_string), start);
    // 13. If _result_ is ~not-found~, return *-1*_𝔽.
    // 14. Return 𝔽(_result_).
    Ok(Value::Number(match result {
        Some(index) => index as f64,
        None => -1.0,
    }))
}

/// StringLastIndexOf(string, searchValue, fromIndex)
///
/// Returns `None` for the spec's ~not-found~.
fn string_last_index_of(string: &[u16], search_value: &[u16], from_index: usize) -> Option<usize> {
    // 1. Let _length_ be the length of _string_.
    let length = string.len();
    // 2. Let _searchLength_ be the length of _searchValue_.
    let search_length = search_value.len();
    // 3. Assert: _fromIndex_ + _searchLength_ ≤ _length_.
    if search_length > length {
        // Unreachable from the caller, which returns -1 first; kept as ~not-found~ so no index is read out of range.
        return None;
    }
    let from_index = from_index.min(length - search_length);
    // 4. For each integer _i_ such that 0 ≤ _i_ ≤ _fromIndex_, in descending order, do
    for i in (0..=from_index).rev() {
        // a. Let _candidate_ be the substring of _string_ from _i_ to _i_ + _searchLength_.
        let candidate = &string[i..i + search_length];
        // b. If _candidate_ is _searchValue_, return _i_.
        if candidate == search_value {
            return Some(i);
        }
    }
    // 5. Return ~not-found~.
    None
}
