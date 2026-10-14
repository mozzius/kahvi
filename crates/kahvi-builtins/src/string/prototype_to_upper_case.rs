//! String.prototype.toUpperCase ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.touppercase

use kahvi_core::prelude::*;
use kahvi_unicode::to_uppercase;

/// String.prototype.toUpperCase
pub(crate) fn prototype_to_upper_case(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
    // It behaves in exactly the same way as `String.prototype.toLowerCase`, except that the String is mapped using the toUppercase algorithm of the Unicode Default Case Conversion.
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 4. Let _sText_ be StringToCodePoints(_string_).
    let s_text = string_to_code_points(agent.str(string));
    // 5. Let _upperText_ be toUppercase(_sText_), according to the Unicode Default Case Conversion algorithm.
    let upper_text = to_uppercase_code_points(&s_text);
    // 6. Let _uppercaseString_ be CodePointsToString(_upperText_).
    let uppercase_string = code_points_to_string(&upper_text);
    // 7. Return _uppercaseString_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (args, new_target);
    Ok(Value::String(agent.intern(&uppercase_string)))
}

/// toUppercase ( _sText_ ): the full uppercase mapping of each code point.
/// Unlike toLowercase there is no context-sensitive rule for uppercase in the
/// unconditional mappings, so each code point is mapped independently.
fn to_uppercase_code_points(s_text: &[u32]) -> Vec<u32> {
    let mut upper_text = Vec::with_capacity(s_text.len());
    for &c in s_text {
        match to_uppercase(c) {
            Some(mapping) => upper_text.extend_from_slice(mapping),
            None => upper_text.push(c),
        }
    }
    upper_text
}

/// StringToCodePoints ( _string_ ): a lone surrogate stands for itself.
fn string_to_code_points(units: &[u16]) -> Vec<u32> {
    char::decode_utf16(units.iter().copied())
        .map(|r| match r {
            Ok(c) => c as u32,
            Err(e) => u32::from(e.unpaired_surrogate()),
        })
        .collect()
}

/// CodePointsToString ( _text_ ): UTF16EncodeCodePoint over each code point.
fn code_points_to_string(code_points: &[u32]) -> Vec<u16> {
    let mut out = Vec::with_capacity(code_points.len());
    for &cp in code_points {
        match char::from_u32(cp) {
            Some(c) => {
                let mut buf = [0u16; 2];
                out.extend_from_slice(c.encode_utf16(&mut buf));
            }
            // A lone surrogate.
            None => out.push(cp as u16),
        }
    }
    out
}
