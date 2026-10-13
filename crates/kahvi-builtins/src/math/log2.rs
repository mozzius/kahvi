//! Math.log2 ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.log2

use kahvi_core::prelude::*;

/// Math.log2
pub(crate) fn log2(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the base 2 logarithm of _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is either *NaN* or *+∞*_𝔽, return _n_.
    // 3. If _n_ is *1*_𝔽, return *+0*_𝔽.
    // 4. If _n_ is either *+0*_𝔽 or *-0*_𝔽, return *-∞*_𝔽.
    // 5. If _n_ < *-0*_𝔽, return *NaN*.
    // 6. Return an implementation-approximated Number value representing log2(ℝ(_n_)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.log2"))
}
