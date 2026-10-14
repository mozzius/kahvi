//! Math.log1p ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.log1p

use kahvi_core::prelude::*;

/// Math.log1p
pub(crate) fn log1p(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the natural logarithm of 1 + _x_. The result is computed in a way that is accurate even when the value of x is close to zero.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, *-0*_𝔽, or *+∞*_𝔽, return _n_.
    // 3. If _n_ is *-1*_𝔽, return *-∞*_𝔽.
    // 4. If _n_ < *-1*_𝔽, return *NaN*.
    // 5. Return an implementation-approximated Number value representing ln(1 + ℝ(_n_)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.log1p"))
}
