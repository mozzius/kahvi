//! The WHATWG Encoding Standard's `TextEncoder` and `TextDecoder` (UTF-8
//! only), and HTML's `atob` and `btoa`: web APIs that Hermes provides and
//! React Native code expects, installed by hosts with `install`.
//!
//! The classes are a small script over native helpers that do the byte
//! work. HTML throws "InvalidCharacterError" DOMExceptions; there is no
//! DOMException here, so those are Errors with that name. A TextDecoder
//! for any encoding but UTF-8 throws the not-implemented error.

use kahvi_core::object::typed_array::{
    ElementType, allocate_array_buffer, array_buffer_byte_length, array_buffer_data,
    array_buffer_data_mut, data_view_data, is_detached_buffer, is_typed_array_out_of_bounds,
    make_ta_record, typed_array_byte_length, typed_array_data,
};
use kahvi_core::prelude::*;

use crate::{Agent, JsResult, Runtime, Value};

/// The classes, given the global object and the native helpers.
const CLASSES: &str = r#"(function (global, encode, encodeInto, decode, notImplemented) {
  "use strict";
  const UTF8_LABELS = [
    "unicode-1-1-utf-8", "unicode11utf8", "unicode20utf8", "utf-8", "utf8", "x-unicode20utf8",
  ];
  const define = (name, value) =>
    Object.defineProperty(global, name, { value, writable: true, enumerable: false, configurable: true });
  const tag = (C, name) =>
    Object.defineProperty(C.prototype, Symbol.toStringTag, { value: name, configurable: true });

  class TextEncoder {
    #brand = true;
    get encoding() {
      this.#brand;
      return "utf-8";
    }
    encode(input = "") {
      this.#brand;
      return new Uint8Array(encode(`${input}`));
    }
    encodeInto(source, destination) {
      this.#brand;
      if (arguments.length < 2) {
        throw new TypeError("TextEncoder.encodeInto: 2 arguments required");
      }
      const [read, written] = encodeInto(`${source}`, destination);
      return { read, written };
    }
  }
  tag(TextEncoder, "TextEncoder");

  class TextDecoder {
    #fatal;
    #ignoreBOM;
    // Bytes of an incomplete sequence kept from a streaming decode, and
    // whether the byte order mark has been dealt with.
    #pending = undefined;
    #bomSeen = false;
    constructor(label = "utf-8", options = undefined) {
      const name = `${label}`.replace(/^[\t\n\f\r ]+|[\t\n\f\r ]+$/g, "").toLowerCase();
      if (!UTF8_LABELS.includes(name)) {
        notImplemented(`TextDecoder for encoding label "${name}"`);
      }
      if (options !== undefined && options !== null && typeof options !== "object" && typeof options !== "function") {
        throw new TypeError("TextDecoder: options must be an object");
      }
      this.#fatal = options == null ? false : Boolean(options.fatal);
      this.#ignoreBOM = options == null ? false : Boolean(options.ignoreBOM);
    }
    get encoding() {
      this.#fatal;
      return "utf-8";
    }
    get fatal() {
      return this.#fatal;
    }
    get ignoreBOM() {
      return this.#ignoreBOM;
    }
    decode(input = undefined, options = undefined) {
      this.#fatal;
      if (options !== undefined && options !== null && typeof options !== "object" && typeof options !== "function") {
        throw new TypeError("TextDecoder.decode: options must be an object");
      }
      const stream = options == null ? false : Boolean(options.stream);
      const [text, pending, bomSeen] = decode(
        this.#pending, input, this.#fatal, this.#ignoreBOM, this.#bomSeen, stream,
      );
      this.#pending = pending;
      this.#bomSeen = bomSeen;
      return text;
    }
  }
  tag(TextDecoder, "TextDecoder");

  define("TextEncoder", TextEncoder);
  define("TextDecoder", TextDecoder);
})"#;

