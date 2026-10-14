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
        self.gc.allocated += 1;
        let id = match self.gc.free_bigints.pop() {
            Some(i) => {
                self.gc.is_free_bigint.remove(i);
                self.bigints[i as usize] = v;
                i
            }
            None => {
                self.bigints.push(v);
                (self.bigints.len() - 1) as u32
            }
        };
        if self.gc.pinning {
            self.gc.pinned_bigints.insert(id);
        } else if let Some(unit) = &mut self.gc.compiling {
            unit.bigints.push(id);
        }
        BigInt(id)
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

// ---- Arithmetic (§6.1.6.2) ------------------------------------------------------------------

/// The most bits a BigInt result may have; beyond it the operation throws a RangeError.
pub const MAX_BITS: u64 = 1 << 26;

fn mag_cmp(a: &[u32], b: &[u32]) -> Ordering {
    a.len()
        .cmp(&b.len())
        .then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

fn mag_trim(mut v: Vec<u32>) -> Vec<u32> {
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}

fn mag_add(a: &[u32], b: &[u32]) -> Vec<u32> {
    let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = Vec::with_capacity(a.len() + 1);
    let mut carry = 0u64;
    for (i, &x) in a.iter().enumerate() {
        let s = x as u64 + *b.get(i).unwrap_or(&0) as u64 + carry;
        out.push(s as u32);
        carry = s >> 32;
    }
    if carry > 0 {
        out.push(carry as u32);
    }
    out
}

/// a - b for a ≥ b.
fn mag_sub(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len());
    let mut borrow = 0i64;
    for (i, &x) in a.iter().enumerate() {
        let mut d = x as i64 - *b.get(i).unwrap_or(&0) as i64 - borrow;
        borrow = if d < 0 {
            d += 1 << 32;
            1
        } else {
            0
        };
        out.push(d as u32);
    }
    mag_trim(out)
}

fn mag_mul(a: &[u32], b: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u32; a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        let mut carry = 0u64;
        for (j, &y) in b.iter().enumerate() {
            let t = x as u64 * y as u64 + out[i + j] as u64 + carry;
            out[i + j] = t as u32;
            carry = t >> 32;
        }
        out[i + b.len()] = carry as u32;
    }
    mag_trim(out)
}

fn mag_shl(a: &[u32], bits: u64) -> Vec<u32> {
    if a.is_empty() {
        return Vec::new();
    }
    let (limbs, rem) = ((bits / 32) as usize, (bits % 32) as u32);
    let mut out = vec![0u32; limbs];
    let mut carry = 0u32;
    for &x in a {
        out.push((x << rem) | carry);
        carry = if rem == 0 { 0 } else { x >> (32 - rem) };
    }
    if carry > 0 {
        out.push(carry);
    }
    mag_trim(out)
}

/// a >> bits, and whether any 1 bits were shifted out.
fn mag_shr(a: &[u32], bits: u64) -> (Vec<u32>, bool) {
    let limbs = (bits / 32) as usize;
    let rem = (bits % 32) as u32;
    if limbs >= a.len() {
        return (Vec::new(), !a.is_empty());
    }
    let mut lost = a[..limbs].iter().any(|&x| x != 0);
    if rem > 0 {
        lost |= a[limbs] & ((1 << rem) - 1) != 0;
    }
    let mut out = Vec::with_capacity(a.len() - limbs);
    for i in limbs..a.len() {
        let lo = a[i] >> rem;
        let hi = if rem == 0 {
            0
        } else {
            a.get(i + 1).map_or(0, |&n| n << (32 - rem))
        };
        out.push(lo | hi);
    }
    (mag_trim(out), lost)
}

