//! RegExp.prototype [ %Symbol.replace% ] ( _string_, _replaceValue_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype-%symbol.replace%

use kahvi_core::prelude::*;

use crate::support::regexp::{advance_string_index, regexp_exec};

/// RegExp.prototype[Symbol.replace]
pub(crate) fn prototype_symbol_replace(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    let Value::Object(regexp) = this else {
        return Err(
            agent.throw_type_error("RegExp.prototype[Symbol.replace] called on a non-object")
        );
    };
    // 3. Set _string_ to ? ToString(_string_).
    let string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 4. Let _stringLength_ be the length of _string_.
    let string_length = agent.str_len(string) as f64;
    // 5. Let _functionalReplace_ be IsCallable(_replaceValue_).
    let replace_value = args.get_or_undefined(1);
    let functional_replace = ops::is_callable(agent, replace_value);
    // 6. If _functionalReplace_ is *false*, then
    let replace_value = if functional_replace {
        replace_value
    } else {
        // a. Set _replaceValue_ to ? ToString(_replaceValue_).
        Value::String(ops::to_string(agent, replace_value)?)
    };
    // 7. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
    let key = agent.key("flags");
    let flags_value = ops::get(agent, regexp, key)?;
    let flags = ops::to_string(agent, flags_value)?;
    let flags_units = agent.str(flags).to_vec();
    // 8. If _flags_ contains *"g"*, let _global_ be *true*; else let _global_ be *false*.
    let global = flags_units.contains(&u16::from(b'g'));
    // 12.c.iii.2.b. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*; else let _fullUnicode_ be *false*.
    let full_unicode =
        flags_units.contains(&u16::from(b'u')) || flags_units.contains(&u16::from(b'v'));
    // 9. If _global_ is *true*, then
    if global {
        // a. Perform ? Set(_regexp_, *"lastIndex"*, *+0*_𝔽, *true*).
        let key = agent.key("lastIndex");
        ops::set(agent, regexp, key, Value::Number(0.0), true)?;
    }
    // 10. Let _results_ be a new empty List.
    let mut results: Vec<Object> = Vec::new();
    // 11. Let _done_ be *false*.
    // 12. Repeat, while _done_ is *false*,
    loop {
        // a. Let _result_ be ? RegExpExec(_regexp_, _string_).
        let result = regexp_exec(agent, regexp, string)?;
        // b. If _result_ is *null*, then
        //    i. Set _done_ to *true*.
        let Value::Object(result) = result else {
            break;
        };
        // c. Else,
        //    i. Append _result_ to _results_.
        results.push(result);
        // ii. If _global_ is *false*, then
        //     1. Set _done_ to *true*.
        if !global {
            break;
        }
        // iii. Else,
        //      1. Let _matchString_ be ? ToString(? Get(_result_, *"0"*)).
        let key = agent.key_from_index(0.0);
        let match_value = ops::get(agent, result, key)?;
        let match_string = ops::to_string(agent, match_value)?;
        //      2. If _matchString_ is the empty String, then
        if agent.str_len(match_string) == 0 {
            // a. Let _thisIndex_ be ℝ(? ToLength(? Get(_regexp_, *"lastIndex"*))).
            let key = agent.key("lastIndex");
            let last_index = ops::get(agent, regexp, key)?;
            let this_index = ops::to_length(agent, last_index)?;
            // b. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*; else let _fullUnicode_ be *false*.
            //    (Computed above.)
            // c. Let _nextIndex_ be AdvanceStringIndex(_string_, _thisIndex_, _fullUnicode_).
            let next_index = advance_string_index(agent.str(string), this_index, full_unicode);
            // d. Perform ? Set(_regexp_, *"lastIndex"*, 𝔽(_nextIndex_), *true*).
            let key = agent.key("lastIndex");
            ops::set(agent, regexp, key, Value::Number(next_index), true)?;
        }
    }
    // 13. Let _accumulatedResult_ be the empty String.
    let mut accumulated: Vec<u16> = Vec::new();
    // 14. Let _nextSourcePosition_ be 0.
    let mut next_source_position = 0.0;
    let string_units = agent.str(string).to_vec();
    // 15. For each element _result_ of _results_, do
    for result in results {
        // a. Let _resultLength_ be ? LengthOfArrayLike(_result_).
        let result_length = ops::length_of_array_like(agent, result)?;
        // b. Let _capturesCount_ be max(_resultLength_ - 1, 0).
        let captures_count = (result_length - 1.0).max(0.0);
        // c. Let _matched_ be ? ToString(? Get(_result_, *"0"*)).
        let key = agent.key_from_index(0.0);
        let matched_value = ops::get(agent, result, key)?;
        let matched = ops::to_string(agent, matched_value)?;
        // d. Let _matchLength_ be the length of _matched_.
        let match_length = agent.str_len(matched) as f64;
        // e. Let _position_ be ? ToIntegerOrInfinity(? Get(_result_, *"index"*)).
        let key = agent.key("index");
        let index_value = ops::get(agent, result, key)?;
        let mut position = ops::to_integer_or_infinity(agent, index_value)?;
        // f. Set _position_ to the result of clamping _position_ between 0 and _stringLength_.
        if position < 0.0 {
            position = 0.0;
        } else if position > string_length {
            position = string_length;
        }
        // g. Let _captures_ be a new empty List.
        let mut captures: Vec<Value> = Vec::new();
        // h. Let _captureNumber_ be 1.
        let mut capture_number = 1.0;
        // i. Repeat, while _captureNumber_ ≤ _capturesCount_,
        while capture_number <= captures_count {
            // i. Let _capture_ be ? Get(_result_, ! ToString(𝔽(_captureNumber_))).
            let key = agent.key_from_index(capture_number);
            let mut capture = ops::get(agent, result, key)?;
            // ii. If _capture_ is not *undefined*, then
            if !capture.is_undefined() {
                // 1. Set _capture_ to ? ToString(_capture_).
                capture = Value::String(ops::to_string(agent, capture)?);
            }
            // iii. Append _capture_ to _captures_.
            captures.push(capture);
            // iv. NOTE: When _captureNumber_ = 1, the preceding step puts the first element into _captures_ (at index 0). More generally, the _captureNumber_^th capture (the characters captured by the _captureNumber_^th set of capturing parentheses) is at _captures_[_captureNumber_ - 1].
            // v. Set _captureNumber_ to _captureNumber_ + 1.
            capture_number += 1.0;
        }
        // j. Let _namedCaptures_ be ? Get(_result_, *"groups"*).
        let key = agent.key("groups");
        let named_captures = ops::get(agent, result, key)?;
        let replacement_string = if functional_replace {
            // k. If _functionalReplace_ is *true*, then
            //    i. Let _replacerArgs_ be the list-concatenation of « _matched_ », _captures_, and « 𝔽(_position_), _string_ ».
            let mut replacer_args = vec![Value::String(matched)];
            replacer_args.extend(captures);
            replacer_args.push(Value::Number(position));
            replacer_args.push(Value::String(string));
            //    ii. If _namedCaptures_ is not *undefined*, then
            if !named_captures.is_undefined() {
                //        1. Append _namedCaptures_ to _replacerArgs_.
                replacer_args.push(named_captures);
            }
            //    iii. Let _replacementValue_ be ? Call(_replaceValue_, *undefined*, _replacerArgs_).
            let replacement_value =
                ops::call(agent, replace_value, Value::Undefined, &replacer_args)?;
            //    iv. Let _replacementString_ be ? ToString(_replacementValue_).
            ops::to_string(agent, replacement_value)?
        } else {
            // l. Else,
            //    i. If _namedCaptures_ is not *undefined*, then
            //       1. Set _namedCaptures_ to ? ToObject(_namedCaptures_).
            let named_captures = match named_captures {
                Value::Undefined => None,
                v => Some(ops::to_object(agent, v)?),
            };
            let Value::String(replace_template) = replace_value else {
                unreachable!("replaceValue is converted to a String when not functional")
            };
            //    ii. Let _replacementString_ be ? GetSubstitution(_matched_, _string_, _position_, _captures_, _namedCaptures_, _replaceValue_).
            get_substitution(
                agent,
                matched,
                string,
                position as usize,
                &captures,
                named_captures,
                replace_template,
            )?
        };
        // m. If _position_ ≥ _nextSourcePosition_, then
        if position >= next_source_position {
            // i. NOTE: _position_ should not normally move backwards. If it does, it is an indication of an ill-behaving RegExp subclass or use of an access triggered side-effect to change the global flag or other characteristics of _regexp_. In such cases, the corresponding substitution is ignored.
            // ii. Set _accumulatedResult_ to the string-concatenation of _accumulatedResult_, the substring of _string_ from _nextSourcePosition_ to _position_, and _replacementString_.
            accumulated
                .extend_from_slice(&string_units[next_source_position as usize..position as usize]);
            accumulated.extend_from_slice(agent.str(replacement_string));
            // iii. Set _nextSourcePosition_ to _position_ + _matchLength_.
            next_source_position = position + match_length;
        }
    }
    // 16. If _nextSourcePosition_ ≥ _stringLength_, return _accumulatedResult_.
    if next_source_position >= string_length {
        return Ok(Value::String(agent.intern(&accumulated)));
    }
    // 17. Return the string-concatenation of _accumulatedResult_ and the substring of _string_ from _nextSourcePosition_.
    accumulated.extend_from_slice(&string_units[next_source_position as usize..]);
    Ok(Value::String(agent.intern(&accumulated)))
}