/// Installs `TextEncoder`, `TextDecoder`, `atob` and `btoa` as globals.
pub fn install(rt: &mut Runtime) {
    rt.set_global_function("atob", 1, |agent, _, args| {
        let Some(&data) = args.first() else {
            return Err(agent.throw_type_error("atob: 1 argument required"));
        };
        let data = ops::to_string(agent, data)?;
        let units = agent.str(data).to_vec();
        match forgiving_base64_decode(&units) {
            Some(bytes) => {
                let units: Vec<u16> = bytes.into_iter().map(u16::from).collect();
                Ok(Value::String(agent.intern(&units)))
            }
            None => Err(invalid_character_error(
                agent,
                "atob: the string is not valid base64",
            )),
        }
    });
    rt.set_global_function("btoa", 1, |agent, _, args| {
        let Some(&data) = args.first() else {
            return Err(agent.throw_type_error("btoa: 1 argument required"));
        };
        let data = ops::to_string(agent, data)?;
        let mut bytes = Vec::with_capacity(agent.str(data).len());
        for &u in agent.str(data) {
            match u8::try_from(u) {
                Ok(b) => bytes.push(b),
                Err(_) => {
                    return Err(invalid_character_error(
                        agent,
                        "btoa: the string contains characters outside of the Latin1 range",
                    ));
                }
            }
        }
        let text = base64_encode(&bytes);
        Ok(Value::String(agent.string(&text)))
    });

    let encode = rt.function("encode", 1, |agent, _, args| {
        let s = string_arg(agent, args)?;
        let bytes = String::from_utf16_lossy(agent.str(s)).into_bytes();
        let ctor = agent.intrinsic("%ArrayBuffer%");
        let buffer = allocate_array_buffer(agent, ctor, bytes.len() as f64, None)?;
        array_buffer_data_mut(agent, buffer)
            .expect("an ArrayBuffer")
            .data = Some(bytes);
        Ok(Value::Object(buffer))
    });
    let encode_into = rt.function("encodeInto", 2, |agent, _, args| {
        let s = string_arg(agent, args)?;
        let dest = match args.get(1) {
            Some(&Value::Object(o))
                if typed_array_data(agent, o).is_some_and(|d| d.kind == ElementType::Uint8) =>
            {
                o
            }
            _ => {
                return Err(agent.throw_type_error(
                    "TextEncoder.encodeInto: the destination must be a Uint8Array",
                ));
            }
        };
        let (buffer, offset, len) = typed_array_bytes(agent, dest);
        let units = agent.str(s).to_vec();
        let mut out = Vec::with_capacity(len.min(units.len() * 3));
        let mut read = 0;
        for c in char::decode_utf16(units.iter().copied()) {
            let (c, n) = match c {
                Ok(c) => (c, c.len_utf16()),
                Err(_) => (char::REPLACEMENT_CHARACTER, 1),
            };
            let mut buf = [0; 4];
            let encoded = c.encode_utf8(&mut buf).as_bytes();
            if out.len() + encoded.len() > len {
                break;
            }
            out.extend_from_slice(encoded);
            read += n;
        }
        if let Some(data) = array_buffer_data_mut(agent, buffer).and_then(|d| d.data.as_mut()) {
            data[offset..offset + out.len()].copy_from_slice(&out);
        }
        let result = [Value::Number(read as f64), Value::Number(out.len() as f64)];
        Ok(Value::Object(ops::create_array_from_list(agent, &result)))
    });
    // decode(pending, input, fatal, ignoreBOM, bomSeen, stream) returns
    // [text, pending, bomSeen] for the decoder to keep.
    let decode = rt.function("decode", 6, |agent, _, args| {
        let arg = |i: usize| args.get(i).copied().unwrap_or(Value::Undefined);
        let mut bytes = match arg(0) {
            Value::Object(o) => buffer_source_bytes(agent, o).unwrap_or_default(),
            _ => Vec::new(),
        };
        match arg(1) {
            Value::Undefined => {}
            Value::Object(o) => match buffer_source_bytes(agent, o) {
                Some(b) => bytes.extend_from_slice(&b),
                None => return Err(not_buffer_source(agent)),
            },
            _ => return Err(not_buffer_source(agent)),
        }
        let fatal = matches!(arg(2), Value::Boolean(true));
        let ignore_bom = matches!(arg(3), Value::Boolean(true));
        let mut bom_seen = matches!(arg(4), Value::Boolean(true));
        let stream = matches!(arg(5), Value::Boolean(true));
        const BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
        let mut start = 0;
        if !ignore_bom && !bom_seen {
            if stream && bytes.len() < BOM.len() && BOM.starts_with(&bytes) {
                // Not enough bytes yet to tell whether they are a BOM.
                let text = Value::String(agent.names.empty);
                return decode_result(agent, text, bytes, false);
            }
            if bytes.starts_with(&BOM) {
                start = BOM.len();
            }
            bom_seen = !bytes.is_empty();
        }
        let (text, rest) = match utf8_decode(&bytes[start..], fatal, stream) {
            Some(r) => r,
            None => {
                return Err(
                    agent.throw_type_error("TextDecoder.decode: the data is not valid UTF-8")
                );
            }
        };
        let rest = rest.to_vec();
        if !stream {
            bom_seen = false;
        }
        let text = Value::String(agent.string(&text));
        decode_result(agent, text, rest, bom_seen)
    });
    let not_implemented = rt.function("notImplemented", 1, |agent, _, args| {
        let s = string_arg(agent, args)?;
        let what = agent.str_lossy(s);
        Err(agent.not_implemented(&what))
    });

    // Compiling the script can collect garbage: keep the helpers alive.
    let helpers: Vec<_> = [encode, encode_into, decode, not_implemented]
        .into_iter()
        .map(|f| rt.persist(Value::Object(f)))
        .collect();
    let classes = rt
        .eval_script(CLASSES, "<kahvi encoding>")
        .expect("the encoding classes compile");
    let mut args = vec![Value::Object(rt.global())];
    args.extend(helpers.iter().map(|p| rt.persisted(p)));
    let helpers = args;
    rt.call(classes, Value::Undefined, &helpers)
        .expect("the encoding classes install");
}

