//! Uint8Array.fromBase64 ( _string_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.frombase64

use kahvi_core::object::typed_array::{ElementType, set_value_in_buffer};
use kahvi_core::prelude::*;

use crate::support::typed_array::{allocate_typed_array, constructor_of, viewed_buffer};

/// The _alphabet_ option of the Uint8Array base64 methods.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Alphabet {
    Base64,
    Base64Url,
}

/// The _lastChunkHandling_ option of the Uint8Array base64 methods.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum LastChunkHandling {
    Loose,
    Strict,
    StopBeforePartial,
}

/// Whether the value `v` is a String equal to the ASCII text `text`.
fn string_is(agent: &Agent, v: Value, text: &str) -> bool {
    match v {
        Value::String(s) => agent.str(s).iter().copied().eq(text.encode_utf16()),
        _ => false,
    }
}

/// GetOptionsObject ( _options_ ) (sec-getoptionsobject)
pub(crate) fn get_options_object(agent: &mut Agent, options: Value) -> JsResult<Object> {
    // 1. If _options_ is *undefined*, then
    if options.is_undefined() {
        // a. Return OrdinaryObjectCreate(*null*).
        return Ok(agent.ordinary_object_create(None));
    }
    // 2. If _options_ is an Object, then
    if let Value::Object(o) = options {
        // a. Return _options_.
        return Ok(o);
    }
    // 3. Throw a *TypeError* exception.
    Err(agent.throw_type_error("options must be an object or undefined"))
}

/// Steps 3-5 of Uint8Array.fromBase64: read and validate the _alphabet_ option.
pub(crate) fn get_alphabet_option(agent: &mut Agent, options_obj: Object) -> JsResult<Alphabet> {
    // 3. Let _alphabet_ be ? Get(_optionsObj_, *"alphabet"*).
    let key = agent.key("alphabet");
    let alphabet = ops::get(agent, options_obj, key)?;
    // 4. If _alphabet_ is *undefined*, set _alphabet_ to *"base64"*.
    if alphabet.is_undefined() {
        return Ok(Alphabet::Base64);
    }
    // 5. If _alphabet_ is neither *"base64"* nor *"base64url"*, throw a *TypeError* exception.
    if string_is(agent, alphabet, "base64") {
        Ok(Alphabet::Base64)
    } else if string_is(agent, alphabet, "base64url") {
        Ok(Alphabet::Base64Url)
    } else {
        Err(agent.throw_type_error("alphabet must be \"base64\" or \"base64url\""))
    }
}

/// Steps 6-8 of Uint8Array.fromBase64: read and validate the _lastChunkHandling_ option.
pub(crate) fn get_last_chunk_handling_option(
    agent: &mut Agent,
    options_obj: Object,
) -> JsResult<LastChunkHandling> {
    // 6. Let _lastChunkHandling_ be ? Get(_optionsObj_, *"lastChunkHandling"*).
    let key = agent.key("lastChunkHandling");
    let handling = ops::get(agent, options_obj, key)?;
    // 7. If _lastChunkHandling_ is *undefined*, set _lastChunkHandling_ to *"loose"*.
    if handling.is_undefined() {
        return Ok(LastChunkHandling::Loose);
    }
    // 8. If _lastChunkHandling_ is not one of *"loose"*, *"strict"*, or *"stop-before-partial"*, throw a *TypeError* exception.
    if string_is(agent, handling, "loose") {
        Ok(LastChunkHandling::Loose)
    } else if string_is(agent, handling, "strict") {
        Ok(LastChunkHandling::Strict)
    } else if string_is(agent, handling, "stop-before-partial") {
        Ok(LastChunkHandling::StopBeforePartial)
    } else {
        Err(agent.throw_type_error(
            "lastChunkHandling must be \"loose\", \"strict\" or \"stop-before-partial\"",
        ))
    }
}

