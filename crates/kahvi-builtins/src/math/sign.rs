//! Math.sign ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.sign

use kahvi_core::prelude::*;

/// Math.sign
pub(crate) fn sign(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the sign of _x_, indicating whether _x_ is positive, negative, or zero.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return _n_.
    // 3. If _n_ < *-0*_𝔽, return *-1*_𝔽.
    // 4. Return *1*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.sign"))
}
