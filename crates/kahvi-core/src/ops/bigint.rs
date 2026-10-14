//! BigInt values (ECMA-262 §6.1.6.2).
//!
//! Representation, parsing, printing and comparison are here; the
//! arithmetic operations (BigInt::add etc.) are not implemented yet.

use std::cmp::Ordering;

use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{BigInt, JsString};

/// Sign and magnitude, magnitude as little-endian base-2^32 limbs with no
/// trailing zero limbs (so zero is an empty magnitude, never negative).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BigIntValue {
    pub negative: bool,
    pub magnitude: Vec<u32>,
}

impl BigIntValue {
    pub fn zero() -> Self {
        BigIntValue {
            negative: false,
            magnitude: Vec::new(),
        }
    }

    fn normalize(mut self) -> Self {
        while self.magnitude.last() == Some(&0) {
            self.magnitude.pop();
        }
        if self.magnitude.is_empty() {
            self.negative = false;
        }
        self
    }

    /// Parses decimal digits (no sign).
    pub fn from_decimal(digits: &str) -> Option<Self> {
        let mut mag: Vec<u32> = Vec::new();
        for c in digits.chars() {
            let d = c.to_digit(10)? as u64;
            let mut carry = d;
            for limb in mag.iter_mut() {
                let v = *limb as u64 * 10 + carry;
                *limb = v as u32;
                carry = v >> 32;
            }
            if carry > 0 {
                mag.push(carry as u32);
            }
        }
        Some(
            BigIntValue {
                negative: false,
                magnitude: mag,
            }
            .normalize(),
        )
    }

    pub fn to_string_radix(&self, radix: u32) -> String {
        if self.magnitude.is_empty() {
            return "0".into();
        }
        let mut mag = self.magnitude.clone();
        let mut digits = Vec::new();
        while !mag.is_empty() {
            let mut rem: u64 = 0;
            for limb in mag.iter_mut().rev() {
                let v = (rem << 32) | *limb as u64;
                *limb = (v / radix as u64) as u32;
                rem = v % radix as u64;
            }
            digits.push(std::char::from_digit(rem as u32, radix).unwrap_or('0'));
            while mag.last() == Some(&0) {
                mag.pop();
            }
        }
        if self.negative {
            digits.push('-');
        }
        digits.iter().rev().collect()
    }

    fn cmp_magnitude(a: &[u32], b: &[u32]) -> Ordering {
        a.len()
            .cmp(&b.len())
            .then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => Self::cmp_magnitude(&self.magnitude, &other.magnitude),
            (true, true) => Self::cmp_magnitude(&other.magnitude, &self.magnitude),
        }
    }

    /// The exact value as f64 when representable, else an approximation (for comparisons
    /// with non-integral numbers only the sign/ordering matters).
    pub fn to_f64_approx(&self) -> f64 {
        let mut v = 0.0f64;
        for limb in self.magnitude.iter().rev() {
            v = v * 4294967296.0 + *limb as f64;
        }
        if self.negative { -v } else { v }
    }
}

impl Agent {
    pub fn new_bigint(&mut self, v: BigIntValue) -> BigInt {
        self.bigints.push(v);
        BigInt((self.bigints.len() - 1) as u32)
    }

    pub fn bigint(&self, b: BigInt) -> &BigIntValue {
        &self.bigints[b.0 as usize]
    }
}

pub fn is_zero(agent: &Agent, b: BigInt) -> bool {
    agent.bigint(b).magnitude.is_empty()
}

pub fn equal(agent: &Agent, a: BigInt, b: BigInt) -> bool {
    agent.bigint(a) == agent.bigint(b)
}

pub fn less_than(agent: &Agent, a: BigInt, b: BigInt) -> bool {
    agent.bigint(a).compare(agent.bigint(b)) == Ordering::Less
}

/// Compares a BigInt with a Number; `None` if the number is NaN.
pub fn compare_number(agent: &Agent, a: BigInt, n: f64) -> Option<Ordering> {
    if n.is_nan() {
        return None;
    }
    if n == f64::INFINITY {
        return Some(Ordering::Less);
    }
    if n == f64::NEG_INFINITY {
        return Some(Ordering::Greater);
    }
    let a = agent.bigint(a);
    // Compare against the integer part, then the fraction breaks ties.
    let int = n.trunc();
    let int_big =
        BigIntValue::from_decimal(&format!("{:.0}", int.abs())).unwrap_or_else(BigIntValue::zero);
    let int_big = BigIntValue {
        negative: int < 0.0,
        ..int_big
    }
    .normalize();
    match a.compare(&int_big) {
        Ordering::Equal => Some(if n > int {
            Ordering::Less
        } else if n < int {
            Ordering::Greater
        } else {
            Ordering::Equal
        }),
        o => Some(o),
    }
}