/// Whether the code unit is a decimal digit (0x0030 to 0x0039).
fn is_decimal_digit(unit: u16) -> bool {
    (u16::from(b'0')..=u16::from(b'9')).contains(&unit)
}

/// The value of a decimal digit code unit.
fn digit_value(unit: u16) -> usize {
    usize::from(unit).saturating_sub(usize::from(b'0'))
}

/// GetSubstitution(_matched_, _string_, _position_, _captures_, _namedCaptures_, _replacementTemplate_)
///
/// Local to this file until another built-in needs it; `String.prototype.replace`
/// and `replaceAll` will want the same operation.
fn get_substitution(
    agent: &mut Agent,
    matched: JsString,
    string: JsString,
    position: usize,
    captures: &[Value],
    named_captures: Option<Object>,
    replacement_template: JsString,
) -> JsResult<JsString> {
    // 1. Let _stringLength_ be the length of _string_.
    let string_units = agent.str(string).to_vec();
    let string_length = string_units.len();
    // 2. Assert: _position_ ≤ _stringLength_.
    // 3. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::new();
    // 4. Let _templateRemainder_ be _replacementTemplate_.
    let template = agent.str(replacement_template).to_vec();
    let dollar = u16::from(b'$');
    let mut i = 0;
    // 5. Repeat, while _templateRemainder_ is not the empty String,
    while i < template.len() {
        let rest = &template[i..];
        // a. NOTE: The following steps isolate _ref_ (a prefix of _templateRemainder_), determine _refReplacement_ (its replacement), and then append that replacement to _result_.
        let (ref_len, replacement): (usize, Vec<u16>) =
            if rest.starts_with(&[dollar, u16::from(b'$')]) {
                // b. If _templateRemainder_ starts with *"$$"*, then
                //    i. Let _ref_ be *"$$"*.
                //    ii. Let _refReplacement_ be *"$"*.
                (2, vec![dollar])
            } else if rest.starts_with(&[dollar, u16::from(b'`')]) {
                // c. Else if _templateRemainder_ starts with *"$`"*, then
                //    i. Let _ref_ be *"$`"*.
                //    ii. Let _refReplacement_ be the substring of _string_ from 0 to _position_.
                (2, string_units[..position].to_vec())
            } else if rest.starts_with(&[dollar, u16::from(b'&')]) {
                // d. Else if _templateRemainder_ starts with *"$&"*, then
                //    i. Let _ref_ be *"$&"*.
                //    ii. Let _refReplacement_ be _matched_.
                (2, agent.str(matched).to_vec())
            } else if rest.starts_with(&[dollar, u16::from(b'\'')]) {
                // e. Else if _templateRemainder_ starts with *"$'"* (0x0024 (DOLLAR SIGN) followed by 0x0027 (APOSTROPHE)), then
                //    i. Let _ref_ be *"$'"*.
                //    ii. Let _matchLength_ be the length of _matched_.
                let match_length = agent.str_len(matched);
                //    iii. Let _tailPosition_ be _position_ + _matchLength_.
                let tail_position = position + match_length;
                //    iv. Let _refReplacement_ be the substring of _string_ from min(_tailPosition_, _stringLength_).
                (2, string_units[tail_position.min(string_length)..].to_vec())
            } else if rest.len() >= 2 && rest[0] == dollar && is_decimal_digit(rest[1]) {
                // f. Else if _templateRemainder_ starts with *"$"* followed by 1 or more decimal digits, then
                //    i. If _templateRemainder_ starts with *"$"* followed by 2 or more decimal digits, let _digitCount_ be 2; else let _digitCount_ be 1.
                let mut digit_count = if rest.len() >= 3 && is_decimal_digit(rest[2]) {
                    2
                } else {
                    1
                };
                //    ii. Let _digits_ be the substring of _templateRemainder_ from 1 to 1 + _digitCount_.
                //    iii. Let _index_ be ℝ(StringToNumber(_digits_)).
                let mut index = digit_value(rest[1]);
                if digit_count == 2 {
                    index = index * 10 + digit_value(rest[2]);
                }
                //    iv. Assert: 0 ≤ _index_ ≤ 99.
                //    v. Let _captureLength_ be the number of elements in _captures_.
                let capture_length = captures.len();
                //    vi. If _index_ > _captureLength_ and _digitCount_ = 2, then
                if index > capture_length && digit_count == 2 {
                    //        1. NOTE: When a two-digit replacement pattern specifies an index exceeding the count of capturing groups, it is treated as a one-digit replacement pattern followed by a literal digit.
                    //        2. Set _digitCount_ to 1.
                    digit_count = 1;
                    //        3. Set _digits_ to the substring of _digits_ from 0 to 1.
                    //        4. Set _index_ to ℝ(StringToNumber(_digits_)).
                    index = digit_value(rest[1]);
                }
                //    vii. Let _ref_ be the substring of _templateRemainder_ from 0 to 1 + _digitCount_.
                let ref_len = 1 + digit_count;
                //    viii. If 1 ≤ _index_ ≤ _captureLength_, then
                let replacement = if (1..=capture_length).contains(&index) {
                    //        1. Let _capture_ be _captures_[_index_ - 1].
                    //        2. If _capture_ is *undefined*, then
                    //           a. Let _refReplacement_ be the empty String.
                    //        3. Else, let _refReplacement_ be _capture_.
                    match captures[index - 1] {
                        Value::String(capture) => agent.str(capture).to_vec(),
                        _ => Vec::new(),
                    }
                } else {
                    // ix. Else, let _refReplacement_ be _ref_.
                    rest[..ref_len].to_vec()
                };
                (ref_len, replacement)
            } else if rest.starts_with(&[dollar, u16::from(b'<')]) {
                // g. Else if _templateRemainder_ starts with *"$<"*, then
                //    i. Let _gtPosition_ be StringIndexOf(_templateRemainder_, *">"*, 0).
                let gt_position = rest.iter().position(|&u| u == u16::from(b'>'));
                match (gt_position, named_captures) {
                    //    ii. If _gtPosition_ is ~not-found~ or _namedCaptures_ is *undefined*, then
                    (Some(gt_position), Some(named_captures)) => {
                        //    iii. Else,
                        //         1. Let _ref_ be the substring of _templateRemainder_ from 0 to _gtPosition_ + 1.
                        let ref_len = gt_position + 1;
                        //         2. Let _groupName_ be the substring of _templateRemainder_ from 2 to _gtPosition_.
                        let group_name = agent.intern(&rest[2..gt_position]);
                        //         3. Assert: _namedCaptures_ is an Object.
                        //         4. Let _capture_ be ? Get(_namedCaptures_, _groupName_).
                        let key = ops::to_property_key(agent, Value::String(group_name))?;
                        let capture = ops::get(agent, named_captures, key)?;
                        //         5. If _capture_ is *undefined*, then
                        //            a. Let _refReplacement_ be the empty String.
                        //         6. Else, let _refReplacement_ be ? ToString(_capture_).
                        let replacement = if capture.is_undefined() {
                            Vec::new()
                        } else {
                            let capture = ops::to_string(agent, capture)?;
                            agent.str(capture).to_vec()
                        };
                        (ref_len, replacement)
                    }
                    _ => {
                        //        1. Let _ref_ be *"$<"*.
                        //        2. Let _refReplacement_ be _ref_.
                        (2, rest[..2].to_vec())
                    }
                }
            } else {
                // h. Else,
                //    i. Let _ref_ be the substring of _templateRemainder_ from 0 to 1.
                //    ii. Let _refReplacement_ be _ref_.
                (1, rest[..1].to_vec())
            };
        // i. Let _refLength_ be the length of _ref_.
        // j. Set _templateRemainder_ to the substring of _templateRemainder_ from _refLength_.
        i += ref_len;
        // k. Set _result_ to the string-concatenation of _result_ and _refReplacement_.
        result.extend_from_slice(&replacement);
    }
    // 6. Return _result_.
    Ok(agent.intern(&result))
}
