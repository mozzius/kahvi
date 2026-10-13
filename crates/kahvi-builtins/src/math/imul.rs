//! Math.imul ( _x_, _y_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.imul

use kahvi_core::prelude::*;

/// Math.imul
pub(crate) fn imul(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _a_ be ℝ(? ToUint32(_x_)).
    // 2. Let _b_ be ℝ(? ToUint32(_y_)).
    // 3. Let _product_ be (_a_ × _b_) modulo 2^32.
    // 4. If _product_ ≥ 2^31, return 𝔽(_product_ - 2^32).
    // 5. Return 𝔽(_product_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.imul"))
}
