//! String.prototype.padStart ( _maxLength_ [ , _fillString_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.padstart

use kahvi_core::prelude::*;

/// The longest String this engine will build, in code units. The spec leaves
/// the limit to the implementation; a longer result throws a RangeError
/// instead of exhausting memory.
const MAX_STRING_LENGTH: f64 = ((1u64 << 29) - 24) as f64;

/// String.prototype.padStart
pub(crate) fn prototype_pad_start(
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
    // 3. Return ? StringPaddingBuiltinsImpl(_thisValue_, _maxLength_, _fillString_, ~start~).
    let max_length = args.get_or_undefined(0);
    let fill_string = args.get_or_undefined(1);
    string_padding_builtins_impl(agent, this_value, max_length, fill_string, true)
}

/// StringPaddingBuiltinsImpl ( _thisValue_, _maxLength_, _fillString_, _placement_ )
///
/// `at_start` is true for the ~start~ placement and false for ~end~.
fn string_padding_builtins_impl(
    agent: &mut Agent,
    this_value: Value,
    max_length: Value,
    fill_string: Value,
    at_start: bool,
) -> JsResult<Value> {
    // 1. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // The conversions below may run user code and collect garbage: _string_ and
    // _fillString_ stay rooted.
    agent.with_roots(&[Value::String(string), fill_string], |agent| {
        // 2. Let _intMaxLength_ be ℝ(? ToLength(_maxLength_)).
        let int_max_length = ops::to_length(agent, max_length)?;
        // 3. Let _stringLength_ be the length of _string_.
        let string_length = agent.str_len(string) as f64;
        // 4. If _intMaxLength_ ≤ _stringLength_, return _string_.
        if int_max_length <= string_length {
            return Ok(Value::String(string));
        }
        // 5. If _fillString_ is *undefined*, set _fillString_ to the String value consisting solely of the code unit 0x0020 (SPACE).
        // 6. Else, set _fillString_ to ? ToString(_fillString_).
        let fill = if matches!(fill_string, Value::Undefined) {
            agent.intern(&[0x0020])
        } else {
            ops::to_string(agent, fill_string)?
        };
        // 7. Return StringPad(_string_, _intMaxLength_, _fillString_, _placement_).
        string_pad(agent, string, int_max_length, fill, at_start)
    })
}

/// StringPad ( _string_, _maxLength_, _fillString_, _placement_ )
fn string_pad(
    agent: &mut Agent,
    string: JsString,
    max_length: f64,
    fill_string: JsString,
    at_start: bool,
) -> JsResult<Value> {
    // 1. Let _len_ be the length of _string_.
    let len = agent.str_len(string) as f64;
    // 2. If _maxLength_ ≤ _len_, return _string_.
    if max_length <= len {
        return Ok(Value::String(string));
    }
    // 3. If _fillString_ is the empty String, return _string_.
    let fill_units = agent.str(fill_string).to_vec();
    if fill_units.is_empty() {
        return Ok(Value::String(string));
    }
    // Engine limit: refuse results too long to represent, rather than abort.
    if max_length > MAX_STRING_LENGTH {
        return Err(agent.throw_range_error("Invalid string length"));
    }
    // 4. Let _fillLen_ be _maxLength_ - _len_.
    let fill_len = (max_length - len) as usize;
    // 5. Let _truncatedStringFiller_ be a String made of repeated copies of
    //    _fillString_, truncated to length _fillLen_.
    let filler: Vec<u16> = fill_units.iter().copied().cycle().take(fill_len).collect();
    // 6. If _placement_ is ~start~, return the string-concatenation of
    //    _truncatedStringFiller_ and _string_.
    // 7. Return the string-concatenation of _string_ and _truncatedStringFiller_.
    let string_units = agent.str(string).to_vec();
    let result: Vec<u16> = if at_start {
        filler.into_iter().chain(string_units).collect()
    } else {
        string_units.into_iter().chain(filler).collect()
    };
    Ok(Value::String(agent.intern(&result)))
}
