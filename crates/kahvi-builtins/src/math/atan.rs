//! Math.atan ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.atan

use kahvi_core::prelude::*;

/// Math.atan
pub(crate) fn atan(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the inverse tangent of _x_. The result is expressed in radians and is in the inclusive interval from 𝔽(-π / 2) to 𝔽(π / 2).
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    // 3. If _n_ is *+∞*_𝔽, return an implementation-approximated Number value representing π / 2.
    // 4. If _n_ is *-∞*_𝔽, return an implementation-approximated Number value representing -π / 2.
    // 5. Return an implementation-approximated Number value representing the inverse tangent of ℝ(_n_).
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    if n.is_nan() || n == 0.0 {
        return Ok(Value::Number(n));
    }
    // 3. If _n_ is *+∞*_𝔽, return an implementation-approximated Number value representing π / 2.
    if n == f64::INFINITY {
        return Ok(Value::Number(std::f64::consts::FRAC_PI_2));
    }
    // 4. If _n_ is *-∞*_𝔽, return an implementation-approximated Number value representing -π / 2.
    if n == f64::NEG_INFINITY {
        return Ok(Value::Number(-std::f64::consts::FRAC_PI_2));
    }
    // 5. Return an implementation-approximated Number value representing the inverse tangent of ℝ(_n_).
    Ok(Value::Number(n.atan()))
}
