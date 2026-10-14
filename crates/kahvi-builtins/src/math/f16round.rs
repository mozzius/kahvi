//! Math.f16round ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.f16round

use kahvi_core::prelude::*;

/// Math.f16round
pub(crate) fn f16round(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is *NaN*, return *NaN*.
    // 3. If _n_ is one of *+0*_𝔽, *-0*_𝔽, *+∞*_𝔽, or *-∞*_𝔽, return _n_.
    // 4. Let _n16_ be the result of converting _n_ to IEEE 754-2019 binary16 format using roundTiesToEven mode.
    // 5. Let _n64_ be the result of converting _n16_ to IEEE 754-2019 binary64 format.
    // 6. Return the ECMAScript Number value corresponding to _n64_.
    //
    // NOTE:
    // This operation is not the same as casting to binary32 and then to binary16 because of the possibility of double-rounding: consider the number _k_ = *1.00048828125000022204*_𝔽, for example, for which Math.f16round(_k_) is *1.0009765625*_𝔽, but Math.f16round(Math.fround(_k_)) is *1*_𝔽.
    // Not all platforms provide native support for casting from binary64 to binary16. There are various libraries which can provide this, including the MIT-licensed half library. Alternatively, it is possible to first cast from binary64 to binary32 under roundTiesToEven and then check whether the result could lead to incorrect double-rounding. Such cases can be handled explicitly by adjusting the mantissa of the binary32 value so that it is the value which would be produced by performing the initial cast under roundTiesToOdd. Casting the adjusted value to binary16 under roundTiesToEven then produces the correct value.
    let _ = (this, args, new_target);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, args.get_or_undefined(0))?;
    // 2. If _n_ is *NaN*, return *NaN*.
    if n.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 3. If _n_ is one of *+0*_𝔽, *-0*_𝔽, *+∞*_𝔽, or *-∞*_𝔽, return _n_.
    if n == 0.0 || n.is_infinite() {
        return Ok(Value::Number(n));
    }
    // 4. Let _n16_ be the result of converting _n_ to IEEE 754-2019 binary16 format using roundTiesToEven mode.
    // 5. Let _n64_ be the result of converting _n16_ to IEEE 754-2019 binary64 format.
    // 6. Return the ECMAScript Number value corresponding to _n64_.
    Ok(Value::Number(round_to_binary16(n)))
}

/// The smallest positive normal binary16 value, 2^-14.
const MIN_NORMAL_BINARY16: f64 = 6.103515625e-5;

/// Rounds a finite, non-zero Number to the nearest binary16 value (ties to even),
/// returning the result as the exactly equal binary64 value.
fn round_to_binary16(n: f64) -> f64 {
    let magnitude = n.abs();
    // binary16 keeps 11 significant bits. A value in [2^e, 2^(e+1)) with e >= -14
    // is spaced 2^(e-10) apart; smaller values are subnormal and spaced 2^-24 apart.
    let quantum_exponent = if magnitude < MIN_NORMAL_BINARY16 {
        -24
    } else {
        // `magnitude` is a normal binary64 value here, so its biased exponent is in the
        // exponent field of its bit pattern.
        let biased = ((magnitude.to_bits() >> 52) & 0x7ff) as i32;
        biased - 1023 - 10
    };
    // Dividing and multiplying by a power of two is exact in binary64, so the only
    // rounding is the integer rounding below.
    let quantum = power_of_two(quantum_exponent);
    let scaled = magnitude / quantum;
    let floor = scaled.floor();
    let fraction = scaled - floor;
    let rounded = if fraction > 0.5 || (fraction == 0.5 && floor % 2.0 != 0.0) {
        floor + 1.0
    } else {
        floor
    };
    let result = rounded * quantum;
    // The largest binary16 value is 65504; anything that rounds to 2^16 or more overflows to infinity.
    if result >= 65536.0 {
        return f64::INFINITY.copysign(n);
    }
    result.copysign(n)
}

/// Returns 2^exponent for an exponent in the normal binary64 range.
fn power_of_two(exponent: i32) -> f64 {
    f64::from_bits(((exponent + 1023) as u64) << 52)
}
