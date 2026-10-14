//! encodeURIComponent ( _uriComponent_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-encodeuricomponent-uricomponent

use kahvi_core::prelude::*;

/// encodeURIComponent
pub(crate) fn encode_uri_component(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function computes a new version of a UTF-16 encoded ([sec-ecmascript-language-types-string-type]) URI in which each instance of certain code points is replaced by one, two, three, or four escape sequences representing the UTF-8 encoding of the code point.
    // It is the %encodeURIComponent% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _componentString_ be ? ToString(_uriComponent_).
    let component_string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 2. Let _extraUnescaped_ be the empty String.
    let extra_unescaped: &[u16] = &[];
    // 3. Return ? Encode(_componentString_, _extraUnescaped_).
    let encoded = encode(agent, component_string, extra_unescaped)?;
    Ok(Value::String(encoded))
}

/// Encode ( _string_, _extraUnescaped_ )
///
/// The abstract operation is not in `kahvi_core::ops` yet; it lives here
/// until `encodeURI` needs it too.
fn encode(agent: &mut Agent, string: JsString, extra_unescaped: &[u16]) -> JsResult<JsString> {
    // 1. Let _length_ be the length of _string_.
    let units = agent.str(string).to_vec();
    let length = units.len();
    // 2. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::new();
    // 3. Let _alwaysUnescaped_ be the string-concatenation of the ASCII word characters and *"-.!~\*'()"*.
    // 4. Let _unescapedSet_ be the string-concatenation of _alwaysUnescaped_ and _extraUnescaped_.
    // 5. Let _k_ be 0.
    let mut k = 0;
    // 6. Repeat, while _k_ < _length_,
    while k < length {
        // a. Let _codeUnit_ be the code unit at index _k_ within _string_.
        let Some(&code_unit) = units.get(k) else {
            break;
        };
        // b. If _unescapedSet_ contains _codeUnit_, then
        if is_unescaped(code_unit, extra_unescaped) {
            // i. Set _k_ to _k_ + 1.
            k += 1;
            // ii. Set _result_ to the string-concatenation of _result_ and _codeUnit_.
            result.push(code_unit);
        } else {
            // c. Else,
            // i. Let _codePoint_ be CodePointAt(_string_, _k_).
            let (code_point, code_unit_count, is_unpaired_surrogate) = code_point_at(&units, k);
            // ii. If _codePoint_.[[IsUnpairedSurrogate]] is *true*, throw a *URIError* exception.
            if is_unpaired_surrogate {
                return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
            }
            // iii. Set _k_ to _k_ + _codePoint_.[[CodeUnitCount]].
            k += code_unit_count;
            // iv. Let _octets_ be the List of octets resulting by applying the UTF-8 transformation to _codePoint_.[[CodePoint]].
            let Some(ch) = char::from_u32(code_point) else {
                return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
            };
            let mut buf = [0u8; 4];
            // v. For each element _octet_ of _octets_, do
            for octet in ch.encode_utf8(&mut buf).as_bytes() {
                // 1. Let _hex_ be the String representation of _octet_, formatted as an uppercase hexadecimal number.
                // 2. Set _result_ to the string-concatenation of _result_, *"%"*, and StringPad(_hex_, 2, *"0"*, ~start~).
                result.extend(format!("%{octet:02X}").encode_utf16());
            }
        }
    }
    // 7. Return _result_.
    Ok(agent.intern(&result))
}

/// Whether a code unit is in _unescapedSet_: an ASCII word character, one of
/// *"-.!~\*'()"*, or one of _extraUnescaped_.
fn is_unescaped(code_unit: u16, extra_unescaped: &[u16]) -> bool {
    let ascii_word = u8::try_from(code_unit).is_ok_and(|b| b.is_ascii_alphanumeric() || b == b'_');
    ascii_word
        || matches!(
            code_unit,
            0x2D | 0x2E | 0x21 | 0x7E | 0x2A | 0x27 | 0x28 | 0x29
        )
        || extra_unescaped.contains(&code_unit)
}

/// CodePointAt ( _string_, _position_ )
///
/// Returns the code point at `position`, how many code units it occupies, and
/// whether it is an unpaired surrogate.
fn code_point_at(units: &[u16], position: usize) -> (u32, usize, bool) {
    // 1. Let _size_ be the length of _string_.
    // 2. Assert: 0 ≤ _position_ < _size_.
    // 3. Let _first_ be the code unit at index _position_ within _string_.
    let first = units.get(position).copied().unwrap_or(0);
    // 4. Let _cp_ be CodePoint(_first_).
    let cp = u32::from(first);
    // 5. If _cp_ is not a leading surrogate and not a trailing surrogate, then
    if !(0xD800..=0xDFFF).contains(&cp) {
        // a. Return the Record { [[CodePoint]]: _cp_, [[CodeUnitCount]]: 1, [[IsUnpairedSurrogate]]: false }.
        return (cp, 1, false);
    }
    // 6. If _cp_ is a trailing surrogate, then
    if cp >= 0xDC00 {
        // a. Return the Record { [[CodePoint]]: _cp_, [[CodeUnitCount]]: 1, [[IsUnpairedSurrogate]]: true }.
        return (cp, 1, true);
    }
    // 7. If _position_ + 1 = _size_, then
    // 8. Let _second_ be the code unit at index _position_ + 1 within _string_.
    let second = units.get(position + 1).copied().unwrap_or(0);
    // 9. If _second_ is not a trailing surrogate, then
    if !(0xDC00..=0xDFFF).contains(&second) {
        // a. Return the Record { [[CodePoint]]: _cp_, [[CodeUnitCount]]: 1, [[IsUnpairedSurrogate]]: true }.
        return (cp, 1, true);
    }
    // 10. Set _cp_ to UTF16SurrogatePairToCodePoint(_first_, _second_).
    let cp = 0x10000 + ((cp - 0xD800) << 10) + (u32::from(second) - 0xDC00);
    // 11. Return the Record { [[CodePoint]]: _cp_, [[CodeUnitCount]]: 2, [[IsUnpairedSurrogate]]: false }.
    (cp, 2, false)
}
