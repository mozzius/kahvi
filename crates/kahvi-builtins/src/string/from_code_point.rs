//! String.fromCodePoint ( ..._codePoints_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.fromcodepoint

use kahvi_core::prelude::*;

/// String.fromCodePoint
pub(crate) fn from_code_point(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function may be called with any number of arguments which form the rest parameter _codePoints_.
    // It performs the following steps when called:
    //
    // 1. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::new();
    // 2. For each element _next_ of _codePoints_, do
    for next in args {
        // a. Let _nextCP_ be ? ToNumber(_next_).
        let next_cp = ops::to_number(agent, *next)?;
        // b. If _nextCP_ is not an integral Number, throw a *RangeError* exception.
        if !ops::is_integral_number(Value::Number(next_cp)) {
            return Err(
                agent.throw_range_error("String.fromCodePoint: argument is not an integral number")
            );
        }
        // c. If ℝ(_nextCP_) < 0 or ℝ(_nextCP_) > 0x10FFFF, throw a *RangeError* exception.
        if !(0.0..=f64::from(0x10FFFF_u32)).contains(&next_cp) {
            return Err(agent.throw_range_error("String.fromCodePoint: code point out of range"));
        }
        // d. Set _result_ to the string-concatenation of _result_ and UTF16EncodeCodePoint(ℝ(_nextCP_)).
        utf16_encode_code_point(&mut result, next_cp as u32);
    }
    // 3. Assert: If _codePoints_ is empty, then _result_ is the empty String.
    // 4. Return _result_.
    Ok(Value::String(agent.intern(&result)))
}

/// UTF16EncodeCodePoint ( _cp_ )
///
/// Appends the UTF-16 code units of the code point `cp` (0 to 0x10FFFF) to `out`.
fn utf16_encode_code_point(out: &mut Vec<u16>, cp: u32) {
    // 1. If 0 ≤ cp ≤ 0xFFFF, then
    if cp <= 0xFFFF {
        // a. Return the String value consisting of the code unit whose numeric value is cp.
        out.push(cp as u16);
        return;
    }
    // 2. Let cu1 be the code unit whose numeric value is ((cp - 0x10000) / 0x400) + 0xD800.
    let cu1 = ((cp - 0x10000) / 0x400 + 0xD800) as u16;
    // 3. Let cu2 be the code unit whose numeric value is ((cp - 0x10000) modulo 0x400) + 0xDC00.
    let cu2 = ((cp - 0x10000) % 0x400 + 0xDC00) as u16;
    // 4. Return the string-concatenation of cu1 and cu2.
    out.push(cu1);
    out.push(cu2);
}
