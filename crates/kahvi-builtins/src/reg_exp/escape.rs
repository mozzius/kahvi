//! RegExp.escape ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.escape

use kahvi_core::prelude::*;

/// RegExp.escape
pub(crate) fn escape(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns a copy of _string_ in which characters that are potentially special in a regular expression |Pattern| have been replaced by equivalent escape sequences.
    // It performs the following steps when called:
    //
    let _ = (this, new_target);
    // 1. If _string_ is not a String, throw a *TypeError* exception.
    let string = match args.get_or_undefined(0) {
        Value::String(s) => s.clone(),
        _ => {
            return Err(agent.throw_type_error("RegExp.escape: argument is not a string"));
        }
    };
    // 2. Let _escaped_ be the empty String.
    let mut escaped: Vec<u16> = Vec::new();
    // 3. Let _codePointList_ be StringToCodePoints(_string_).
    let code_point_list = string_to_code_points(agent.str(string));
    // 4. For each code point _codePoint_ of _codePointList_, do
    for code_point in code_point_list {
        // a. If _escaped_ is the empty String and _codePoint_ is matched by either |DecimalDigit| or |AsciiLetter|, then
        let is_ascii_alnum = matches!(
            code_point,
            0x30..=0x39 | 0x41..=0x5A | 0x61..=0x7A
        );
        if escaped.is_empty() && is_ascii_alnum {
            // i. NOTE: Escaping a leading digit ensures that output corresponds with pattern text which may be used after a `\0` character escape or a |DecimalEscape| such as `\1` and still match _string_ rather than be interpreted as an extension of the preceding escape sequence. Escaping a leading ASCII letter does the same for the context after `\c`.
            // ii. Let _numericValue_ be the numeric value of _codePoint_.
            let numeric_value = code_point;
            // iii. Let _hex_ be Number::toString(𝔽(_numericValue_), 16).
            let hex = format!("{numeric_value:x}");
            // iv. Assert: The length of _hex_ is 2.
            // v. Set _escaped_ to the string-concatenation of the code unit 0x005C (REVERSE SOLIDUS), *"x"*, and _hex_.
            escaped.push(0x005C);
            escaped.push(u16::from(b'x'));
            escaped.extend(hex.encode_utf16());
        }
        // b. Else,
        else {
            // i. Set _escaped_ to the string-concatenation of _escaped_ and EncodeForRegExpEscape(_codePoint_).
            escaped.extend(encode_for_reg_exp_escape(code_point));
        }
    }
    // 5. Return _escaped_.
    Ok(Value::String(agent.intern(&escaped)))
}