/// Quotient and remainder of magnitudes (Knuth's Algorithm D); b is non-zero.
fn mag_divrem(a: &[u32], b: &[u32]) -> (Vec<u32>, Vec<u32>) {
    if mag_cmp(a, b) == Ordering::Less {
        return (Vec::new(), a.to_vec());
    }
    if b.len() == 1 {
        let d = b[0] as u64;
        let mut q = vec![0u32; a.len()];
        let mut r = 0u64;
        for i in (0..a.len()).rev() {
            let cur = (r << 32) | a[i] as u64;
            q[i] = (cur / d) as u32;
            r = cur % d;
        }
        return (mag_trim(q), mag_trim(vec![r as u32]));
    }
    // Normalize so the divisor's top limb has its high bit set.
    let s = b[b.len() - 1].leading_zeros() as u64;
    let b = mag_shl(b, s);
    let mut u = mag_shl(a, s);
    u.push(0);
    if u.len() < a.len() + 1 {
        u.resize(a.len() + 1, 0);
    }
    let n = b.len();
    let m = u.len() - n - 1;
    let mut q = vec![0u32; m + 1];
    let base = 1u64 << 32;
    for j in (0..=m).rev() {
        let num = ((u[j + n] as u64) << 32) | u[j + n - 1] as u64;
        let mut qhat = num / b[n - 1] as u64;
        let mut rhat = num % b[n - 1] as u64;
        while qhat >= base || qhat * b[n - 2] as u64 > ((rhat << 32) | u[j + n - 2] as u64) {
            qhat -= 1;
            rhat += b[n - 1] as u64;
            if rhat >= base {
                break;
            }
        }
        // u[j..j+n+1] -= qhat * b
        let mut borrow = 0i64;
        let mut carry = 0u64;
        for i in 0..n {
            let p = qhat * b[i] as u64 + carry;
            carry = p >> 32;
            let t = u[i + j] as i64 - borrow - (p & 0xFFFF_FFFF) as i64;
            u[i + j] = t as u32;
            borrow = if t < 0 { 1 } else { 0 };
        }
        let t = u[j + n] as i64 - borrow - carry as i64;
        u[j + n] = t as u32;
        if t < 0 {
            // qhat was one too large: add b back.
            qhat -= 1;
            let mut c = 0u64;
            for i in 0..n {
                let s = u[i + j] as u64 + b[i] as u64 + c;
                u[i + j] = s as u32;
                c = s >> 32;
            }
            u[j + n] = u[j + n].wrapping_add(c as u32);
        }
        q[j] = qhat as u32;
    }
    let (r, _) = mag_shr(&mag_trim(u[..n].to_vec()), s);
    (mag_trim(q), r)
}

/// The value as n-limb two's complement.
fn to_twos(v: &BigIntValue, n: usize) -> Vec<u32> {
    let mut out = v.magnitude.clone();
    out.resize(n, 0);
    if v.negative {
        for x in out.iter_mut() {
            *x = !*x;
        }
        for x in out.iter_mut() {
            let (s, c) = x.overflowing_add(1);
            *x = s;
            if !c {
                break;
            }
        }
    }
    out
}

fn from_twos(mut v: Vec<u32>) -> BigIntValue {
    let negative = v.last().is_some_and(|&x| x & 0x8000_0000 != 0);
    if negative {
        for x in v.iter_mut() {
            *x = !*x;
        }
        for x in v.iter_mut() {
            let (s, c) = x.overflowing_add(1);
            *x = s;
            if !c {
                break;
            }
        }
    }
    BigIntValue {
        negative,
        magnitude: v,
    }
    .normalize()
}

impl BigIntValue {
    fn signed(negative: bool, magnitude: Vec<u32>) -> Self {
        BigIntValue {
            negative,
            magnitude,
        }
        .normalize()
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude.is_empty()
    }

    /// The number of bits of the magnitude.
    pub fn bit_length(&self) -> u64 {
        match self.magnitude.last() {
            None => 0,
            Some(&top) => self.magnitude.len() as u64 * 32 - top.leading_zeros() as u64,
        }
    }

    /// BigInt::unaryMinus(x)
    pub fn neg(&self) -> Self {
        BigIntValue::signed(!self.negative, self.magnitude.clone())
    }

