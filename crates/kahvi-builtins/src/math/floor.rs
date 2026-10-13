//! Math.floor ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.floor

use kahvi_core::prelude::*;

/// Math.floor
pub(crate) fn floor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the greatest (closest to +∞) integral Number value that is not greater than _x_. If _x_ is already an integral Number, the result is _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is not a finite Number or _n_ is either *+0*_𝔽 or *-0*_𝔽, return _n_.
    // 3. If _n_ < *1*_𝔽 and _n_ > *+0*_𝔽, return *+0*_𝔽.
    // 4. If _n_ is an integral Number, return _n_.
    // 5. Return the greatest (closest to +∞) integral Number value that is not greater than _n_.
    //
    // NOTE:
    // The value of `Math.floor(x)` is the same as the value of `-Math.ceil(-x)`.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.floor"))
}