/// EncodeForRegExpEscape ( _codePoint_: a code point ): a String
///
/// Returns a String representing a |Pattern| for matching _codePoint_.
fn encode_for_reg_exp_escape(code_point: u32) -> Vec<u16> {
    // 1. If _codePoint_ is matched by |SyntaxCharacter| or _codePoint_ is U+002F (SOLIDUS), then
    let is_syntax_character = matches!(
        code_point,
        0x5E | 0x24
            | 0x5C
            | 0x2E
            | 0x2A
            | 0x2B
            | 0x3F
            | 0x28
            | 0x29
            | 0x5B
            | 0x5D
            | 0x7B
            | 0x7D
            | 0x7C
            | 0x2F
    );
    if is_syntax_character {
        // a. Return the string-concatenation of 0x005C (REVERSE SOLIDUS) and UTF16EncodeCodePoint(_codePoint_).
        let mut out = vec![0x005C];
        out.extend(utf16_encode_code_point(code_point));
        return out;
    }
    // 2. If _codePoint_ is a code point listed in the “Code Point” column of [table-controlescape-code-point-values], then
    let control_escape = match code_point {
        0x09 => Some(b't'),
        0x0A => Some(b'n'),
        0x0B => Some(b'v'),
        0x0C => Some(b'f'),
        0x0D => Some(b'r'),
        _ => None,
    };
    if let Some(letter) = control_escape {
        // a. Return the string-concatenation of 0x005C (REVERSE SOLIDUS) and the string in the “ControlEscape” column of the row whose “Code Point” column contains _codePoint_.
        return vec![0x005C, u16::from(letter)];
    }
    // 3. Let _otherPunctuators_ be the string-concatenation of *",-=<>#&!%:;@~'`"* and the code unit 0x0022 (QUOTATION MARK).
    const OTHER_PUNCTUATORS: &[u32] = &[
        0x2C, 0x2D, 0x3D, 0x3C, 0x3E, 0x23, 0x26, 0x21, 0x25, 0x3A, 0x3B, 0x40, 0x7E, 0x27, 0x60,
        0x22,
    ];
    // 4. Let _toEscape_ be StringToCodePoints(_otherPunctuators_).
    //    NOTE: OTHER_PUNCTUATORS is already the list of code points.
    // 5. If _toEscape_ contains _codePoint_, _codePoint_ is matched by either |WhiteSpace| or |LineTerminator|, or _codePoint_ has the same numeric value as a leading surrogate or trailing surrogate, then
    if OTHER_PUNCTUATORS.contains(&code_point)
        || is_white_space(code_point)
        || is_line_terminator(code_point)
        || (0xD800..=0xDFFF).contains(&code_point)
    {
        // a. Let _codePointNumber_ be the numeric value of _codePoint_.
        let code_point_number = code_point;
        // b. If _codePointNumber_ ≤ 0xFF, then
        if code_point_number <= 0xFF {
            // i. Let _hex_ be Number::toString(𝔽(_codePointNumber_), 16).
            let hex = format!("{code_point_number:x}");
            // ii. Return the string-concatenation of the code unit 0x005C (REVERSE SOLIDUS), *"x"*, and StringPad(_hex_, 2, *"0"*, ~start~).
            let padded = format!("{hex:0>2}");
            let mut out = vec![0x005C, u16::from(b'x')];
            out.extend(padded.encode_utf16());
            return out;
        }
        // c. Let _escaped_ be the empty String.
        let mut escaped: Vec<u16> = Vec::new();
        // d. Let _codeUnits_ be UTF16EncodeCodePoint(_codePoint_).
        let code_units = utf16_encode_code_point(code_point);
        // e. For each code unit _codeUnit_ of _codeUnits_, do
        for code_unit in code_units {
            // i. Set _escaped_ to the string-concatenation of _escaped_ and UnicodeEscape(_codeUnit_).
            escaped.extend(unicode_escape(code_unit));
        }
        // f. Return _escaped_.
        return escaped;
    }
    // 6. Return UTF16EncodeCodePoint(_codePoint_).
    utf16_encode_code_point(code_point)
}

/// UnicodeEscape ( _cu_ ): a String of the form `\uXXXX`.
fn unicode_escape(code_unit: u16) -> Vec<u16> {
    let hex = format!("{code_unit:04x}");
    let mut out = vec![0x005C, u16::from(b'u')];
    out.extend(hex.encode_utf16());
    out
}

/// WhiteSpace: TAB, VT, FF, SP, NBSP, ZWNBSP, and any code point of category Zs.
fn is_white_space(code_point: u32) -> bool {
    matches!(
        code_point,
        0x09 | 0x0B | 0x0C | 0x20 | 0xA0 | 0x1680 | 0x2000
            ..=0x200A | 0x202F | 0x205F | 0x3000 | 0xFEFF
    )
}

/// LineTerminator: LF, CR, LS, PS.
fn is_line_terminator(code_point: u32) -> bool {
    matches!(code_point, 0x0A | 0x0D | 0x2028 | 0x2029)
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

/// UTF16EncodeCodePoint ( _cp_ ): the UTF-16 code units of a code point.
fn utf16_encode_code_point(cp: u32) -> Vec<u16> {
    match char::from_u32(cp) {
        Some(c) => {
            let mut buf = [0u16; 2];
            c.encode_utf16(&mut buf).to_vec()
        }
        // A lone surrogate.
        None => vec![cp as u16],
    }
}
