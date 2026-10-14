//! Type conversion (ECMA-262 §7.1).

use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{ObjectKind, PropertyKey};
use crate::ops::{call, get_method, is_callable};
use crate::value::{BigInt, JsString, Object, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferredType {
    Default,
    String,
    Number,
}

/// ToPrimitive(input [, preferredType]) (§7.1.1)
pub fn to_primitive(agent: &mut Agent, input: Value, preferred: PreferredType) -> JsResult<Value> {
    // 1. If input is an Object, then
    let Value::Object(o) = input else {
        // 2. Return input.
        return Ok(input);
    };
    // a. Let exoticToPrim be ? GetMethod(input, %Symbol.toPrimitive%).
    let key = PropertyKey::Symbol(agent.symbols_wk.to_primitive);
    let exotic_to_prim = get_method(agent, input, key)?;
    // b. If exoticToPrim is not undefined, then
    if !exotic_to_prim.is_undefined() {
        // i-iv. Let hint be "default", "string" or "number".
        let hint = match preferred {
            PreferredType::Default => "default",
            PreferredType::String => "string",
            PreferredType::Number => "number",
        };
        let hint = Value::String(agent.string(hint));
        // v. Let result be ? Call(exoticToPrim, input, « hint »).
        let result = call(agent, exotic_to_prim, input, &[hint])?;
        // vi. If result is not an Object, return result.
        if !result.is_object() {
            return Ok(result);
        }
        // vii. Throw a TypeError exception.
        return Err(agent.throw_type_error("Cannot convert object to primitive value"));
    }
    // c. If preferredType is not present, let preferredType be number.
    let preferred = if preferred == PreferredType::Default {
        PreferredType::Number
    } else {
        preferred
    };
    // d. Return ? OrdinaryToPrimitive(input, preferredType).
    ordinary_to_primitive(agent, o, preferred)
}

/// OrdinaryToPrimitive(O, hint) (§7.1.1.1)
pub fn ordinary_to_primitive(agent: &mut Agent, o: Object, hint: PreferredType) -> JsResult<Value> {
    // 1-2. Let methodNames be « "toString", "valueOf" » for string, else « "valueOf", "toString" ».
    let names = if hint == PreferredType::String {
        [agent.names.to_string, agent.names.value_of]
    } else {
        [agent.names.value_of, agent.names.to_string]
    };
    // 3. For each element name of methodNames, do
    for name in names {
        // a. Let method be ? Get(O, name).
        let method = crate::ops::get(agent, o, PropertyKey::String(name))?;
        // b. If IsCallable(method) is true, then
        if is_callable(agent, method) {
            // i. Let result be ? Call(method, O).
            let result = call(agent, method, Value::Object(o), &[])?;
            // ii. If result is not an Object, return result.
            if !result.is_object() {
                return Ok(result);
            }
        }
    }
    // 4. Throw a TypeError exception.
    Err(agent.throw_type_error("Cannot convert object to primitive value"))
}

/// ToBoolean(argument) (§7.1.2)
pub fn to_boolean(agent: &Agent, v: Value) -> bool {
    match v {
        Value::Undefined | Value::Null => false,
        Value::Boolean(b) => b,
        Value::Number(n) => !(n == 0.0 || n.is_nan()),
        Value::String(s) => agent.str_len(s) != 0,
        Value::Symbol(_) => true,
        Value::BigInt(b) => !crate::ops::bigint::is_zero(agent, b),
        Value::Object(_) => true,
    }
}

/// A numeric value: the result of ToNumeric.
#[derive(Debug, Clone, Copy)]
pub enum Numeric {
    Number(f64),
    BigInt(BigInt),
}

/// ToNumeric(value) (§7.1.3)
pub fn to_numeric(agent: &mut Agent, value: Value) -> JsResult<Numeric> {
    // 1. Let primValue be ? ToPrimitive(value, number).
    let prim = to_primitive(agent, value, PreferredType::Number)?;
    // 2. If primValue is a BigInt, return primValue.
    if let Value::BigInt(b) = prim {
        return Ok(Numeric::BigInt(b));
    }
    // 3. Return ? ToNumber(primValue).
    Ok(Numeric::Number(to_number(agent, prim)?))
}

/// ToNumber(argument) (§7.1.4)
pub fn to_number(agent: &mut Agent, v: Value) -> JsResult<f64> {
    match v {
        // 1. If argument is a Number, return argument.
        Value::Number(n) => Ok(n),
        // 2. If argument is either a Symbol or a BigInt, throw a TypeError exception.
        Value::Symbol(_) => {
            Err(agent.throw_type_error("Cannot convert a Symbol value to a number"))
        }
        Value::BigInt(_) => {
            Err(agent.throw_type_error("Cannot convert a BigInt value to a number"))
        }
        // 3. If argument is undefined, return NaN.
        Value::Undefined => Ok(f64::NAN),
        // 4. If argument is either null or false, return +0𝔽.
        Value::Null => Ok(0.0),
        // 5. If argument is true, return 1𝔽.
        Value::Boolean(b) => Ok(if b { 1.0 } else { 0.0 }),
        // 6. If argument is a String, return StringToNumber(argument).
        Value::String(s) => Ok(string_to_number(agent.str(s))),
        // 7-9. Let primValue be ? ToPrimitive(argument, number). Return ? ToNumber(primValue).
        Value::Object(_) => {
            let prim = to_primitive(agent, v, PreferredType::Number)?;
            to_number(agent, prim)
        }
    }
}

/// StringToNumber(str) (§7.1.4.1.1): the StringNumericLiteral grammar.
pub fn string_to_number(units: &[u16]) -> f64 {
    let Ok(s) = String::from_utf16(units) else {
        return f64::NAN;
    };
    let s = s.trim_matches(|c: char| {
        kahvi_parser::lexer::is_whitespace(c) || kahvi_parser::lexer::is_line_terminator(c)
    });
    if s.is_empty() {
        return 0.0;
    }
    let (sign, body) = match s.as_bytes()[0] {
        b'+' => (1.0, &s[1..]),
        b'-' => (-1.0, &s[1..]),
        _ => (1.0, s),
    };
    if body == "Infinity" {
        return sign * f64::INFINITY;
    }
    // Hex / octal / binary literals take no sign.
    if s.len() > 2
        && (s.starts_with("0x")
            || s.starts_with("0X")
            || s.starts_with("0o")
            || s.starts_with("0O")
            || s.starts_with("0b")
            || s.starts_with("0B"))
    {
        let radix = match s.as_bytes()[1] | 0x20 {
            b'x' => 16,
            b'o' => 8,
            _ => 2,
        };
        let digits = &s[2..];
        if digits.is_empty() || !digits.chars().all(|c| c.is_digit(radix)) {
            return f64::NAN;
        }
        return kahvi_parser::number::radix_digits_to_f64(digits, radix);
    }
    // StrUnsignedDecimalLiteral: digits [. digits] [e [+-] digits], or . digits.
    let bytes = body.as_bytes();
    let mut i = 0;
    let mut int_digits = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
        int_digits += 1;
    }
    let mut frac_digits = 0;
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
            frac_digits += 1;
        }
    }
    if int_digits == 0 && frac_digits == 0 {
        return f64::NAN;
    }
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        i += 1;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == start {
            return f64::NAN;
        }
    }
    if i != bytes.len() {
        return f64::NAN;
    }
    let mut text = body.to_string();
    if text.starts_with('.') {
        text.insert(0, '0');
    }
    text = text.replace(".e", ".0e").replace(".E", ".0E");
    if text.ends_with('.') {
        text.push('0');
    }
    sign * text.parse::<f64>().unwrap_or(f64::NAN)
}

