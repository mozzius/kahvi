//! String.prototype.substring ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.substring

use kahvi_core::prelude::*;

/// String.prototype.substring
pub(crate) fn prototype_substring(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a substring of the result of converting this object to a String, starting from index _start_ and running to, but not including, index _end_ of the String (or through the end of the String if _end_ is *undefined*). The result is a String value, not a String object.
    // If either argument is *NaN* or negative, it is replaced with zero; if either argument is strictly greater than the length of the String, it is replaced with the length of the String.
    // If _start_ is strictly greater than _end_, they are swapped.
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 4. Let _length_ be the length of _string_.
    let length = agent.str_len(string);
    // 5. Let _finalStart_ be the result of clamping ? ToIntegerOrInfinity(_start_) between 0 and _length_.
    let start = ops::to_integer_or_infinity(agent, args.get_or_undefined(0))?;
    let final_start = start.max(0.0).min(length as f64);
    // 6. Assert: If _start_ is *undefined*, then _finalStart_ is 0.
    // 7. If _end_ is *undefined*, let _finalEnd_ be _length_; else let _finalEnd_ be the result of clamping ? ToIntegerOrInfinity(_end_) between 0 and _length_.
    let end = args.get_or_undefined(1);
    let final_end = if end.is_undefined() {
        length as f64
    } else {
        ops::to_integer_or_infinity(agent, end)?
            .max(0.0)
            .min(length as f64)
    };
    // 8. Let _from_ be min(_finalStart_, _finalEnd_).
    let from = final_start.min(final_end);
    // 9. Let _to_ be max(_finalStart_, _finalEnd_).
    let to = final_start.max(final_end);
    // 10. Return the substring of _string_ from _from_ to _to_.
    Ok(Value::String(agent.substring(
        string,
        from as usize,
        to as usize,
    )))
}