    /// BigInt::add(x, y)
    pub fn add(&self, o: &Self) -> Self {
        if self.negative == o.negative {
            return BigIntValue::signed(self.negative, mag_add(&self.magnitude, &o.magnitude));
        }
        match mag_cmp(&self.magnitude, &o.magnitude) {
            Ordering::Less => {
                BigIntValue::signed(o.negative, mag_sub(&o.magnitude, &self.magnitude))
            }
            _ => BigIntValue::signed(self.negative, mag_sub(&self.magnitude, &o.magnitude)),
        }
    }

    /// BigInt::subtract(x, y)
    pub fn sub(&self, o: &Self) -> Self {
        self.add(&o.neg())
    }

    /// BigInt::multiply(x, y)
    pub fn mul(&self, o: &Self) -> Self {
        BigIntValue::signed(
            self.negative != o.negative,
            mag_mul(&self.magnitude, &o.magnitude),
        )
    }

    /// BigInt::divide and BigInt::remainder: truncating quotient, remainder with the dividend's sign.
    /// `None` for a zero divisor.
    pub fn divrem(&self, o: &Self) -> Option<(Self, Self)> {
        if o.is_zero() {
            return None;
        }
        let (q, r) = mag_divrem(&self.magnitude, &o.magnitude);
        Some((
            BigIntValue::signed(self.negative != o.negative, q),
            BigIntValue::signed(self.negative, r),
        ))
    }

    /// x × 2^n for n ≥ 0, or floor(x / 2^-n) for n < 0 (BigInt::leftShift).
    pub fn shift_left(&self, n: i128) -> Self {
        if n >= 0 {
            return BigIntValue::signed(self.negative, mag_shl(&self.magnitude, n as u64));
        }
        let bits = n.unsigned_abs().min(u64::MAX as u128) as u64;
        let (q, lost) = mag_shr(&self.magnitude, bits);
        let q = BigIntValue::signed(self.negative, q);
        // Rounding toward -∞: a negative value that lost 1 bits goes one further down.
        if self.negative && lost {
            q.sub(&BigIntValue::from_i128(1))
        } else {
            q
        }
    }

    fn bitwise(&self, o: &Self, f: impl Fn(u32, u32) -> u32) -> Self {
        let n = self.magnitude.len().max(o.magnitude.len()) + 1;
        let a = to_twos(self, n);
        let b = to_twos(o, n);
        from_twos(a.iter().zip(&b).map(|(&x, &y)| f(x, y)).collect())
    }

    /// BigInt::bitwiseAND(x, y)
    pub fn and(&self, o: &Self) -> Self {
        self.bitwise(o, |x, y| x & y)
    }

    /// BigInt::bitwiseOR(x, y)
    pub fn or(&self, o: &Self) -> Self {
        self.bitwise(o, |x, y| x | y)
    }

    /// BigInt::bitwiseXOR(x, y)
    pub fn xor(&self, o: &Self) -> Self {
        self.bitwise(o, |x, y| x ^ y)
    }

    /// BigInt::bitwiseNOT(x): -x - 1.
    pub fn not(&self) -> Self {
        self.neg().sub(&BigIntValue::from_i128(1))
    }

    /// 𝔽(ℝ(x)): the Number nearest the value, ties to even.
    pub fn to_f64(&self) -> f64 {
        let bits = self.bit_length();
        if bits <= 64 {
            let m = self
                .magnitude
                .iter()
                .rev()
                .fold(0u128, |acc, &l| (acc << 32) | l as u128);
            let v = m as f64; // u128 → f64 rounds to nearest, ties to even.
            return if self.negative { -v } else { v };
        }
        if bits > 1024 {
            return if self.negative {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            };
        }
        // Keep the top 64 bits plus a sticky bit for the rest, then let the
        // integer-to-float conversion round once.
        let shift = bits - 64;
        let (top, lost) = mag_shr(&self.magnitude, shift);
        let top = top
            .iter()
            .rev()
            .fold(0u128, |acc, &l| (acc << 32) | l as u128) as u64;
        let with_sticky = ((top as u128) << 1) | lost as u128;
        let v = (with_sticky as f64) * 2f64.powi(shift as i32 - 1);
        if self.negative { -v } else { v }
    }
}

