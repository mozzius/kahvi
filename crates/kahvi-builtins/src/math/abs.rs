//! Math.abs ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.abs

use kahvi_core::prelude::*;

/// Math.abs
pub(crate) fn abs(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the absolute value of _x_; the result has the same magnitude as _x_ but has positive sign.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is *NaN*, return *NaN*.
    // 3. If _n_ is *-0*_𝔽, return *+0*_𝔽.
    // 4. If _n_ is *-∞*_𝔽, return *+∞*_𝔽.
    // 5. If _n_ < *-0*_𝔽, return -_n_.
    // 6. Return _n_.
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is *NaN*, return *NaN*.
    if n.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 3. If _n_ is *-0*_𝔽, return *+0*_𝔽.
    if n == 0.0 {
        return Ok(Value::Number(0.0));
    }
    // 4. If _n_ is *-∞*_𝔽, return *+∞*_𝔽.
    if n == f64::NEG_INFINITY {
        return Ok(Value::Number(f64::INFINITY));
    }
    // 5. If _n_ < *-0*_𝔽, return -_n_.
    if n < 0.0 {
        return Ok(Value::Number(-n));
    }
    // 6. Return _n_.
    Ok(Value::Number(n))
}
