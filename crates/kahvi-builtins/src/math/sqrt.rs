//! Math.sqrt ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.sqrt

use kahvi_core::prelude::*;

/// Math.sqrt
pub(crate) fn sqrt(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the square root of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, *-0*_𝔽, or *+∞*_𝔽, return _n_.
    // 3. If _n_ < *-0*_𝔽, return *NaN*.
    // 4. Return 𝔽(the square root of ℝ(_n_)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.sqrt"))
}
