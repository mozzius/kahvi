//! Math.clz32 ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.clz32

use kahvi_core::prelude::*;

/// Math.clz32
pub(crate) fn clz32(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _n_ be ? ToUint32(_x_).
    // 2. Let _p_ be the number of leading zero bits in the unsigned 32-bit binary representation of _n_.
    // 3. Return 𝔽(_p_).
    //
    // NOTE:
    // If _n_ is either *+0*_𝔽 or *-0*_𝔽, this method returns *32*_𝔽. If the most significant bit of the 32-bit binary encoding of _n_ is 1, this method returns *+0*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.clz32"))
}
