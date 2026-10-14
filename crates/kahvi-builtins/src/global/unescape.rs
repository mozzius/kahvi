//! unescape ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-unescape-string

use kahvi_core::prelude::*;

/// unescape
pub(crate) fn unescape(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function is a property of the global object. It computes a new version of a String value in which each escape sequence of the sort that might be introduced by the `escape` function is replaced with the code unit that it represents.
    // It is the %unescape% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Set _string_ to ? ToString(_string_).
    let string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 2. Let _length_ be the length of _string_.
    let units: Vec<u16> = agent.str(string).to_vec();
    let length = units.len();
    // 3. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::with_capacity(length);
    // 4. Let _k_ be 0.
    let mut k = 0;
    // 5. Repeat, while _k_ < _length_,
    while k < length {
        // a. Let _codeUnit_ be the code unit at index _k_ within _string_.
        let mut code_unit = units[k];
        // b. If _codeUnit_ is the code unit 0x0025 (PERCENT SIGN), then
        if code_unit == 0x0025 {
            // i. Let _hexDigits_ be the empty String.
            let mut hex_digits: &[u16] = &[];
            // ii. Let _optionalAdvance_ be 0.
            let mut optional_advance = 0;
            // iii. If _k_ + 5 < _length_ and the code unit at index _k_ + 1 within _string_ is the code unit 0x0075 (LATIN SMALL LETTER U), then
            if k + 5 < length && units[k + 1] == 0x0075 {
                // 1. Set _hexDigits_ to the substring of _string_ from _k_ + 2 to _k_ + 6.
                hex_digits = &units[k + 2..k + 6];
                // 2. Set _optionalAdvance_ to 5.
                optional_advance = 5;
            }
            // iv. Else if _k_ + 3 ≤ _length_, then
            else if k + 3 <= length {
                // 1. Set _hexDigits_ to the substring of _string_ from _k_ + 1 to _k_ + 3.
                hex_digits = &units[k + 1..k + 3];
                // 2. Set _optionalAdvance_ to 2.
                optional_advance = 2;
            }
            // v. Let _parseResult_ be ParseText(_hexDigits_, |HexDigits[~Sep]|).
            // vi. If _parseResult_ is a Parse Node, then
            if let Some(code_unit_number) = parse_hex_digits(hex_digits) {
                // 1. Let _codeUnitNumber_ be the MV of _parseResult_.
                // 2. Set _codeUnit_ to the code unit whose numeric value is _codeUnitNumber_.
                code_unit = code_unit_number;
                // 3. Set _k_ to _k_ + _optionalAdvance_.
                k += optional_advance;
            }
        }
        // c. Set _result_ to the string-concatenation of _result_ and _codeUnit_.
        result.push(code_unit);
        // d. Set _k_ to _k_ + 1.
        k += 1;
    }
    // 6. Return _result_.
    Ok(Value::String(agent.intern(&result)))
}

/// Parses `digits` as a non-empty HexDigits production and returns its MV,
/// or `None` when some code unit is not a hexadecimal digit.
fn parse_hex_digits(digits: &[u16]) -> Option<u16> {
    if digits.is_empty() {
        return None;
    }
    let mut value: u16 = 0;
    for &unit in digits {
        let digit = char::from_u32(u32::from(unit))?.to_digit(16)?;
        value = value * 16 + digit as u16;
    }
    Some(value)
}
