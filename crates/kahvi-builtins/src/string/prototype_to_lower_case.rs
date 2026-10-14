//! String.prototype.toLowerCase ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.tolowercase

use kahvi_core::prelude::*;
use kahvi_unicode::{binary_property, in_ranges, to_lowercase};

/// String.prototype.toLowerCase
pub(crate) fn prototype_to_lower_case(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
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
    // 5. Let _lowerText_ be toLowercase(_sText_), according to the Unicode Default Case Conversion algorithm.
    let lower_text = to_lowercase_code_points(agent, &s_text)?;
    // 6. Let _lowercaseString_ be CodePointsToString(_lowerText_).
    let lowercase_string = code_points_to_string(&lower_text);
    // 7. Return _lowercaseString_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (args, new_target);
    Ok(Value::String(agent.intern(&lowercase_string)))
}

/// toLowercase ( _sText_ ): the full lowercase mapping of each code point,
/// with the context-sensitive Final_Sigma rule of the Unicode Default Case
/// Conversion algorithm (Table 3-17, R3).
fn to_lowercase_code_points(agent: &mut Agent, s_text: &[u32]) -> JsResult<Vec<u32>> {
    let Some(cased) = binary_property("Cased") else {
        return Err(agent.not_implemented("the Cased property for String.prototype.toLowerCase"));
    };
    let Some(case_ignorable) = binary_property("Case_Ignorable") else {
        return Err(
            agent.not_implemented("the Case_Ignorable property for String.prototype.toLowerCase")
        );
    };
    let mut lower_text = Vec::with_capacity(s_text.len());
    for (i, &c) in s_text.iter().enumerate() {
        // Final_Sigma is the only context-sensitive mapping that applies here.
        if c == 0x03A3 && is_final_sigma(s_text, i, cased, case_ignorable) {
            lower_text.push(0x03C2);
            continue;
        }
        match to_lowercase(c) {
            Some(mapping) => lower_text.extend_from_slice(mapping),
            None => lower_text.push(c),
        }
    }
    Ok(lower_text)
}

/// The Final_Sigma condition: the code point at _i_ is preceded by a cased
/// letter followed by zero or more case-ignorable characters, and is not
/// followed by zero or more case-ignorable characters followed by a cased letter.
fn is_final_sigma(
    s_text: &[u32],
    i: usize,
    cased: &[(u32, u32)],
    case_ignorable: &[(u32, u32)],
) -> bool {
    let before = s_text
        .iter()
        .take(i)
        .rev()
        .find(|&&c| !in_ranges(case_ignorable, c));
    if !before.is_some_and(|&c| in_ranges(cased, c)) {
        return false;
    }
    let after = s_text
        .iter()
        .skip(i + 1)
        .find(|&&c| !in_ranges(case_ignorable, c));
    !after.is_some_and(|&c| in_ranges(cased, c))
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