/// BigInt::exponentiate(base, exponent) (§6.1.6.2.3)
pub fn exponentiate(
    agent: &mut Agent,
    base: &BigIntValue,
    exponent: &BigIntValue,
) -> JsResult<BigIntValue> {
    // 1. If exponent < 0n, throw a RangeError exception.
    if exponent.negative {
        return Err(agent.throw_range_error("BigInt negative exponent"));
    }
    // 2. If base is 0n and exponent is 0n, return 1n.
    if exponent.is_zero() {
        return Ok(BigIntValue::from_i128(1));
    }
    // 3. Return base raised to the power exponent.
    if base.is_zero() || base.magnitude == [1] {
        let negative = base.negative && exponent.magnitude[0] & 1 == 1;
        return Ok(BigIntValue::signed(negative, base.magnitude.clone()));
    }
    if exponent.bit_length() > 32 || base.bit_length().saturating_mul(exponent.low_u64()) > MAX_BITS
    {
        return Err(agent.throw_range_error("Maximum BigInt size exceeded"));
    }
    let mut e = exponent.low_u64();
    let mut result = BigIntValue::from_i128(1);
    let mut b = base.clone();
    while e > 0 {
        if e & 1 == 1 {
            result = result.mul(&b);
        }
        e >>= 1;
        if e > 0 {
            b = b.mul(&b);
        }
    }
    Ok(result)
}

/// The BigInt cases of ApplyStringOrNumericBinaryOperator (Table 39) and the shift operators.
pub fn binary(agent: &mut Agent, op: &str, a: BigInt, b: BigInt) -> JsResult<BigInt> {
    let (x, y) = (agent.bigint(a).clone(), agent.bigint(b).clone());
    let too_big = |agent: &mut Agent| agent.throw_range_error("Maximum BigInt size exceeded");
    let r = match op {
        "add" => x.add(&y),
        "subtract" => x.sub(&y),
        "multiply" => {
            if x.bit_length() + y.bit_length() > MAX_BITS {
                return Err(too_big(agent));
            }
            x.mul(&y)
        }
        // BigInt::divide: if y is 0n, throw a RangeError exception.
        "divide" | "remainder" => match x.divrem(&y) {
            None => return Err(agent.throw_range_error("Division by zero")),
            Some((q, r)) => {
                if op == "divide" {
                    q
                } else {
                    r
                }
            }
        },
        "exponentiate" => exponentiate(agent, &x, &y)?,
        "leftShift" | "signedRightShift" => {
            // signedRightShift(x, y) is leftShift(x, -y).
            // |y|, saturated: no BigInt can be shifted by 2^64 bits and survive.
            let m = if y.bit_length() > 64 {
                i128::MAX / 2
            } else {
                y.magnitude
                    .iter()
                    .rev()
                    .fold(0i128, |acc, &l| (acc << 32) | l as i128)
            };
            let n = if y.negative { -m } else { m };
            let n = if op == "signedRightShift" { -n } else { n };
            if n > 0 && x.bit_length() as i128 + n > MAX_BITS as i128 && !x.is_zero() {
                return Err(too_big(agent));
            }
            x.shift_left(n)
        }
        // BigInt::unsignedRightShift: throw a TypeError exception.
        "unsignedRightShift" => {
            return Err(
                agent.throw_type_error("BigInts have no unsigned right shift, use >> instead")
            );
        }
        "bitwiseAND" => x.and(&y),
        "bitwiseOR" => x.or(&y),
        "bitwiseXOR" => x.xor(&y),
        _ => unreachable!("not a BigInt operator: {op}"),
    };
    Ok(agent.new_bigint(r))
}

