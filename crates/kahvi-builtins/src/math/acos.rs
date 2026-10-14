//! Math.acos ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.acos

use kahvi_core::prelude::*;

/// Math.acos
pub(crate) fn acos(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the inverse cosine of _x_. The result is expressed in radians and is in the inclusive interval from *+0*_𝔽 to 𝔽(π).
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is *NaN*, _n_ > *1*_𝔽, or _n_ < *-1*_𝔽, return *NaN*.
    // 3. If _n_ is *1*_𝔽, return *+0*_𝔽.
    // 4. Return an implementation-approximated Number value representing the inverse cosine of ℝ(_n_).
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is *NaN*, _n_ > *1*_𝔽, or _n_ < *-1*_𝔽, return *NaN*.
    if n.is_nan() || n > 1.0 || n < -1.0 {
        return Ok(Value::Number(f64::NAN));
    }
    // 3. If _n_ is *1*_𝔽, return *+0*_𝔽.
    if n == 1.0 {
        return Ok(Value::Number(0.0));
    }
    // 4. Return an implementation-approximated Number value representing the inverse cosine of ℝ(_n_).
    Ok(Value::Number(n.acos()))
}
