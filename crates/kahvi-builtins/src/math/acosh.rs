//! Math.acosh ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.acosh

use kahvi_core::prelude::*;

/// Math.acosh
pub(crate) fn acosh(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the inverse hyperbolic cosine of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is either *NaN* or *+∞*_𝔽, return _n_.
    // 3. If _n_ is *1*_𝔽, return *+0*_𝔽.
    // 4. If _n_ < *1*_𝔽, return *NaN*.
    // 5. Return an implementation-approximated Number value representing the inverse hyperbolic cosine of ℝ(_n_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.acosh"))
}
