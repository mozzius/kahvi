//! Math.atanh ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.atanh

use kahvi_core::prelude::*;

/// Math.atanh
pub(crate) fn atanh(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the inverse hyperbolic tangent of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    // 3. If _n_ > *1*_𝔽 or _n_ < *-1*_𝔽, return *NaN*.
    // 4. If _n_ is *1*_𝔽, return *+∞*_𝔽.
    // 5. If _n_ is *-1*_𝔽, return *-∞*_𝔽.
    // 6. Return an implementation-approximated Number value representing the inverse hyperbolic tangent of ℝ(_n_).
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    if n.is_nan() || n == 0.0 {
        return Ok(Value::Number(n));
    }
    // 3. If _n_ > *1*_𝔽 or _n_ < *-1*_𝔽, return *NaN*.
    if n > 1.0 || n < -1.0 {
        return Ok(Value::Number(f64::NAN));
    }
    // 4. If _n_ is *1*_𝔽, return *+∞*_𝔽.
    if n == 1.0 {
        return Ok(Value::Number(f64::INFINITY));
    }
    // 5. If _n_ is *-1*_𝔽, return *-∞*_𝔽.
    if n == -1.0 {
        return Ok(Value::Number(f64::NEG_INFINITY));
    }
    // 6. Return an implementation-approximated Number value representing the inverse hyperbolic tangent of ℝ(_n_).
    Ok(Value::Number(n.atanh()))
}
