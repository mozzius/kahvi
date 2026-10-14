//! Math.cosh ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.cosh

use kahvi_core::prelude::*;

/// Math.cosh
pub(crate) fn cosh(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the hyperbolic cosine of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is *NaN*, return *NaN*.
    // 3. If _n_ is either *+∞*_𝔽 or *-∞*_𝔽, return *+∞*_𝔽.
    // 4. If _n_ is either *+0*_𝔽 or *-0*_𝔽, return *1*_𝔽.
    // 5. Return an implementation-approximated Number value representing the hyperbolic cosine of ℝ(_n_).
    //
    // NOTE:
    // The value of `Math.cosh(x)` is the same as the value of `(Math.exp(x) + Math.exp(-x)) / 2`.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.cosh"))
}
