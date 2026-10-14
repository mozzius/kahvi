//! Math.sumPrecise ( _items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.sumprecise

use kahvi_core::ops::IteratorKind;
use kahvi_core::prelude::*;

/// Limbs of the exact accumulator. A Number is a multiple of 2^-1074 below
/// 2^1024, so each term fits in 2098 bits; the sum of fewer than 2^53 terms
/// needs at most 2151 bits, and the top bit of the 2240 is the sign.
const LIMBS: usize = 35;

/// The states of the sum (`state` in the spec steps).
#[derive(Clone, Copy, PartialEq)]
enum State {
    MinusZero,
    Finite,
    PlusInfinity,
    MinusInfinity,
    NotANumber,
}

/// The exact sum of Numbers, as a two's complement integer in units of 2^-1074.
/// The spec's _sum_ is computed exactly, so no rounding happens until the end.
struct ExactSum {
    limbs: [u64; LIMBS],
}

impl ExactSum {
    /// Sets _sum_ to _sum_ + ℝ(_n_) for a finite Number _n_.
    fn add_finite(&mut self, n: f64) {
        let bits = n.to_bits();
        let negative = bits >> 63 == 1;
        let biased = ((bits >> 52) & 0x7ff) as usize;
        let fraction = bits & ((1 << 52) - 1);
        // |n| = m * 2^(shift - 1074), with m < 2^53.
        let (m, shift) = if biased == 0 {
            (fraction, 0)
        } else {
            (fraction | (1 << 52), biased - 1)
        };
        if m == 0 {
            return;
        }
        let mut term = [0u64; LIMBS];
        let limb = shift / 64;
        let offset = shift % 64;
        term[limb] = m << offset;
        if offset > 0 {
            term[limb + 1] = m >> (64 - offset);
        }
        if negative {
            negate(&mut term);
        }
        // Wrapping addition is exact here: the true sum always fits in LIMBS limbs.
        let mut carry = false;
        for (acc, &t) in self.limbs.iter_mut().zip(term.iter()) {
            let (s1, c1) = acc.overflowing_add(t);
            let (s2, c2) = s1.overflowing_add(u64::from(carry));
            *acc = s2;
            carry = c1 || c2;
        }
    }

    /// 𝔽(_sum_): the exact sum rounded to the nearest Number, ties to even.
    fn to_number(&self) -> f64 {
        let mut magnitude = self.limbs;
        let negative = magnitude[LIMBS - 1] >> 63 == 1;
        if negative {
            negate(&mut magnitude);
        }
        // The sum is exactly zero: 𝔽(0) is +0.
        let Some(h) = highest_bit(&magnitude) else {
            return 0.0;
        };
        let bit = |i: usize| (magnitude[i / 64] >> (i % 64)) & 1;
        // Keep 53 significant bits, or every bit for a subnormal result.
        let shift = h.saturating_sub(52);
        let mut mantissa: u64 = 0;
        for i in (shift..=h).rev() {
            mantissa = (mantissa << 1) | bit(i);
        }
        // The biased exponent of 2^(h - 1074) (0 for subnormals).
        let mut biased = if h >= 52 { h as u64 - 51 } else { 0 };
        if shift > 0 {
            // Round half to even on the discarded bits.
            let round = bit(shift - 1) == 1;
            let sticky = (0..shift - 1).any(|i| bit(i) == 1);
            if round && (sticky || mantissa & 1 == 1) {
                mantissa += 1;
            }
            if mantissa == 1 << 53 {
                mantissa >>= 1;
                biased += 1;
            }
        }
        let value = if biased >= 2047 {
            f64::INFINITY
        } else {
            f64::from_bits((biased << 52) | (mantissa & ((1 << 52) - 1)))
        };
        if negative { -value } else { value }
    }
}

/// Two's complement negation of a limb array.
fn negate(limbs: &mut [u64; LIMBS]) {
    let mut carry = true;
    for limb in limbs.iter_mut() {
        *limb = !*limb;
        if carry {
            let (v, c) = limb.overflowing_add(1);
            *limb = v;
            carry = c;
        }
    }
}

/// The index of the highest set bit, or None when every bit is clear.
fn highest_bit(limbs: &[u64; LIMBS]) -> Option<usize> {
    limbs
        .iter()
        .enumerate()
        .rev()
        .find(|(_, limb)| **limb != 0)
        .map(|(i, limb)| i * 64 + 63 - limb.leading_zeros() as usize)
}