/// The first argument, which the script has already converted to a string.
fn string_arg(agent: &mut Agent, args: &[Value]) -> JsResult<JsString> {
    match args.first() {
        Some(&Value::String(s)) => Ok(s),
        _ => Err(agent.throw_type_error("expected a string")),
    }
}

fn decode_result(
    agent: &mut Agent,
    text: Value,
    pending: Vec<u8>,
    bom_seen: bool,
) -> JsResult<Value> {
    let pending = if pending.is_empty() {
        Value::Undefined
    } else {
        let ctor = agent.intrinsic("%ArrayBuffer%");
        let buffer = allocate_array_buffer(agent, ctor, pending.len() as f64, None)?;
        array_buffer_data_mut(agent, buffer)
            .expect("an ArrayBuffer")
            .data = Some(pending);
        Value::Object(buffer)
    };
    let list = [text, pending, Value::Boolean(bom_seen)];
    Ok(Value::Object(ops::create_array_from_list(agent, &list)))
}

fn not_buffer_source(agent: &mut Agent) -> kahvi_core::JsError {
    agent.throw_type_error(
        "TextDecoder.decode: the input must be an ArrayBuffer, a TypedArray or a DataView",
    )
}

/// The buffer, byte offset and byte length a typed array views (a length
/// of 0 when it is out of bounds or its buffer is detached).
fn typed_array_bytes(agent: &Agent, o: Object) -> (Object, usize, usize) {
    let d = typed_array_data(agent, o).expect("a TypedArray");
    let buffer = d.buffer.expect("an initialized TypedArray");
    let rec = make_ta_record(agent, o);
    if is_typed_array_out_of_bounds(agent, rec) {
        return (buffer, 0, 0);
    }
    (buffer, d.byte_offset, typed_array_byte_length(agent, rec))
}

