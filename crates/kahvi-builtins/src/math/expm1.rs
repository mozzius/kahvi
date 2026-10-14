//! Math.expm1 ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.expm1

use kahvi_core::prelude::*;

/// Math.expm1
pub(crate) fn expm1(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the result of subtracting 1 from the exponential function of _x_ (_e_ raised to the power of _x_, where _e_ is the base of the natural logarithms). The result is computed in a way that is accurate even when the value of _x_ is close to 0.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, *-0*_𝔽, or *+∞*_𝔽, return _n_.
    // 3. If _n_ is *-∞*_𝔽, return *-1*_𝔽.
    // 4. Let _exp_ be the exponential function of ℝ(_n_).
    // 5. Return an implementation-approximated Number value representing _exp_ - 1.
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, *-0*_𝔽, or *+∞*_𝔽, return _n_.
    if n.is_nan() || n == 0.0 || n == f64::INFINITY {
        return Ok(Value::Number(n));
    }
    // 3. If _n_ is *-∞*_𝔽, return *-1*_𝔽.
    if n == f64::NEG_INFINITY {
        return Ok(Value::Number(-1.0));
    }
    // 4. Let _exp_ be the exponential function of ℝ(_n_).
    // 5. Return an implementation-approximated Number value representing _exp_ - 1.
    Ok(Value::Number(n.exp_m1()))
}
