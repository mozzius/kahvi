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
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is not a finite Number, return *NaN*.
    // 3. If _n_ is either *+0*_𝔽 or *-0*_𝔽, return *1*_𝔽.
    // 4. Return an implementation-approximated Number value representing the cosine of ℝ(_n_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.cos"))
}
