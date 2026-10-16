//! Uint8Array.fromHex ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.fromhex

use crate::support::typed_array::{allocate_typed_array, constructor_of, viewed_buffer};
use kahvi_core::object::typed_array::{ElementType, set_value_in_buffer};
use kahvi_core::prelude::*;

/// FromHex ( _string_ [ , _maxLength_ ] ) (sec-fromhex)
///
/// Returns ([[Read]], [[Bytes]], [[Error]]).
pub(crate) fn fromhex(
    agent: &mut Agent,
    string: JsString,
    max_length: Option<usize>,
) -> (usize, Vec<u8>, Option<JsError>) {
    // 1. If _maxLength_ is not present, set _maxLength_ to 2^53 - 1.
    let max_length = max_length.unwrap_or(usize::try_from((1u64 << 53) - 1).unwrap_or(usize::MAX));
    // 2. Let _length_ be the length of _string_.
    let units: Vec<u16> = agent.str(string).to_vec();
    let length = units.len();
    // 3. Let _bytes_ be a new empty List.
    let mut bytes: Vec<u8> = Vec::new();
    // 4. Let _read_ be 0.
    let mut read = 0;
    // 5. If _length_ modulo 2 ≠ 0, then
    if length % 2 != 0 {
        // a. Let _error_ be a newly created *SyntaxError* object.
        let error = agent.throw_syntax_error("hex string must have an even length");
        // b. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
        return (read, bytes, Some(error));
    }
    // 6. Repeat, while _read_ < _length_ and the number of elements in _bytes_ < _maxLength_,
    while read < length && bytes.len() < max_length {
        // a. Let _hexits_ be the substring of _string_ from _read_ to _read_ + 2.
        let hexits = &units[read..read + 2];
        // b. If _hexits_ contains any code units which are not in *"0123456789abcdefABCDEF"*, then
        let digit = |u: u16| char::from_u32(u32::from(u)).and_then(|c| c.to_digit(16));
        let (Some(high), Some(low)) = (digit(hexits[0]), digit(hexits[1])) else {
            // i. Let _error_ be a newly created *SyntaxError* object.
            let error = agent.throw_syntax_error("invalid hex digit");
            // ii. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
            return (read, bytes, Some(error));
        };
        // c. Set _read_ to _read_ + 2.
        read += 2;
        // d. Let _byte_ be the integer value represented by _hexits_ in base-16 notation.
        let byte = (high * 16 + low) as u8;
        // e. Append _byte_ to _bytes_.
        bytes.push(byte);
    }
    // 7. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.
    (read, bytes, None)
}

/// Uint8Array.fromHex
pub(crate) fn from_hex(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    // 1. If _string_ is not a String, throw a *TypeError* exception.
    let Value::String(string) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Uint8Array.fromHex requires a string"));
    };
    // 2. Let _result_ be FromHex(_string_).
    let (_read, bytes, error) = fromhex(agent, string, None);
    // 3. If _result_.[[Error]] is not ~none~, then
    if let Some(error) = error {
        // a. Throw _result_.[[Error]].
        return Err(error);
    }
    // 4. Let _resultLength_ be the number of elements in _result_.[[Bytes]].
    let result_length = bytes.len();
    // 5. Let _ta_ be ? AllocateTypedArray(*"Uint8Array"*, %Uint8Array%, *"%Uint8Array.prototype%"*, _resultLength_).
    let ctor = constructor_of(agent, ElementType::Uint8);
    let ta = allocate_typed_array(agent, ElementType::Uint8, ctor, Some(result_length as f64))?;
    // 6. Assert: _ta_.[[ViewedArrayBuffer]].[[ArrayBufferByteLength]] is the number of elements in _result_.[[Bytes]].
    // 7. Set the value at each index of _ta_.[[ViewedArrayBuffer]].[[ArrayBufferData]] to the value at the corresponding index of _result_.[[Bytes]].
    let buffer = viewed_buffer(agent, ta);
    for (index, byte) in bytes.into_iter().enumerate() {
        set_value_in_buffer(
            agent,
            buffer,
            index,
            ElementType::Uint8,
            Value::Number(f64::from(byte)),
            true,
        );
    }
    // 8. Return _ta_.
    Ok(Value::Object(ta))
}
