//! Math.tan ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.tan

use kahvi_core::prelude::*;

/// Math.tan
pub(crate) fn tan(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the tangent of _x_. The argument is expressed in radians.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    if n.is_nan() || n == 0.0 {
        return Ok(Value::Number(n));
    }
    // 3. If _n_ is either *+∞*_𝔽 or *-∞*_𝔽, return *NaN*.
    if n.is_infinite() {
        return Ok(Value::Number(f64::NAN));
    }
    // 4. Return an implementation-approximated Number value representing the tangent of ℝ(_n_).
    Ok(Value::Number(n.tan()))
}
