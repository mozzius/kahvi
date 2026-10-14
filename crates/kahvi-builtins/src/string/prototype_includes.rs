//! String.prototype.includes ( _searchString_ [ , _position_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.includes

use kahvi_core::prelude::*;

/// String.prototype.includes
pub(crate) fn prototype_includes(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
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
        return Err(
            agent.throw_type_error("String.prototype.includes must not be called with a RegExp")
        );
    }
    // 6. Set _searchString_ to ? ToString(_searchString_).
    let search_string = ops::to_string(agent, search_arg)?;
    // 7. Let _length_ be the length of _string_.
    let length = agent.str_len(string);
    // 8. Let _start_ be the result of clamping ? ToIntegerOrInfinity(_position_) between 0 and _length_.
    let position = ops::to_integer_or_infinity(agent, args.get_or_undefined(1))?;
    let start = position.max(0.0).min(length as f64);
    // 9. Assert: If _position_ is *undefined*, then _start_ is 0.
    // 10. Let _index_ be StringIndexOf(_string_, _searchString_, _start_).
    let index = string_index_of(agent.str(string), agent.str(search_string), start as usize);
    // 11. If _index_ is ~not-found~, return *false*.
    // 12. Return *true*.
    Ok(Value::Boolean(index.is_some()))
}

/// StringIndexOf(string, searchValue, fromIndex)
///
/// Returns `None` for the spec's ~not-found~.
fn string_index_of(string: &[u16], search_value: &[u16], from_index: usize) -> Option<usize> {
    // 1. Let _length_ be the length of _string_.
    let length = string.len();
    // 2. If _searchValue_ is the empty String and _fromIndex_ ≤ _length_, return _fromIndex_.
    if search_value.is_empty() && from_index <= length {
        return Some(from_index);
    }
    // 3. Let _searchLength_ be the length of _searchValue_.
    let search_length = search_value.len();
    // 4. For each integer _i_ such that _fromIndex_ ≤ _i_ ≤ _length_ - _searchLength_, in ascending order, do
    if search_length > length {
        // The range is empty, so the search is ~not-found~.
        return None;
    }
    for i in from_index..=(length - search_length) {
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
