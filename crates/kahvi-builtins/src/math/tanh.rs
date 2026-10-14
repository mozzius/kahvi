//! Math.tanh ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.tanh

use kahvi_core::prelude::*;

/// Math.tanh
pub(crate) fn tanh(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the hyperbolic tangent of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    // 3. If _n_ is *+∞*_𝔽, return *1*_𝔽.
    // 4. If _n_ is *-∞*_𝔽, return *-1*_𝔽.
    // 5. Return an implementation-approximated Number value representing the hyperbolic tangent of ℝ(_n_).
    //
    // NOTE:
    // The value of `Math.tanh(x)` is the same as the value of `(Math.exp(x) - Math.exp(-x)) / (Math.exp(x) + Math.exp(-x))`.
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    if n.is_nan() || n == 0.0 {
        return Ok(Value::Number(n));
    }
    // 3. If _n_ is *+∞*_𝔽, return *1*_𝔽.
    if n == f64::INFINITY {
        return Ok(Value::Number(1.0));
    }
    // 4. If _n_ is *-∞*_𝔽, return *-1*_𝔽.
    if n == f64::NEG_INFINITY {
        return Ok(Value::Number(-1.0));
    }
    // 5. Return an implementation-approximated Number value representing the hyperbolic tangent of ℝ(_n_).
    Ok(Value::Number(n.tanh()))
}
