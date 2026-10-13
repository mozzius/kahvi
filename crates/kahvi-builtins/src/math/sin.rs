//! Math.sin ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.sin

use kahvi_core::prelude::*;

/// Math.sin
pub(crate) fn sin(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the sine of _x_. The argument is expressed in radians.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    // 3. If _n_ is either *+∞*_𝔽 or *-∞*_𝔽, return *NaN*.
    // 4. Return an implementation-approximated Number value representing the sine of ℝ(_n_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.sin"))
}
