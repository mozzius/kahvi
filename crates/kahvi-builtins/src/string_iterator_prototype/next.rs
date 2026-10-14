//! %StringIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%stringiteratorprototype%.next

use crate::support::string_iterator::StringIterator;
use kahvi_core::prelude::*;

/// StringIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(*this* value, ~empty~, *"%StringIteratorPrototype%"*).
    // GeneratorValidate: the receiver must be a String Iterator.
    let it = match this {
        Value::Object(o) if agent.native::<StringIterator>(o).is_some() => o,
        _ => return Err(agent.throw_type_error("not a String Iterator")),
    };
    // GeneratorResume step 2: a completed generator returns { value: undefined, done: true }.
    let (string, position) = match agent.native::<StringIterator>(it) {
        Some(StringIterator {
            string: Some(string),
            position,
        }) => (*string, *position),
        _ => {
            return Ok(Value::Object(ops::create_iter_result_object(
                agent,
                Value::Undefined,
                true,
            )));
        }
    };
    // Closure step 4.a. Let _length_ be the length of _string_.
    let length = agent.str_len(string);
    // Closure step 4.c. Repeat, while _position_ < _length_,
    if position >= length {
        // Closure step 4.d. Return NormalCompletion(~unused~).
        // The generator completes; later calls take the completed path above.
        agent
            .native_mut::<StringIterator>(it)
            .expect("a String Iterator")
            .string = None;
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    }
    // Closure step 4.c.i. Let _codePoint_ be CodePointAt(_string_, _position_).
    let code_unit_count = code_point_unit_count(agent.str(string), position);
    // Closure step 4.c.ii. Let _nextIndex_ be _position_ + _codePoint_.[[CodeUnitCount]].
    let next_index = position + code_unit_count;
    // Closure step 4.c.iii. Let _resultString_ be the substring of _string_ from _position_ to _nextIndex_.
    let result_string = agent.substring(string, position, next_index);
    // Closure step 4.c.iv. Set _position_ to _nextIndex_.
    agent
        .native_mut::<StringIterator>(it)
        .expect("a String Iterator")
        .position = next_index;
    // Closure step 4.c.v. Perform ? GeneratorYield(CreateIteratorResultObject(_resultString_, *false*)).
    Ok(Value::Object(ops::create_iter_result_object(
        agent,
        Value::String(result_string),
        false,
    )))
}

/// The [[CodeUnitCount]] of CodePointAt(_string_, _position_) (§6.1.5.1), for a
/// position below the length of `units`. A valid surrogate pair counts as 2 code
/// units; any other code unit, including a lone surrogate, counts as 1.
/// Local to this file: no shared CodePointAt helper exists yet.
fn code_point_unit_count(units: &[u16], position: usize) -> usize {
    // 1. Let _size_ be the length of _string_.
    let size = units.len();
    // 2. Let _first_ be the code unit at index _position_ within _string_.
    let first = units[position];
    // 3. If _first_ is not a leading surrogate or trailing surrogate, then
    //    a. Return the Record { [[CodePoint]]: ℝ(_first_), [[CodeUnitCount]]: 1, [[IsUnpairedSurrogate]]: false }.
    // 4. If _first_ is a trailing surrogate or _position_ + 1 = _size_, then
    //    a. Return the Record { [[CodePoint]]: ℝ(_first_), [[CodeUnitCount]]: 1, [[IsUnpairedSurrogate]]: true }.
    if !(0xD800..=0xDBFF).contains(&first) || position + 1 == size {
        return 1;
    }
    // 5. Let _second_ be the code unit at index _position_ + 1 within _string_.
    let second = units[position + 1];
    // 6. If _second_ is not a trailing surrogate, then
    //    a. Return the Record { [[CodePoint]]: ℝ(_first_), [[CodeUnitCount]]: 1, [[IsUnpairedSurrogate]]: true }.
    if !(0xDC00..=0xDFFF).contains(&second) {
        return 1;
    }
    // 7. Set _cp_ to UTF16SurrogatePairToCodePoint(_first_, _second_).
    // 8. Return the Record { [[CodePoint]]: _cp_, [[CodeUnitCount]]: 2, [[IsUnpairedSurrogate]]: false }.
    2
}