/// Math.sumPrecise
pub(crate) fn sum_precise(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // Given an iterable of Numbers, this function sums each value in the iterable and returns their sum. If any value is not a Number it throws a *TypeError* exception.
    // It performs the following steps when called:
    //
    // 1. Perform ? RequireObjectCoercible(_items_).
    let items = args.get_or_undefined(0);
    ops::require_object_coercible(agent, items)?;
    // 2. Let _iteratorRecord_ be ? GetIterator(_items_, ~sync~).
    let mut iterator_record = ops::get_iterator(agent, items, IteratorKind::Sync)?;
    // The iterator may collect garbage while it runs: it stays rooted.
    let roots = [
        Value::Object(iterator_record.iterator),
        iterator_record.next_method,
    ];
    agent.with_roots(&roots, |agent| {
        // 3. Let _state_ be ~minus-zero~.
        let mut state = State::MinusZero;
        // 4. Let _sum_ be 0.
        let mut sum = ExactSum { limbs: [0; LIMBS] };
        // 5. Let _count_ be 0.
        let mut count = 0.0;
        // 6. Let _next_ be ~not-started~.
        // 7. Repeat, while _next_ is not ~done~,
        loop {
            // a. Set _next_ to ? IteratorStepValue(_iteratorRecord_).
            // b. If _next_ is not ~done~, then
            let Some(next) = ops::iterator_step_value_rooted(agent, &mut iterator_record)? else {
                break;
            };
            // i. If _count_ ≥ 2^53 - 1, then
            if count >= 9_007_199_254_740_991.0 {
                // 1. NOTE: This step is not expected to be reached in practice and is included only so that implementations may rely on inputs being “reasonably sized” without violating this specification.
                // 2. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
                let error = agent.throw_range_error("Math.sumPrecise: too many values");
                // 3. Return ? IteratorClose(_iteratorRecord_, _error_).
                return ops::iterator_close(agent, &iterator_record, Err(error));
            }
            // ii. If _next_ is not a Number, then
            let Value::Number(n) = next else {
                // 1. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
                let error = agent.throw_type_error("Math.sumPrecise: value is not a Number");
                // 2. Return ? IteratorClose(_iteratorRecord_, _error_).
                return ops::iterator_close(agent, &iterator_record, Err(error));
            };
            // iii. Let _n_ be _next_.
            // iv. If _state_ is not ~not-a-number~, then
            if state != State::NotANumber {
                // 1. If _n_ is *NaN*, then
                if n.is_nan() {
                    // a. Set _state_ to ~not-a-number~.
                    state = State::NotANumber;
                // 2. Else if _n_ is *+∞*_𝔽, then
                } else if n == f64::INFINITY {
                    // a. If _state_ is ~minus-infinity~, set _state_ to ~not-a-number~.
                    // b. Else, set _state_ to ~plus-infinity~.
                    state = if state == State::MinusInfinity {
                        State::NotANumber
                    } else {
                        State::PlusInfinity
                    };
                // 3. Else if _n_ is *-∞*_𝔽, then
                } else if n == f64::NEG_INFINITY {
                    // a. If _state_ is ~plus-infinity~, set _state_ to ~not-a-number~.
                    // b. Else, set _state_ to ~minus-infinity~.
                    state = if state == State::PlusInfinity {
                        State::NotANumber
                    } else {
                        State::MinusInfinity
                    };
                // 4. Else if _n_ is not *-0*_𝔽 and _state_ is either ~minus-zero~ or ~finite~, then
                } else if !(n == 0.0 && n.is_sign_negative())
                    && matches!(state, State::MinusZero | State::Finite)
                {
                    // a. Set _state_ to ~finite~.
                    state = State::Finite;
                    // b. Set _sum_ to _sum_ + ℝ(_n_).
                    sum.add_finite(n);
                }
            }
            // v. Set _count_ to _count_ + 1.
            count += 1.0;
        }
        match state {
            // 8. If _state_ is ~not-a-number~, return *NaN*.
            State::NotANumber => Ok(Value::Number(f64::NAN)),
            // 9. If _state_ is ~plus-infinity~, return *+∞*_𝔽.
            State::PlusInfinity => Ok(Value::Number(f64::INFINITY)),
            // 10. If _state_ is ~minus-infinity~, return *-∞*_𝔽.
            State::MinusInfinity => Ok(Value::Number(f64::NEG_INFINITY)),
            // 11. If _state_ is ~minus-zero~, return *-0*_𝔽.
            State::MinusZero => Ok(Value::Number(-0.0)),
            // 12. Return 𝔽(_sum_).
            State::Finite => Ok(Value::Number(sum.to_number())),
        }
    })
}
