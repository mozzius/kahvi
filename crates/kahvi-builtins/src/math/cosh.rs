//! Math.cosh ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.cosh

use kahvi_core::prelude::*;

/// Math.cosh
pub(crate) fn cosh(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the hyperbolic cosine of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is *NaN*, return *NaN*.
    // 3. If _n_ is either *+∞*_𝔽 or *-∞*_𝔽, return *+∞*_𝔽.
    // 4. If _n_ is either *+0*_𝔽 or *-0*_𝔽, return *1*_𝔽.
    // 5. Return an implementation-approximated Number value representing the hyperbolic cosine of ℝ(_n_).
    //
    // NOTE:
    // The value of `Math.cosh(x)` is the same as the value of `(Math.exp(x) + Math.exp(-x)) / 2`.
    let _ = (this, new_target);
    // 1. Let _n_ be ? ToNumber(_x_).
    let x = args.get_or_undefined(0);
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is *NaN*, return *NaN*.
    if n.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 3. If _n_ is either *+∞*_𝔽 or *-∞*_𝔽, return *+∞*_𝔽.
    if n.is_infinite() {
        return Ok(Value::Number(f64::INFINITY));
    }
    // 4. If _n_ is either *+0*_𝔽 or *-0*_𝔽, return *1*_𝔽.
    if n == 0.0 {
        return Ok(Value::Number(1.0));
    }
    // 5. Return an implementation-approximated Number value representing the hyperbolic cosine of ℝ(_n_).
    Ok(Value::Number(n.cosh()))
}
