//! Math.trunc ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.trunc

use kahvi_core::prelude::*;

/// Math.trunc
pub(crate) fn trunc(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the integral part of the number _x_, removing any fractional digits. If _x_ is already integral, the result is _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    let n = ops::to_number(agent, x)?;
    // 2. If _n_ is not a finite Number or _n_ is either *+0*_𝔽 or *-0*_𝔽, return _n_.
    if !n.is_finite() || n == 0.0 {
        return Ok(Value::Number(n));
    }
    // 3. If _n_ < *1*_𝔽 and _n_ > *+0*_𝔽, return *+0*_𝔽.
    if n < 1.0 && n > 0.0 {
        return Ok(Value::Number(0.0));
    }
    // 4. If _n_ < *-0*_𝔽 and _n_ > *-1*_𝔽, return *-0*_𝔽.
    if n < -0.0 && n > -1.0 {
        return Ok(Value::Number(-0.0));
    }
    // 5. Return the integral Number nearest _n_ in the direction of *+0*_𝔽.
    Ok(Value::Number(n.trunc()))
}
