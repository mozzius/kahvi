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
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.acos"))
}
