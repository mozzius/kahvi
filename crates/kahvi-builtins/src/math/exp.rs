//! Math.exp ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.exp

use kahvi_core::prelude::*;

/// Math.exp
pub(crate) fn exp(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the exponential function of _x_ (_e_ raised to the power of _x_, where _e_ is the base of the natural logarithms).
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is either *NaN* or *+∞*_𝔽, return _n_.
    if n.is_nan() || n == f64::INFINITY {
        return Ok(Value::Number(n));
    }
    // 3. If _n_ is either *+0*_𝔽 or *-0*_𝔽, return *1*_𝔽.
    if n == 0.0 {
        return Ok(Value::Number(1.0));
    }
    // 4. If _n_ is *-∞*_𝔽, return *+0*_𝔽.
    if n == f64::NEG_INFINITY {
        return Ok(Value::Number(0.0));
    }
    // 5. Return an implementation-approximated Number value representing the exponential function of ℝ(_n_).
    Ok(Value::Number(n.exp()))
}
