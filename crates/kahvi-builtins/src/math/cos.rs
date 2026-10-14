//! Math.cos ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.cos

use kahvi_core::prelude::*;

/// Math.cos
pub(crate) fn cos(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the cosine of _x_. The argument is expressed in radians.
    // It performs the following steps when called:
    //
    let _ = (this, new_target);
    // 1. Let _n_ be ? ToNumber(_x_).
    let x = args.get_or_undefined(0);
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is not a finite Number, return *NaN*.
    if !n.is_finite() {
        return Ok(Value::Number(f64::NAN));
    }
    // 3. If _n_ is either *+0*_𝔽 or *-0*_𝔽, return *1*_𝔽.
    if n == 0.0 {
        return Ok(Value::Number(1.0));
    }
    // 4. Return an implementation-approximated Number value representing the cosine of ℝ(_n_).
    Ok(Value::Number(n.cos()))
}