/// StringToBigInt(str) (§7.1.14); `None` for syntax errors.
pub fn string_to_bigint(agent: &mut Agent, s: JsString) -> Option<BigInt> {
    let text = String::from_utf16(agent.str(s)).ok()?;
    let t = text.trim_matches(|c: char| {
        kahvi_parser::lexer::is_whitespace(c) || kahvi_parser::lexer::is_line_terminator(c)
    });
    let value = if t.is_empty() {
        BigIntValue::zero()
    } else if let Some(rest) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        parse_radix(rest, 16)?
    } else if let Some(rest) = t.strip_prefix("0o").or_else(|| t.strip_prefix("0O")) {
        parse_radix(rest, 8)?
    } else if let Some(rest) = t.strip_prefix("0b").or_else(|| t.strip_prefix("0B")) {
        parse_radix(rest, 2)?
    } else {
        let (neg, digits) = match t.as_bytes()[0] {
            b'-' => (true, &t[1..]),
            b'+' => (false, &t[1..]),
            _ => (false, t),
        };
        if digits.is_empty() {
            return None;
        }
        let v = BigIntValue::from_decimal(digits)?;
        BigIntValue { negative: neg, ..v }.normalize()
    };
    Some(agent.new_bigint(value))
}

fn parse_radix(digits: &str, radix: u32) -> Option<BigIntValue> {
    if digits.is_empty() {
        return None;
    }
    let mut mag: Vec<u32> = Vec::new();
    for c in digits.chars() {
        let d = c.to_digit(radix)? as u64;
        let mut carry = d;
        for limb in mag.iter_mut() {
            let v = *limb as u64 * radix as u64 + carry;
            *limb = v as u32;
            carry = v >> 32;
        }
        if carry > 0 {
            mag.push(carry as u32);
        }
    }
    Some(
        BigIntValue {
            negative: false,
            magnitude: mag,
        }
        .normalize(),
    )
}

/// BigInt::toString(x, radix) as a String value.
pub fn to_js_string(agent: &mut Agent, b: BigInt, radix: u32) -> JsString {
    let s = agent.bigint(b).to_string_radix(radix);
    agent.string(&s)
}

/// A BigInt from a literal's decimal digits.
pub fn from_literal(agent: &mut Agent, digits: &str) -> BigInt {
    let v = BigIntValue::from_decimal(digits).unwrap_or_else(BigIntValue::zero);
    agent.new_bigint(v)
}

/// Placeholder for BigInt arithmetic until it is implemented.
pub fn unsupported<T>(agent: &mut Agent, op: &str) -> JsResult<T> {
    Err(agent.not_implemented(&format!("BigInt::{op}")))
}

impl BigIntValue {
    /// The value modulo 2^64, as the two's-complement bit pattern.
    pub fn low_u64(&self) -> u64 {
        let lo = self.magnitude.first().copied().unwrap_or(0) as u64;
        let hi = self.magnitude.get(1).copied().unwrap_or(0) as u64;
        let m = lo | (hi << 32);
        if self.negative { m.wrapping_neg() } else { m }
    }

    pub fn from_i128(v: i128) -> Self {
        let negative = v < 0;
        let mut m = v.unsigned_abs();
        let mut magnitude = Vec::new();
        while m > 0 {
            magnitude.push(m as u32);
            m >>= 32;
        }
        BigIntValue {
            negative,
            magnitude,
        }
        .normalize()
    }
}

/// ToBigInt(argument) (§7.1.13)
pub fn to_bigint(agent: &mut Agent, argument: crate::value::Value) -> JsResult<BigInt> {
    use crate::value::Value;
    // 1. Let prim be ? ToPrimitive(argument, number).
    let prim = super::to_primitive(agent, argument, super::PreferredType::Number)?;
    // 2. Return the value that prim corresponds to in Table 12.
    match prim {
        Value::Undefined | Value::Null => Err(agent.throw_type_error("cannot convert to a BigInt")),
        Value::Boolean(b) => Ok(agent.new_bigint(BigIntValue::from_i128(b as i128))),
        Value::BigInt(b) => Ok(b),
        Value::Number(_) => Err(agent.throw_type_error("cannot convert a Number to a BigInt")),
        Value::String(s) => match string_to_bigint(agent, s) {
            Some(b) => Ok(b),
            None => Err(agent.throw_syntax_error("cannot convert string to a BigInt")),
        },
        Value::Symbol(_) => Err(agent.throw_type_error("cannot convert a Symbol to a BigInt")),
        Value::Object(_) => unreachable!("ToPrimitive returns a primitive"),
    }
}

/// ToBigInt64(argument) (§7.1.15)
pub fn to_big_int64(agent: &mut Agent, argument: crate::value::Value) -> JsResult<i64> {
    // 1. Let n be ? ToBigInt(argument).
    let n = to_bigint(agent, argument)?;
    // 2-3. Let int64bit be ℝ(n) modulo 2^64; as a signed value.
    Ok(agent.bigint(n).low_u64() as i64)
}

/// ToBigUint64(argument) (§7.1.16)
pub fn to_big_uint64(agent: &mut Agent, argument: crate::value::Value) -> JsResult<u64> {
    // 1. Let n be ? ToBigInt(argument).
    let n = to_bigint(agent, argument)?;
    // 2. Let int64bit be ℝ(n) modulo 2^64.
    Ok(agent.bigint(n).low_u64())
}

/// ℤ(v): a BigInt for an integer.
pub fn from_i128(agent: &mut Agent, v: i128) -> BigInt {
    agent.new_bigint(BigIntValue::from_i128(v))
}