// ---- The BigInt built-ins' abstract operations (§21.2) ---------------------------------------

impl BigIntValue {
    /// ℝ(number) for an integral, finite f64.
    fn from_integral_f64(number: f64) -> Self {
        let bits = number.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i64;
        let fraction = bits & ((1u64 << 52) - 1);
        // number = significand × 2^(exponent - 1075); subnormals have exponent 1.
        let (significand, exponent) = if exponent == 0 {
            (fraction, 1)
        } else {
            (fraction | (1u64 << 52), exponent)
        };
        let limbs = mag_trim(vec![significand as u32, (significand >> 32) as u32]);
        let shift = exponent - 1075;
        let magnitude = if shift >= 0 {
            mag_shl(&limbs, shift as u64)
        } else {
            // Exact: the number is integral, so no set bit is shifted out.
            mag_shr(&limbs, (-shift) as u64).0
        };
        BigIntValue::signed(number < 0.0, magnitude)
    }

    /// The value modulo 2^bits as `bits.div_ceil(32)` limbs of two's complement,
    /// bits above `bits` cleared.
    fn modulo_pow2_limbs(&self, bits: u64) -> Vec<u32> {
        let n = bits.div_ceil(32) as usize;
        let mut limbs = to_twos(self, n);
        if !bits.is_multiple_of(32) {
            limbs[n - 1] &= (1u32 << (bits % 32)) - 1;
        }
        limbs
    }
}

/// NumberToBigInt(number) (§21.2.1.1.1)
pub fn number_to_bigint(agent: &mut Agent, number: f64) -> JsResult<BigInt> {
    // 1. If number is not an integral Number, throw a RangeError exception.
    if !number.is_finite() || number.trunc() != number {
        return Err(agent.throw_range_error("cannot convert a non-integral Number to a BigInt"));
    }
    // 2. Return ℤ(ℝ(number)).
    Ok(agent.new_bigint(BigIntValue::from_integral_f64(number)))
}

/// ℤ(ℝ(bigint) modulo 2^bits), BigInt.asUintN's step 3 (§21.2.2.2).
///
/// A negative BigInt modulo 2^bits has about `bits` bits; beyond MAX_BITS that
/// is a RangeError, as for every other BigInt result too large to represent.
pub fn as_uint_n(agent: &mut Agent, bits: u64, bigint: BigInt) -> JsResult<BigInt> {
    let v = agent.bigint(bigint);
    if !v.negative && v.bit_length() <= bits {
        return Ok(bigint);
    }
    if bits > MAX_BITS {
        return Err(agent.throw_range_error("BigInt is too large"));
    }
    let limbs = v.modulo_pow2_limbs(bits);
    Ok(agent.new_bigint(BigIntValue::signed(false, limbs)))
}

/// BigInt.asIntN's steps 3-5 (§21.2.2.1): ℝ(bigint) modulo 2^bits, less 2^bits
/// when that remainder is at least 2^(bits - 1).
pub fn as_int_n(agent: &mut Agent, bits: u64, bigint: BigInt) -> JsResult<BigInt> {
    let v = agent.bigint(bigint);
    if bits == 0 {
        return Ok(agent.new_bigint(BigIntValue::zero()));
    }
    // A value of fewer than bits bits is its own remainder's signed reading.
    if v.bit_length() < bits {
        return Ok(bigint);
    }
    let mut limbs = v.modulo_pow2_limbs(bits);
    let n = limbs.len();
    let top_bit = (bits - 1) % 32;
    if limbs[n - 1] & (1u32 << top_bit) != 0 && top_bit != 31 {
        // remainder ≥ 2^(bits-1): sign-extend so the limbs read as remainder - 2^bits.
        limbs[n - 1] |= !((1u32 << (top_bit + 1)) - 1);
    }
    Ok(agent.new_bigint(from_twos(limbs)))
}
