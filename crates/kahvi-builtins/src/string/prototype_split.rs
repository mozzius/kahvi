//! String.prototype.split ( _separator_, _limit_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.split

use kahvi_core::prelude::*;

/// String.prototype.split
pub(crate) fn prototype_split(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns an Array into which substrings of the result of converting this object to a String have been stored. The substrings are determined by searching from left to right for occurrences of _separator_; these occurrences are not part of any String in the returned array, but serve to divide up the String value. The value of _separator_ may be a String of any length or it may be an object, such as a RegExp, that has a %Symbol.split% method.
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    let separator = args.get_or_undefined(0);
    let limit = args.get_or_undefined(1);
    // 3. If _separator_ is an Object, then
    if let Value::Object(_) = separator {
        // a. Let _splitter_ be ? GetMethod(_separator_, %Symbol.split%).
        let splitter = ops::get_method(
            agent,
            separator,
            PropertyKey::Symbol(agent.symbols_wk.split),
        )?;
        // b. If _splitter_ is not *undefined*, then
        if !matches!(splitter, Value::Undefined) {
            // i. Return ? Call(_splitter_, _separator_, « _thisValue_, _limit_ »).
            return ops::call(agent, splitter, separator, &[this_value, limit]);
        }
    }
    // 4. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 5. If _limit_ is *undefined*, let _lim_ be 2^32 - 1; else let _lim_ be ℝ(? ToUint32(_limit_)).
    let lim: f64 = if matches!(limit, Value::Undefined) {
        4_294_967_295.0
    } else {
        f64::from(ops::to_uint32(agent, limit)?)
    };
    // 6. Let _separatorString_ be ? ToString(_separator_).
    let separator_string = ops::to_string(agent, separator)?;
    // 7. If _lim_ = 0, then
    if lim == 0.0 {
        // a. Return CreateArrayFromList(« »).
        return Ok(Value::Object(ops::create_array_from_list(agent, &[])));
    }
    // 8. If _separator_ is *undefined*, then
    if matches!(separator, Value::Undefined) {
        // a. Return CreateArrayFromList(« _string_ »).
        return Ok(Value::Object(ops::create_array_from_list(
            agent,
            &[Value::String(string)],
        )));
    }
    // 9. Let _separatorLength_ be the length of _separatorString_.
    let separator_length = agent.str_len(separator_string);
    // 10. If _separatorLength_ = 0, then
    if separator_length == 0 {
        // a. Let _stringLength_ be the length of _string_.
        let string_length = agent.str_len(string);
        // b. Let _outLength_ be the result of clamping _lim_ between 0 and _stringLength_.
        let out_length = lim.min(string_length as f64) as usize;
        // c. Let _head_ be the substring of _string_ from 0 to _outLength_.
        // d. Let _codeUnits_ be a List consisting of the sequence of code units that are the elements of _head_.
        let mut code_units = Vec::with_capacity(out_length);
        for i in 0..out_length {
            code_units.push(Value::String(agent.substring(string, i, i + 1)));
        }
        // e. Return CreateArrayFromList(_codeUnits_).
        return Ok(Value::Object(ops::create_array_from_list(
            agent,
            &code_units,
        )));
    }
    // 11. If _string_ is the empty String, return CreateArrayFromList(« _string_ »).
    if agent.str_len(string) == 0 {
        return Ok(Value::Object(ops::create_array_from_list(
            agent,
            &[Value::String(string)],
        )));
    }
    // 12. Let _substrings_ be a new empty List.
    let mut substrings: Vec<Value> = Vec::new();
    // 13. Let _searchStart_ be 0.
    let mut search_start: usize = 0;
    let string_units = agent.str(string).to_vec();
    let separator_units = agent.str(separator_string).to_vec();
    // 14. Let _matchIndex_ be StringIndexOf(_string_, _separatorString_, 0).
    let mut match_index = string_index_of(&string_units, &separator_units, 0);
    // 15. Repeat, while _matchIndex_ is not ~not-found~,
    while let Some(index) = match_index {
        // a. Let _substring_ be the substring of _string_ from _searchStart_ to _matchIndex_.
        let substring = agent.substring(string, search_start, index);
        // b. Append _substring_ to _substrings_.
        substrings.push(Value::String(substring));
        // c. If the number of elements in _substrings_ is _lim_, return CreateArrayFromList(_substrings_).
        if substrings.len() as f64 == lim {
            return Ok(Value::Object(ops::create_array_from_list(
                agent,
                &substrings,
            )));
        }
        // d. Set _searchStart_ to _matchIndex_ + _separatorLength_.
        search_start = index + separator_length;
        // e. Set _matchIndex_ to StringIndexOf(_string_, _separatorString_, _searchStart_).
        match_index = string_index_of(&string_units, &separator_units, search_start);
    }
    // 16. Let _substring_ be the substring of _string_ from _searchStart_.
    let substring = agent.substring(string, search_start, string_units.len());
    // 17. Append _substring_ to _substrings_.
    substrings.push(Value::String(substring));
    // 18. Return CreateArrayFromList(_substrings_).
    Ok(Value::Object(ops::create_array_from_list(
        agent,
        &substrings,
    )))
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