/// The value of a code unit in the standard base64 alphabet.
fn base64_value(unit: u16) -> Option<u8> {
    match u8::try_from(unit).ok()? {
        c @ b'A'..=b'Z' => Some(c - b'A'),
        c @ b'a'..=b'z' => Some(c - b'a' + 26),
        c @ b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// SkipAsciiWhitespace ( _string_, _index_ )
fn skip_ascii_whitespace(string: &[u16], mut index: usize) -> usize {
    // 1. Let _length_ be the length of _string_.
    // 2. Repeat, while _index_ < _length_,
    //    a. Let _char_ be the code unit at _index_.
    //    b. If _char_ is not ASCII whitespace, return _index_.
    //    c. Set _index_ to _index_ + 1.
    while let Some(&c) = string.get(index) {
        if !matches!(c, 0x09 | 0x0A | 0x0C | 0x0D | 0x20) {
            return index;
        }
        index += 1;
    }
    // 3. Return _length_.
    index
}

/// DecodeFullLengthBase64Chunk ( _chunk_ ): `chunk` holds four base64 values.
fn decode_full_length_base64_chunk(chunk: [u8; 4]) -> [u8; 3] {
    // 1. Let _byteSequence_ be the unique sequence of 3 bytes resulting from decoding _chunk_ as base64.
    // 2. Return a List whose elements are the elements of _byteSequence_, in order.
    [
        (chunk[0] << 2) | (chunk[1] >> 4),
        (chunk[1] << 4) | (chunk[2] >> 2),
        (chunk[2] << 6) | chunk[3],
    ]
}

/// DecodeFinalBase64Chunk ( _chunk_, _throwOnExtraBits_ ): `chunk` holds 2 or 3 base64 values.
fn decode_final_base64_chunk(
    agent: &mut Agent,
    chunk: &[u8],
    throw_on_extra_bits: bool,
) -> JsResult<Vec<u8>> {
    // 1. Let _chunkLength_ be the length of _chunk_.
    let chunk_length = chunk.len();
    // 2. If _chunkLength_ = 2, then
    //    a. Set _chunk_ to the string-concatenation of _chunk_ and *"AA"*.
    // 3. Else,
    //    a. Assert: _chunkLength_ is 3.
    //    b. Set _chunk_ to the string-concatenation of _chunk_ and *"A"*.
    let mut full = [0u8; 4];
    full[..chunk_length].copy_from_slice(chunk);
    // 4. Let _bytes_ be DecodeFullLengthBase64Chunk(_chunk_).
    let bytes = decode_full_length_base64_chunk(full);
    // 5. If _chunkLength_ = 2, then
    if chunk_length == 2 {
        // a. If _throwOnExtraBits_ is *true* and _bytes_[1] ≠ 0, throw a *SyntaxError* exception.
        if throw_on_extra_bits && bytes[1] != 0 {
            return Err(agent.throw_syntax_error("invalid base64: extra bits in the last chunk"));
        }
        // b. Return « _bytes_[0] ».
        return Ok(vec![bytes[0]]);
    }
    // 6. If _throwOnExtraBits_ is *true* and _bytes_[2] ≠ 0, throw a *SyntaxError* exception.
    if throw_on_extra_bits && bytes[2] != 0 {
        return Err(agent.throw_syntax_error("invalid base64: extra bits in the last chunk"));
    }
    // 7. Return « _bytes_[0], _bytes_[1] ».
    Ok(vec![bytes[0], bytes[1]])
}

/// FromBase64 ( _string_, _alphabet_, _lastChunkHandling_ [ , _maxLength_ ] )
///
/// Returns the Record's ([[Read]], [[Bytes]], [[Error]]) fields.
pub(crate) fn from_base64_bytes(
    agent: &mut Agent,
    string: JsString,
    alphabet: Alphabet,
    last_chunk_handling: LastChunkHandling,
    max_length: Option<usize>,
) -> (usize, Vec<u8>, Option<JsError>) {
    // 1. If _maxLength_ is not present, then
    //    a. Set _maxLength_ to 2^53 - 1.
    let max_length = max_length.unwrap_or((1usize << 53) - 1);
    // 3. If _maxLength_ = 0, then
    if max_length == 0 {
        // a. Return the Record { [[Read]]: 0, [[Bytes]]: « », [[Error]]: ~none~ }.
        return (0, Vec::new(), None);
    }
    let string: Vec<u16> = agent.str(string).to_vec();
    // 4. Let _read_ be 0.
    let mut read = 0;
    // 5. Let _bytes_ be a new empty List.
    let mut bytes: Vec<u8> = Vec::new();
    // 6. Let _chunk_ be the empty String.
    // 7. Let _chunkLength_ be 0.
    // (The chunk holds the base64 values of its characters; _chunkLength_ is its length.)
    let mut chunk: Vec<u8> = Vec::new();
    // 8. Let _index_ be 0.
    let mut index = 0;
    // 9. Let _length_ be the length of _string_.
    let length = string.len();
    // 10. Repeat,
    loop {
        // a. Assert: The number of elements in _bytes_ is evenly divisible by 3.
        // b. Set _index_ to SkipAsciiWhitespace(_string_, _index_).
        index = skip_ascii_whitespace(&string, index);
        // c. If _index_ = _length_, then
        if index == length {
            // i. If _chunkLength_ > 0, then
            if !chunk.is_empty() {
                // 1. If _lastChunkHandling_ is *"stop-before-partial"*, then
                if last_chunk_handling == LastChunkHandling::StopBeforePartial {
                    // a. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.
                    return (read, bytes, None);
                }
                // 2. If _lastChunkHandling_ is *"strict"*, then
                if last_chunk_handling == LastChunkHandling::Strict {
                    // a. Let _error_ be a newly created *SyntaxError* object.
                    let error = agent.throw_syntax_error("invalid base64: incomplete last chunk");
                    // b. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
                    return (read, bytes, Some(error));
                }
                // 3. Assert: _lastChunkHandling_ is *"loose"*.
                // 4. If _chunkLength_ = 1, then
                if chunk.len() == 1 {
                    // a. Let _error_ be a newly created *SyntaxError* object.
                    let error = agent.throw_syntax_error("invalid base64: incomplete last chunk");
                    // b. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
                    return (read, bytes, Some(error));
                }
                // 5. Set _bytes_ to the list-concatenation of _bytes_ and ! DecodeFinalBase64Chunk(_chunk_, *false*).
                match decode_final_base64_chunk(agent, &chunk, false) {
                    Ok(decoded) => bytes.extend(decoded),
                    Err(error) => return (read, bytes, Some(error)),
                }
            }
            // ii. Return the Record { [[Read]]: _length_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.
            return (length, bytes, None);
        }
        // d. Let _char_ be the substring of _string_ from _index_ to _index_ + 1.
        let mut char = string[index];
        // e. Set _index_ to _index_ + 1.
        index += 1;
        // f. If _char_ is *"="*, then
        if char == u16::from(b'=') {
            // i. If _chunkLength_ < 2, then
            if chunk.len() < 2 {
                // 1. Let _error_ be a newly created *SyntaxError* object.
                let error = agent.throw_syntax_error("invalid base64: unexpected padding");
                // 2. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
                return (read, bytes, Some(error));
            }
            // ii. Set _index_ to SkipAsciiWhitespace(_string_, _index_).
            index = skip_ascii_whitespace(&string, index);
            // iii. If _chunkLength_ = 2, then
            if chunk.len() == 2 {
                // 1. If _index_ = _length_, then
                if index == length {
                    // a. If _lastChunkHandling_ is *"stop-before-partial"*, then
                    if last_chunk_handling == LastChunkHandling::StopBeforePartial {
                        // i. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.
                        return (read, bytes, None);
                    }
                    // b. Let _error_ be a newly created *SyntaxError* object.
                    let error = agent.throw_syntax_error("invalid base64: incomplete padding");
                    // c. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
                    return (read, bytes, Some(error));
                }
                // 2. Set _char_ to the substring of _string_ from _index_ to _index_ + 1.
                char = string[index];
                // 3. If _char_ is *"="*, then
                if char == u16::from(b'=') {
                    // a. Set _index_ to SkipAsciiWhitespace(_string_, _index_ + 1).
                    index = skip_ascii_whitespace(&string, index + 1);
                }
            }
            // iv. If _index_ < _length_, then
            if index < length {
                // 1. Let _error_ be a newly created *SyntaxError* object.
                let error = agent.throw_syntax_error("invalid base64: data after padding");
                // 2. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
                return (read, bytes, Some(error));
            }
            // v. If _lastChunkHandling_ is *"strict"*, let _throwOnExtraBits_ be *true*; else let _throwOnExtraBits_ be *false*.
            let throw_on_extra_bits = last_chunk_handling == LastChunkHandling::Strict;
            // vi. Let _decodeResult_ be Completion(DecodeFinalBase64Chunk(_chunk_, _throwOnExtraBits_)).
            // vii. If _decodeResult_ is an abrupt completion, then
            //    1. Let _error_ be _decodeResult_.[[Value]].
            //    2. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
            let decoded = match decode_final_base64_chunk(agent, &chunk, throw_on_extra_bits) {
                Ok(decoded) => decoded,
                Err(error) => return (read, bytes, Some(error)),
            };
            // viii. Set _bytes_ to the list-concatenation of _bytes_ and ! _decodeResult_.
            bytes.extend(decoded);
            // ix. Return the Record { [[Read]]: _length_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.
            return (length, bytes, None);
        }
        // g. If _alphabet_ is *"base64url"*, then
        if alphabet == Alphabet::Base64Url {
            // i. If _char_ is either *"+"* or *"/"*, then
            if char == u16::from(b'+') || char == u16::from(b'/') {
                // 1. Let _error_ be a newly created *SyntaxError* object.
                let error = agent.throw_syntax_error("invalid base64url character");
                // 2. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
                return (read, bytes, Some(error));
            }
            // ii. Else if _char_ is *"-"*, then
            //    1. Set _char_ to *"+"*.
            // iii. Else if _char_ is *"_"*, then
            //    1. Set _char_ to *"/"*.
            if char == u16::from(b'-') {
                char = u16::from(b'+');
            } else if char == u16::from(b'_') {
                char = u16::from(b'/');
            }
        }
        // h. If the sole code unit of _char_ is not an element of the standard base64 alphabet, then
        let Some(value) = base64_value(char) else {
            // i. Let _error_ be a newly created *SyntaxError* object.
            let error = agent.throw_syntax_error("invalid base64 character");
            // ii. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
            return (read, bytes, Some(error));
        };
        // i. Let _remaining_ be _maxLength_ - the number of elements in _bytes_.
        let remaining = max_length - bytes.len();
        // j. If _remaining_ = 1 and _chunkLength_ = 2, or if _remaining_ = 2 and _chunkLength_ = 3, then
        if (remaining == 1 && chunk.len() == 2) || (remaining == 2 && chunk.len() == 3) {
            // i. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.
            return (read, bytes, None);
        }
        // k. Set _chunk_ to the string-concatenation of _chunk_ and _char_.
        chunk.push(value);
        // l. Set _chunkLength_ to the length of _chunk_.
        // m. If _chunkLength_ = 4, then
        if chunk.len() == 4 {
            // i. Set _bytes_ to the list-concatenation of _bytes_ and DecodeFullLengthBase64Chunk(_chunk_).
            bytes.extend(decode_full_length_base64_chunk([
                chunk[0], chunk[1], chunk[2], chunk[3],
            ]));
            // ii. Set _chunk_ to the empty String.
            // iii. Set _chunkLength_ to 0.
            chunk.clear();
            // iv. Set _read_ to _index_.
            read = index;
            // v. If the number of elements in _bytes_ = _maxLength_, then
            if bytes.len() == max_length {
                // 1. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.
                return (read, bytes, None);
            }
        }
    }
}

/// Uint8Array.fromBase64
pub(crate) fn from_base64(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _string_ is not a String, throw a *TypeError* exception.
    let Value::String(string) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Uint8Array.fromBase64 requires a string"));
    };
    // 2. Let _optionsObj_ be ? GetOptionsObject(_options_).
    let options_obj = get_options_object(agent, args.get_or_undefined(1))?;
    // 3-5. Let _alphabet_ be ? Get(_optionsObj_, *"alphabet"*), defaulting to *"base64"*; throw a *TypeError* if invalid.
    let alphabet = get_alphabet_option(agent, options_obj)?;
    // 6-8. Let _lastChunkHandling_ be ? Get(_optionsObj_, *"lastChunkHandling"*), defaulting to *"loose"*; throw a *TypeError* if invalid.
    let last_chunk_handling = get_last_chunk_handling_option(agent, options_obj)?;
    // 9. Let _result_ be FromBase64(_string_, _alphabet_, _lastChunkHandling_).
    let (_, bytes, error) = from_base64_bytes(agent, string, alphabet, last_chunk_handling, None);
    // 10. If _result_.[[Error]] is not ~none~, then
    if let Some(error) = error {
        // a. Throw _result_.[[Error]].
        return Err(error);
    }
    // 11. Let _resultLength_ be the number of elements in _result_.[[Bytes]].
    let result_length = bytes.len();
    // 12. Let _ta_ be ? AllocateTypedArray(*"Uint8Array"*, %Uint8Array%, *"%Uint8Array.prototype%"*, _resultLength_).
    let uint8_array = constructor_of(agent, ElementType::Uint8);
    let ta = allocate_typed_array(
        agent,
        ElementType::Uint8,
        uint8_array,
        Some(result_length as f64),
    )?;
    // 13. Assert: _ta_.[[ViewedArrayBuffer]].[[ArrayBufferByteLength]] is the number of elements in _result_.[[Bytes]].
    // 14. Set the value at each index of _ta_.[[ViewedArrayBuffer]].[[ArrayBufferData]] to the value at the corresponding index of _result_.[[Bytes]].
    let buffer = viewed_buffer(agent, ta);
    for (i, byte) in bytes.into_iter().enumerate() {
        set_value_in_buffer(
            agent,
            buffer,
            i,
            ElementType::Uint8,
            Value::Number(f64::from(byte)),
            true,
        );
    }
    // 15. Return _ta_.
    Ok(Value::Object(ta))
}