/// ToIntegerOrInfinity(argument) (§7.1.5)
pub fn to_integer_or_infinity(agent: &mut Agent, v: Value) -> JsResult<f64> {
    // 1. Let number be ? ToNumber(argument).
    let number = to_number(agent, v)?;
    Ok(integer_or_infinity(number))
}

/// Steps 2-6 of ToIntegerOrInfinity on a number already converted.
pub fn integer_or_infinity(number: f64) -> f64 {
    // 2. If number is one of NaN, +0𝔽, or -0𝔽, return 0.
    if number.is_nan() || number == 0.0 {
        return 0.0;
    }
    // 3-4. If number is +∞𝔽 / -∞𝔽, return +∞ / -∞.
    if number.is_infinite() {
        return number;
    }
    // 5. Return truncate(ℝ(number)).
    number.trunc() + 0.0
}

fn to_int_bits(n: f64, bits: u32, signed: bool) -> f64 {
    // If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !n.is_finite() || n == 0.0 {
        return 0.0;
    }
    // Let int be truncate(ℝ(number)); let intN bit be int modulo 2^N.
    let modulus = 2f64.powi(bits as i32);
    let int = n.trunc();
    let mut v = int % modulus;
    if v < 0.0 {
        v += modulus;
    }
    // If signed and int bit ≥ 2^(N-1), return int bit - 2^N.
    if signed && v >= modulus / 2.0 {
        v -= modulus;
    }
    v + 0.0
}

