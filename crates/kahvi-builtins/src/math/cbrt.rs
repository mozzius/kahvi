//! Math.cbrt ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.cbrt

use kahvi_core::prelude::*;

/// Math.cbrt
pub(crate) fn cbrt(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the cube root of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is not a finite Number or _n_ is either *+0*_𝔽 or *-0*_𝔽, return _n_.
    // 3. Return an implementation-approximated Number value representing the cube root of ℝ(_n_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.cbrt"))
}
