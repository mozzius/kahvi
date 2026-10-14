//! String.prototype.replace ( _searchValue_, _replaceValue_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.replace

use kahvi_core::prelude::*;

/// String.prototype.replace
pub(crate) fn prototype_replace(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    let search_value = args.get_or_undefined(0);
    let replace_value = args.get_or_undefined(1);
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. If _searchValue_ is an Object, then
    if let Value::Object(_) = search_value {
        // a. Let _replacer_ be ? GetMethod(_searchValue_, %Symbol.replace%).
        let replace_key = PropertyKey::Symbol(agent.symbols_wk.replace);
        let replacer = ops::get_method(agent, search_value, replace_key)?;
        // b. If _replacer_ is not *undefined*, then
        if !matches!(replacer, Value::Undefined) {
            // i. Return ? Call(_replacer_, _searchValue_, « _thisValue_, _replaceValue_ »).
            return ops::call(agent, replacer, search_value, &[this_value, replace_value]);
        }
    }
    // 4. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 5. Let _searchString_ be ? ToString(_searchValue_).
    let search_string = ops::to_string(agent, search_value)?;
    // 6. Let _functionalReplace_ be IsCallable(_replaceValue_).
    let functional_replace = ops::is_callable(agent, replace_value);
    // 7. If _functionalReplace_ is *false*, then
    //    a. Set _replaceValue_ to ? ToString(_replaceValue_).
    let replace_template = if functional_replace {
        None
    } else {
        Some(ops::to_string(agent, replace_value)?)
    };
    let string_units = agent.str(string).to_vec();
    let search_units = agent.str(search_string).to_vec();
    // 8. Let _searchLength_ be the length of _searchString_.
    let search_length = search_units.len();
    // 9. Let _position_ be StringIndexOf(_string_, _searchString_, 0).
    let position = string_index_of(&string_units, &search_units, 0);
    // 10. If _position_ is ~not-found~, return _string_.
    let Some(position) = position else {
        return Ok(Value::String(string));
    };
    // 11. Let _preceding_ be the substring of _string_ from 0 to _position_.
    let preceding = &string_units[..position];
    // 12. Let _following_ be the substring of _string_ from _position_ + _searchLength_.
    let following = &string_units[position + search_length..];
    // 13. If _functionalReplace_ is *true*, then
    let replacement: Vec<u16> = match replace_template {
        None => {
            // a. Let _replacement_ be ? ToString(? Call(_replaceValue_, *undefined*, « _searchString_, 𝔽(_position_), _string_ »)).
            let replaced = ops::call(
                agent,
                replace_value,
                Value::Undefined,
                &[
                    Value::String(search_string),
                    Value::Number(position as f64),
                    Value::String(string),
                ],
            )?;
            let replaced = ops::to_string(agent, replaced)?;
            agent.str(replaced).to_vec()
        }
        // 14. Else,
        Some(template) => {
            // a. Assert: _replaceValue_ is a String.
            // b. Let _captures_ be a new empty List.
            // c. Let _replacement_ be ! GetSubstitution(_searchString_, _string_, _position_, _captures_, *undefined*, _replaceValue_).
            get_substitution(&search_units, &string_units, position, agent.str(template))
        }
    };
    // 15. Return the string-concatenation of _preceding_, _replacement_, and _following_.
    let mut result: Vec<u16> =
        Vec::with_capacity(preceding.len() + replacement.len() + following.len());
    result.extend_from_slice(preceding);
    result.extend_from_slice(&replacement);
    result.extend_from_slice(following);
    Ok(Value::String(agent.intern(&result)))
}

/// StringIndexOf(_string_, _searchValue_, _fromIndex_)
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

/// GetSubstitution(_matched_, _string_, _position_, _captures_, *undefined*, _replacementTemplate_)
///
/// Called with an empty _captures_ and no _namedCaptures_, so the `$n` and `$<`
/// forms have nothing to refer to and stay literal text (steps 5.f to 5.h).
/// Infallible, as the caller's `!` requires.
fn get_substitution(
    matched: &[u16],
    string: &[u16],
    position: usize,
    replacement_template: &[u16],
) -> Vec<u16> {
    // 1. Let _stringLength_ be the length of _string_.
    let string_length = string.len();
    // 2. Assert: _position_ ≤ _stringLength_.
    // 3. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::new();
    // 4. Let _templateRemainder_ be _replacementTemplate_.
    let mut i = 0;
    let dollar = u16::from(b'$');
    // 5. Repeat, while _templateRemainder_ is not the empty String,
    while i < replacement_template.len() {
        let rest = &replacement_template[i..];
        // a. NOTE: The following steps isolate _ref_ (a prefix of _templateRemainder_), determine _refReplacement_ (its replacement), and then append that replacement to _result_.
        if rest.starts_with(&[dollar, u16::from(b'$')]) {
            // b. If _templateRemainder_ starts with *"$$"*, then
            //    i. Let _ref_ be *"$$"*.
            //    ii. Let _refReplacement_ be *"$"*.
            result.push(dollar);
            i += 2;
        } else if rest.starts_with(&[dollar, u16::from(b'`')]) {
            // c. Else if _templateRemainder_ starts with *"$`"*, then
            //    i. Let _ref_ be *"$`"*.
            //    ii. Let _refReplacement_ be the substring of _string_ from 0 to _position_.
            result.extend_from_slice(&string[..position]);
            i += 2;
        } else if rest.starts_with(&[dollar, u16::from(b'&')]) {
            // d. Else if _templateRemainder_ starts with *"$&"*, then
            //    i. Let _ref_ be *"$&"*.
            //    ii. Let _refReplacement_ be _matched_.
            result.extend_from_slice(matched);
            i += 2;
        } else if rest.starts_with(&[dollar, u16::from(b'\'')]) {
            // e. Else if _templateRemainder_ starts with *"$'"*, then
            //    i. Let _ref_ be *"$'"*.
            //    ii. Let _matchLength_ be the length of _matched_.
            //    iii. Let _tailPosition_ be _position_ + _matchLength_.
            let tail_position = position + matched.len();
            //    iv. Let _refReplacement_ be the substring of _string_ from min(_tailPosition_, _stringLength_).
            result.extend_from_slice(&string[tail_position.min(string_length)..]);
            i += 2;
        } else {
            // f. to l. With no captures and no named captures, _ref_ is a single "$" that
            //    is kept as literal text, and the remaining code units are copied unchanged.
            result.push(rest[0]);
            i += 1;
        }
    }
    // 6. Return _result_.
    result
}