/// ToInt32(argument) (§7.1.6)
pub fn to_int32(agent: &mut Agent, v: Value) -> JsResult<i32> {
    let n = to_number(agent, v)?;
    Ok(f64_to_int32(n))
}

pub fn f64_to_int32(n: f64) -> i32 {
    to_int_bits(n, 32, true) as i32
}

/// ToUint32(argument) (§7.1.7)
pub fn to_uint32(agent: &mut Agent, v: Value) -> JsResult<u32> {
    let n = to_number(agent, v)?;
    Ok(f64_to_uint32(n))
}

pub fn f64_to_uint32(n: f64) -> u32 {
    to_int_bits(n, 32, false) as u32
}

/// ToInt16 / ToUint16 / ToInt8 / ToUint8 (§7.1.8-7.1.11)
pub fn to_int16(agent: &mut Agent, v: Value) -> JsResult<i16> {
    Ok(to_int_bits(to_number(agent, v)?, 16, true) as i16)
}
pub fn to_uint16(agent: &mut Agent, v: Value) -> JsResult<u16> {
    Ok(to_int_bits(to_number(agent, v)?, 16, false) as u16)
}
pub fn to_int8(agent: &mut Agent, v: Value) -> JsResult<i8> {
    Ok(to_int_bits(to_number(agent, v)?, 8, true) as i8)
}
pub fn to_uint8(agent: &mut Agent, v: Value) -> JsResult<u8> {
    Ok(to_int_bits(to_number(agent, v)?, 8, false) as u8)
}

/// ToUint8Clamp(argument) (§7.1.12)
pub fn to_uint8_clamp(agent: &mut Agent, v: Value) -> JsResult<u8> {
    let n = to_number(agent, v)?;
    // 2. If number is NaN, return +0𝔽.
    if n.is_nan() {
        return Ok(0);
    }
    // 3-4. Clamp to 0..255.
    if n <= 0.0 {
        return Ok(0);
    }
    if n >= 255.0 {
        return Ok(255);
    }
    // 5-9. Round half to even.
    let f = n.floor();
    if f + 0.5 < n {
        return Ok((f + 1.0) as u8);
    }
    if n < f + 0.5 {
        return Ok(f as u8);
    }
    Ok(if f % 2.0 == 1.0 {
        (f + 1.0) as u8
    } else {
        f as u8
    })
}

/// ToString(argument) (§7.1.17)
pub fn to_string(agent: &mut Agent, v: Value) -> JsResult<JsString> {
    match v {
        Value::String(s) => Ok(s),
        Value::Symbol(_) => {
            Err(agent.throw_type_error("Cannot convert a Symbol value to a string"))
        }
        Value::Undefined => Ok(agent.names.undefined),
        Value::Null => Ok(agent.string("null")),
        Value::Boolean(b) => Ok(agent.string(if b { "true" } else { "false" })),
        Value::Number(n) => Ok(number_to_js_string(agent, n)),
        Value::BigInt(b) => Ok(crate::ops::bigint::to_js_string(agent, b, 10)),
        Value::Object(_) => {
            // 9. Let primValue be ? ToPrimitive(argument, string).
            let prim = to_primitive(agent, v, PreferredType::String)?;
            // 11. Return ? ToString(primValue).
            to_string(agent, prim)
        }
    }
}

/// Number::toString(x, 10) as a String value.
pub fn number_to_js_string(agent: &mut Agent, n: f64) -> JsString {
    let s = kahvi_parser::number::number_to_string(n);
    agent.string(&s)
}

