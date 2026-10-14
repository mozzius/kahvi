//! RegExp.prototype [ %Symbol.split% ] ( _string_, _limit_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype-%symbol.split%

use kahvi_core::object::array::array_create;
use kahvi_core::prelude::*;

use crate::support::regexp::{advance_string_index, regexp_exec};

/// RegExp.prototype[Symbol.split]
pub(crate) fn prototype_symbol_split(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method returns an Array into which substrings of the result of converting _string_ to a String have been stored. The substrings are determined by searching from left to right for matches of the *this* value regular expression; these occurrences are not part of any String in the returned array, but serve to divide up the String value.
    // The *this* value may be an empty regular expression or a regular expression that can match an empty String. In this case, the regular expression does not match the empty substring at the beginning or end of the input String, nor does it match the empty substring at the end of the previous separator match. (For example, if the regular expression matches the empty String, the String is split up into individual code unit elements; the length of the result array equals the length of the String, and each substring contains one code unit.) Only the first match at a given index of the String is considered, even if backtracking could yield a non-empty substring match at that index. (For example, `/a*?/[Symbol.split]("ab")` evaluates to the array `["a", "b"]`, while `/a*/[Symbol.split]("ab")` evaluates to the array `["","b"]`.)
    // If _string_ is (or converts to) the empty String, the result depends on whether the regular expression can match the empty String. If it can, the result array contains no elements. Otherwise, the result array contains one element, which is the empty String.
    // If the regular expression contains capturing parentheses, then each time _separator_ is matched the results (including any *undefined* results) of the capturing parentheses are spliced into the output array. For example,
    // /<(\/)?([^<>]+)>/[Symbol.split]("A<B>bold</B>and<CODE>coded</CODE>")
    // evaluates to the array
    // ["A", undefined, "B", "bold", "/", "B", "and", undefined, "CODE", "coded", "/", "CODE", ""]
    // If _limit_ is not *undefined*, then the output array is truncated so that it contains no more than _limit_ elements.
    // This method performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    let Value::Object(regexp) = this else {
        return Err(agent.throw_type_error("RegExp.prototype[Symbol.split] called on a non-object"));
    };
    // 3. Set _string_ to ? ToString(_string_).
    let string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 4. Let _speciesCtor_ be ? SpeciesConstructor(_regexp_, %RegExp%).
    let default_ctor = agent.intrinsic("%RegExp%");
    let species_ctor = ops::species_constructor(agent, regexp, default_ctor)?;
    // 5. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
    let key = agent.key("flags");
    let flags_value = ops::get(agent, regexp, key)?;
    let flags = ops::to_string(agent, flags_value)?;
    let flags_units = agent.str(flags).to_vec();
    // 6. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _unicodeMatching_ be *true*.
    // 7. Else, let _unicodeMatching_ be *false*.
    let unicode_matching =
        flags_units.contains(&u16::from(b'u')) || flags_units.contains(&u16::from(b'v'));
    // 8. If _flags_ contains *"y"*, let _newFlags_ be _flags_.
    // 9. Else, let _newFlags_ be the string-concatenation of _flags_ and *"y"*.
    let new_flags = if flags_units.contains(&u16::from(b'y')) {
        flags
    } else {
        let mut units = flags_units.clone();
        units.push(u16::from(b'y'));
        agent.intern(&units)
    };
    // 10. Let _splitter_ be ? Construct(_speciesCtor_, « _regexp_, _newFlags_ »).
    let splitter = ops::construct(
        agent,
        species_ctor,
        &[Value::Object(regexp), Value::String(new_flags)],
        None,
    )?;
    // 11. Let _array_ be ! ArrayCreate(0).
    let array = array_create(agent, 0.0, None)?;
    // 12. Let _lengthA_ be 0.
    let mut length_a = 0.0;
    // 13. If _limit_ is *undefined*, let _lim_ be 2^32 - 1; else let _lim_ be ℝ(? ToUint32(_limit_)).
    let limit = args.get_or_undefined(1);
    let lim = match limit {
        Value::Undefined => 4_294_967_295.0,
        v => f64::from(ops::to_uint32(agent, v)?),
    };
    // 14. If _lim_ = 0, return _array_.
    if lim == 0.0 {
        return Ok(Value::Object(array));
    }
    // 15. If _string_ is the empty String, then
    if agent.str_len(string) == 0 {
        // a. Let _matchResult_ be ? RegExpExec(_splitter_, _string_).
        let match_result = regexp_exec(agent, splitter, string)?;
        // b. If _matchResult_ is not *null*, return _array_.
        if !matches!(match_result, Value::Null) {
            return Ok(Value::Object(array));
        }
        // c. Perform ! CreateDataPropertyOrThrow(_array_, *"0"*, _string_).
        let key = agent.key_from_index(0.0);
        ops::create_data_property_or_throw(agent, array, key, Value::String(string))?;
        // d. Return _array_.
        return Ok(Value::Object(array));
    }
    // 16. Let _size_ be the length of _string_.
    let size = agent.str_len(string) as f64;
    // 17. Let _lastMatchEnd_ be 0.
    let mut last_match_end = 0.0;
    // 18. Let _searchIndex_ be _lastMatchEnd_.
    let mut search_index = last_match_end;
    // 19. Repeat, while _searchIndex_ < _size_,
    while search_index < size {
        // a. Perform ? Set(_splitter_, *"lastIndex"*, 𝔽(_searchIndex_), *true*).
        let key = agent.key("lastIndex");
        ops::set(agent, splitter, key, Value::Number(search_index), true)?;
        // b. Let _matchResult_ be ? RegExpExec(_splitter_, _string_).
        let match_result = regexp_exec(agent, splitter, string)?;
        match match_result {
            // c. If _matchResult_ is *null*, then
            Value::Null => {
                // i. Set _searchIndex_ to AdvanceStringIndex(_string_, _searchIndex_, _unicodeMatching_).
                search_index =
                    advance_string_index(agent.str(string), search_index, unicode_matching);
            }
            // d. Else,
            Value::Object(match_result) => {
                // i. Let _matchEnd_ be ℝ(? ToLength(? Get(_splitter_, *"lastIndex"*))).
                let key = agent.key("lastIndex");
                let last_index = ops::get(agent, splitter, key)?;
                let mut match_end = ops::to_length(agent, last_index)?;
                // ii. Set _matchEnd_ to min(_matchEnd_, _size_).
                match_end = match_end.min(size);
                // iii. If _matchEnd_ = _lastMatchEnd_, then
                if match_end == last_match_end {
                    // 1. Set _searchIndex_ to AdvanceStringIndex(_string_, _searchIndex_, _unicodeMatching_).
                    search_index =
                        advance_string_index(agent.str(string), search_index, unicode_matching);
                } else {
                    // iv. Else,
                    // 1. Let _substring_ be the substring of _string_ from _lastMatchEnd_ to _searchIndex_.
                    let substring =
                        substring_units(agent.str(string), last_match_end, search_index);
                    let substring = Value::String(agent.intern(&substring));
                    // 2. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _substring_).
                    let key = agent.key_from_index(length_a);
                    ops::create_data_property_or_throw(agent, array, key, substring)?;
                    // 3. Set _lengthA_ to _lengthA_ + 1.
                    length_a += 1.0;
                    // 4. If _lengthA_ = _lim_, return _array_.
                    if length_a == lim {
                        return Ok(Value::Object(array));
                    }
                    // 5. Set _lastMatchEnd_ to _matchEnd_.
                    last_match_end = match_end;
                    // 6. Let _numberOfCaptures_ be ? LengthOfArrayLike(_matchResult_).
                    let number_of_captures = ops::length_of_array_like(agent, match_result)?;
                    // 7. Set _numberOfCaptures_ to max(_numberOfCaptures_ - 1, 0).
                    let number_of_captures = (number_of_captures - 1.0).max(0.0);
                    // 8. Let _captureIndex_ be 1.
                    let mut capture_index = 1.0;
                    // 9. Repeat, while _captureIndex_ ≤ _numberOfCaptures_,
                    while capture_index <= number_of_captures {
                        // a. Let _nextCapture_ be ? Get(_matchResult_, ! ToString(𝔽(_captureIndex_))).
                        let key = agent.key_from_index(capture_index);
                        let next_capture = ops::get(agent, match_result, key)?;
                        // b. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _nextCapture_).
                        let key = agent.key_from_index(length_a);
                        ops::create_data_property_or_throw(agent, array, key, next_capture)?;
                        // c. Set _captureIndex_ to _captureIndex_ + 1.
                        capture_index += 1.0;
                        // d. Set _lengthA_ to _lengthA_ + 1.
                        length_a += 1.0;
                        // e. If _lengthA_ = _lim_, return _array_.
                        if length_a == lim {
                            return Ok(Value::Object(array));
                        }
                    }
                    // 10. Set _searchIndex_ to _lastMatchEnd_.
                    search_index = last_match_end;
                }
            }
            // The step above only admits null or an Object; RegExpExec guarantees that.
            _ => unreachable!("RegExpExec returns an Object or null"),
        }
    }
    // 20. Let _substring_ be the substring of _string_ from _lastMatchEnd_ to _size_.
    let substring = substring_units(agent.str(string), last_match_end, size);
    let substring = Value::String(agent.intern(&substring));
    // 21. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _substring_).
    let key = agent.key_from_index(length_a);
    ops::create_data_property_or_throw(agent, array, key, substring)?;
    // 22. Return _array_.
    Ok(Value::Object(array))
}

/// The code units of `units` from index `from` up to index `to`, clamped to the string's length.
fn substring_units(units: &[u16], from: f64, to: f64) -> Vec<u16> {
    let end = (to as usize).min(units.len());
    let start = (from as usize).min(end);
    units[start..end].to_vec()
}