/// A copy of the bytes of an ArrayBuffer, SharedArrayBuffer, TypedArray or
/// DataView (none when detached or out of bounds); None for other objects.
fn buffer_source_bytes(agent: &Agent, o: Object) -> Option<Vec<u8>> {
    let (buffer, offset, len) = if array_buffer_data(agent, o).is_some() {
        let len = if is_detached_buffer(agent, o) {
            0
        } else {
            array_buffer_byte_length(agent, o)
        };
        (o, 0, len)
    } else if typed_array_data(agent, o).is_some() {
        typed_array_bytes(agent, o)
    } else {
        let d = data_view_data(agent, o)?;
        let buffer_len = if is_detached_buffer(agent, d.buffer) {
            None
        } else {
            Some(array_buffer_byte_length(agent, d.buffer))
        };
        let end = buffer_len.and_then(|b| match d.byte_length {
            Some(len) => Some(d.byte_offset + len).filter(|&e| e <= b),
            None => Some(b).filter(|&b| d.byte_offset <= b),
        });
        match end {
            Some(end) => (d.buffer, d.byte_offset, end - d.byte_offset),
            None => (d.buffer, 0, 0),
        }
    };
    let data = array_buffer_data(agent, buffer)
        .and_then(|d| d.data.as_deref())
        .unwrap_or(&[]);
    Some(data.get(offset..offset + len).unwrap_or(&[]).to_vec())
}

/// UTF-8 decode (Encoding §8.1.1) of `bytes`, each error a U+FFFD, or None
/// for the first error when `fatal`. When `stream`, an incomplete sequence
/// at the end is not decoded but returned for the next call.
fn utf8_decode(mut bytes: &[u8], fatal: bool, stream: bool) -> Option<(String, &[u8])> {
    let mut out = String::with_capacity(bytes.len());
    loop {
        match std::str::from_utf8(bytes) {
            Ok(s) => {
                out.push_str(s);
                return Some((out, &[]));
            }
            Err(e) => {
                let (valid, after) = bytes.split_at(e.valid_up_to());
                out.push_str(std::str::from_utf8(valid).expect("valid UTF-8"));
                match e.error_len() {
                    Some(n) => {
                        if fatal {
                            return None;
                        }
                        out.push(char::REPLACEMENT_CHARACTER);
                        bytes = &after[n..];
                    }
                    // An incomplete sequence at the end.
                    None if stream => return Some((out, after)),
                    None => {
                        if fatal {
                            return None;
                        }
                        out.push(char::REPLACEMENT_CHARACTER);
                        return Some((out, &[]));
                    }
                }
            }
        }
    }
}

const BASE64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = (u32::from(chunk[0]) << 16)
            | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8)
            | u32::from(*chunk.get(2).unwrap_or(&0));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(BASE64[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// Infra's forgiving-base64 decode; None is failure.
fn forgiving_base64_decode(units: &[u16]) -> Option<Vec<u8>> {
    // 1. Remove all ASCII whitespace from data.
    let mut data: Vec<u16> = units
        .iter()
        .copied()
        .filter(|&u| !matches!(u, 0x09 | 0x0A | 0x0C | 0x0D | 0x20))
        .collect();
    // 2. If data's code point length divides by 4 leaving no remainder, then
    // remove one or two U+003D (=) code points from the end of data.
    if data.len().is_multiple_of(4) {
        for _ in 0..2 {
            if data.last() == Some(&0x3D) {
                data.pop();
            }
        }
    }
    // 3. If data's code point length divides by 4 leaving a remainder of 1, return failure.
    if data.len() % 4 == 1 {
        return None;
    }
    // 4-8. Decode each code point's 6 bits; discard trailing bits that do
    // not make a byte. A code point outside the alphabet is failure.
    let mut out = Vec::with_capacity(data.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0;
    for u in data {
        let v = match u {
            0x41..=0x5A => u - 0x41,
            0x61..=0x7A => u - 0x61 + 26,
            0x30..=0x39 => u - 0x30 + 52,
            0x2B => 62,
            0x2F => 63,
            _ => return None,
        };
        buffer = (buffer << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }
    Some(out)
}

/// HTML's "InvalidCharacterError" DOMException, as an Error with that name.
fn invalid_character_error(agent: &mut Agent, message: &str) -> kahvi_core::JsError {
    let e = agent.create_error(ErrorKind::Error, message);
    let name_key = agent.key("name");
    let name = agent.string("InvalidCharacterError");
    ops::create_data_property_or_throw(agent, e, name_key, Value::String(name))
        .expect("a new error object takes a name");
    kahvi_core::JsError::Throw(Value::Object(e))
}