/// ToObject(argument) (§7.1.18)
pub fn to_object(agent: &mut Agent, v: Value) -> JsResult<Object> {
    let (proto, kind) = match v {
        Value::Undefined | Value::Null => {
            return Err(agent.throw_type_error("Cannot convert undefined or null to object"));
        }
        Value::Object(o) => return Ok(o),
        Value::Boolean(b) => ("%Boolean.prototype%", ObjectKind::BooleanObject(b)),
        Value::Number(n) => ("%Number.prototype%", ObjectKind::NumberObject(n)),
        Value::String(s) => {
            // A String exotic object with a "length" own property.
            let proto = agent.intrinsic("%String.prototype%");
            return Ok(string_create(agent, s, proto));
        }
        Value::Symbol(s) => ("%Symbol.prototype%", ObjectKind::SymbolObject(s)),
        Value::BigInt(b) => ("%BigInt.prototype%", ObjectKind::BigIntObject(b)),
    };
    let proto = agent.intrinsic(proto);
    Ok(agent.object_with_kind(Some(proto), kind))
}

/// StringCreate(value, prototype) (§10.4.3.4)
pub fn string_create(agent: &mut Agent, s: JsString, proto: Object) -> Object {
    let o = agent.object_with_kind(Some(proto), ObjectKind::StringObject(s));
    let len = agent.str_len(s);
    let key = PropertyKey::String(agent.names.length);
    agent.obj_mut(o).properties.insert(
        key,
        crate::object::Property::Data {
            value: Value::Number(len as f64),
            writable: false,
            enumerable: false,
            configurable: false,
        },
    );
    o
}

/// ToPropertyKey(argument) (§7.1.19)
pub fn to_property_key(agent: &mut Agent, v: Value) -> JsResult<PropertyKey> {
    // A Number is already primitive, and key_from_index is ToString's key
    // for it, without formatting and interning an array index.
    if let Value::Number(n) = v {
        return Ok(agent.key_from_index(n));
    }
    // 1. Let key be ? ToPrimitive(argument, string).
    let key = to_primitive(agent, v, PreferredType::String)?;
    // 2. If key is a Symbol, return key.
    if let Value::Symbol(s) = key {
        return Ok(PropertyKey::Symbol(s));
    }
    // 3. Return ! ToString(key).
    let s = to_string(agent, key)?;
    Ok(agent.key_from_string(s))
}

/// ToLength(argument) (§7.1.20)
pub fn to_length(agent: &mut Agent, v: Value) -> JsResult<f64> {
    // 1. Let len be ? ToIntegerOrInfinity(argument).
    let len = to_integer_or_infinity(agent, v)?;
    // 2. If len ≤ 0, return +0𝔽.
    if len <= 0.0 {
        return Ok(0.0);
    }
    // 3. Return 𝔽(min(len, 2^53 - 1)).
    Ok(len.min(9007199254740991.0))
}

/// CanonicalNumericIndexString(argument) (§7.1.21)
pub fn canonical_numeric_index_string(agent: &mut Agent, key: PropertyKey) -> Option<f64> {
    match key {
        PropertyKey::Index(i) => Some(i as f64),
        PropertyKey::Symbol(_) => None,
        PropertyKey::String(s) => {
            // 1. If argument is "-0", return -0𝔽.
            if agent.str(s) == [b'-' as u16, b'0' as u16] {
                return Some(-0.0);
            }
            // 2. Let n be ! ToNumber(argument).
            let n = string_to_number(agent.str(s));
            // 3. If ! ToString(n) is argument, return n.
            let back = number_to_js_string(agent, n);
            (back == s).then_some(n)
        }
    }
}

/// ToIndex(value) (§7.1.22)
pub fn to_index(agent: &mut Agent, v: Value) -> JsResult<f64> {
    // 1. Let integer be ? ToIntegerOrInfinity(value).
    let integer = to_integer_or_infinity(agent, v)?;
    // 2. If integer is not in the inclusive interval from 0 to 2^53 - 1, throw a RangeError exception.
    if !(0.0..=9007199254740991.0).contains(&integer) {
        return Err(agent.throw_range_error("index out of range"));
    }
    // 3. Return integer.
    Ok(integer)
}
