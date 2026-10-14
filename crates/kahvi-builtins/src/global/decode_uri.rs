//! decodeURI ( _encodedURI_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-decodeuri-encodeduri

use kahvi_core::prelude::*;

/// decodeURI
pub(crate) fn decode_uri(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function computes a new version of a URI in which each escape sequence and UTF-8 encoding of the sort that might be introduced by the `encodeURI` function is replaced with the UTF-16 encoding of the code point that it represents. Escape sequences that could not have been introduced by `encodeURI` are not replaced.
    // It is the %decodeURI% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _uriString_ be ? ToString(_encodedURI_).
    let uri_string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 2. Let _preserveEscapeSet_ be *";/?:@&=+$,#"*.
    let preserve_escape_set = agent.string(";/?:@&=+$,#");
    // 3. Return ? Decode(_uriString_, _preserveEscapeSet_).
    let _ = (this, new_target);
    let decoded = decode(agent, uri_string, preserve_escape_set)?;
    Ok(Value::String(decoded))
}

/// Decode ( _string_: a String, _preserveEscapeSet_: a String, ): either a normal completion containing a String or a throw completion
///
/// It performs URI unescaping and decoding, preserving any escape sequences that correspond to Basic Latin characters in _preserveEscapeSet_.
pub(crate) fn decode(
    agent: &mut Agent,
    string: JsString,
    preserve_escape_set: JsString,
) -> JsResult<JsString> {
    let units = agent.str(string).to_vec();
    let preserve = agent.str(preserve_escape_set).to_vec();
    // 1. Let _length_ be the length of _string_.
    let length = units.len();
    // 2. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::new();
    // 3. Let _k_ be 0.
    let mut k = 0;
    // 4. Repeat, while _k_ < _length_,
    while k < length {
        // a. Let _codeUnit_ be the code unit at index _k_ within _string_.
        let code_unit = units[k];
        // b. Let _segment_ be _codeUnit_.
        let mut segment: Vec<u16> = vec![code_unit];
        // c. If _codeUnit_ is the code unit 0x0025 (PERCENT SIGN), then
        if code_unit == 0x0025 {
            // i. If _k_ + 3 > _length_, throw a *URIError* exception.
            if k + 3 > length {
                return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
            }
            // ii. Let _escape_ be the substring of _string_ from _k_ to _k_ + 3.
            let escape = units[k..k + 3].to_vec();
            // iii. Let _firstOctet_ be ParseHexOctet(_string_, _k_ + 1).
            // iv. If _firstOctet_ is not an integer, throw a *URIError* exception.
            let Some(first_octet) = parse_hex_octet(&units, k + 1) else {
                return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
            };
            // v. Set _k_ to _k_ + 2.
            k += 2;
            // vi. Let _n_ be the number of leading 1 bits in _firstOctet_.
            let n = first_octet.leading_ones();
            // vii. If _n_ = 0, then
            if n == 0 {
                // 1. Let _asciiChar_ be the code unit whose numeric value is _firstOctet_.
                let ascii_char = u16::from(first_octet);
                // 2. If _preserveEscapeSet_ contains _asciiChar_, set _segment_ to _escape_; else set _segment_ to _asciiChar_.
                segment = if preserve.contains(&ascii_char) {
                    escape
                } else {
                    vec![ascii_char]
                };
            } else {
                // viii. Else,
                // 1. If _n_ = 1 or _n_ > 4, throw a *URIError* exception.
                if n == 1 || n > 4 {
                    return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
                }
                // 2. Let _octets_ be « _firstOctet_ ».
                let mut octets = vec![first_octet];
                // 3. Let _j_ be 1.
                let mut j = 1;
                // 4. Repeat, while _j_ < _n_,
                while j < n {
                    // a. Set _k_ to _k_ + 1.
                    k += 1;
                    // b. If _k_ + 3 > _length_, throw a *URIError* exception.
                    if k + 3 > length {
                        return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
                    }
                    // c. If the code unit at index _k_ within _string_ is not the code unit 0x0025 (PERCENT SIGN), throw a *URIError* exception.
                    if units[k] != 0x0025 {
                        return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
                    }
                    // d. Let _continuationByte_ be ParseHexOctet(_string_, _k_ + 1).
                    // e. If _continuationByte_ is not an integer, throw a *URIError* exception.
                    let Some(continuation_byte) = parse_hex_octet(&units, k + 1) else {
                        return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
                    };
                    // f. Append _continuationByte_ to _octets_.
                    octets.push(continuation_byte);
                    // g. Set _k_ to _k_ + 2.
                    k += 2;
                    // h. Set _j_ to _j_ + 1.
                    j += 1;
                }
                // 5. Assert: The length of _octets_ is _n_.
                // 6. If _octets_ does not contain a valid UTF-8 encoding of a Unicode code point, throw a *URIError* exception.
                let Ok(decoded) = std::str::from_utf8(&octets) else {
                    return Err(agent.throw_error(ErrorKind::Uri, "URI malformed"));
                };
                // 7. Let _codePoint_ be the code point obtained by applying the UTF-8 transformation to _octets_, that is, from a List of octets into a 21-bit value.
                // 8. Set _segment_ to UTF16EncodeCodePoint(_codePoint_).
                // `from_utf8` accepted exactly one code point, so its UTF-16 encoding is the segment.
                segment = decoded.encode_utf16().collect();
            }
        }
        // d. Set _result_ to the string-concatenation of _result_ and _segment_.
        result.extend_from_slice(&segment);
        // e. Set _k_ to _k_ + 1.
        k += 1;
    }
    // 5. Return _result_.
    Ok(agent.intern(&result))
}

/// ParseHexOctet ( _string_: a String, _position_: a non-negative integer, ): an integer or not-a-number
///
/// Returns the octet encoded by the two hexadecimal digits at `position`, or `None` if they are not both hexadecimal digits.
fn parse_hex_octet(units: &[u16], position: usize) -> Option<u8> {
    let [high, low] = units.get(position..position + 2)? else {
        return None;
    };
    let high = char::from_u32(u32::from(*high))?.to_digit(16)?;
    let low = char::from_u32(u32::from(*low))?.to_digit(16)?;
    u8::try_from(high * 16 + low).ok()
}
