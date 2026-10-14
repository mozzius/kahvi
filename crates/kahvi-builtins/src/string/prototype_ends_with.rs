//! String.prototype.endsWith ( _searchString_ [ , _endPosition_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.endswith

use kahvi_core::prelude::*;

/// String.prototype.endsWith
pub(crate) fn prototype_ends_with(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _isRegexp_ be ? IsRegExp(_searchString_).
    // 5. If _isRegexp_ is *true*, throw a *TypeError* exception.
    // 6. Set _searchString_ to ? ToString(_searchString_).
    // 7. Let _length_ be the length of _string_.
    // 8. If _endPosition_ is *undefined*, let _end_ be _length_; else let _end_ be the result of clamping ? ToIntegerOrInfinity(_endPosition_) between 0 and _length_.
    // 9. Let _searchLength_ be the length of _searchString_.
    // 10. If _searchLength_ = 0, return *true*.
    // 11. Let _start_ be _end_ - _searchLength_.
    // 12. If _start_ < 0, return *false*.
    // 13. Let _substring_ be the substring of _string_ from _start_ to _end_.
    // 14. If _substring_ is _searchString_, return *true*.
    // 15. Return *false*.
    //
    // NOTE:
    // Throwing an exception if the first argument is a RegExp is specified in order to allow future editions to define extensions that allow such argument values.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 4. Let _isRegexp_ be ? IsRegExp(_searchString_).
    let search_arg = args.get_or_undefined(0);
    let is_regexp = ops::is_regexp(agent, search_arg)?;
    // 5. If _isRegexp_ is *true*, throw a *TypeError* exception.
    if is_regexp {
        return Err(agent.throw_type_error(
            "String.prototype.endsWith called with a RegExp as the search string",
        ));
    }
    // 6. Set _searchString_ to ? ToString(_searchString_).
    let search_string = ops::to_string(agent, search_arg)?;
    // 7. Let _length_ be the length of _string_.
    let length = agent.str_len(string) as f64;
    // 8. If _endPosition_ is *undefined*, let _end_ be _length_; else let _end_ be the result of clamping ? ToIntegerOrInfinity(_endPosition_) between 0 and _length_.
    let end = match args.get_or_undefined(1) {
        Value::Undefined => length,
        end_position => {
            let pos = ops::to_integer_or_infinity(agent, end_position)?;
            pos.max(0.0).min(length)
        }
    };
    // 9. Let _searchLength_ be the length of _searchString_.
    let search_length = agent.str_len(search_string) as f64;
    // 10. If _searchLength_ = 0, return *true*.
    if search_length == 0.0 {
        return Ok(Value::Boolean(true));
    }
    // 11. Let _start_ be _end_ - _searchLength_.
    let start = end - search_length;
    // 12. If _start_ < 0, return *false*.
    if start < 0.0 {
        return Ok(Value::Boolean(false));
    }
    // 13. Let _substring_ be the substring of _string_ from _start_ to _end_.
    // 14. If _substring_ is _searchString_, return *true*.
    // 15. Return *false*.
    let (start, end) = (start as usize, end as usize);
    let is_match = match agent.str(string).get(start..end) {
        Some(substring) => substring == agent.str(search_string),
        None => false,
    };
    Ok(Value::Boolean(is_match))
}
