//! Math.sinh ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.sinh

use kahvi_core::prelude::*;

/// Math.sinh
pub(crate) fn sinh(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the hyperbolic sine of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is not a finite Number or _n_ is either *+0*_𝔽 or *-0*_𝔽, return _n_.
    // 3. Return an implementation-approximated Number value representing the hyperbolic sine of ℝ(_n_).
    //
    // NOTE:
    // The value of `Math.sinh(x)` is the same as the value of `(Math.exp(x) - Math.exp(-x)) / 2`.
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is not a finite Number or _n_ is either *+0*_𝔽 or *-0*_𝔽, return _n_.
    if !n.is_finite() || n == 0.0 {
        return Ok(Value::Number(n));
    }
    // 3. Return an implementation-approximated Number value representing the hyperbolic sine of ℝ(_n_).
    Ok(Value::Number(n.sinh()))
}
