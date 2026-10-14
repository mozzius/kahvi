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
    let _ = (this, new_target);
    let x = args.get_or_undefined(0);
    let y = args.get_or_undefined(1);
    // 1. Let _a_ be ℝ(? ToUint32(_x_)).
    let a = ops::to_uint32(agent, x)?;
    // 2. Let _b_ be ℝ(? ToUint32(_y_)).
    let b = ops::to_uint32(agent, y)?;
    // 3. Let _product_ be (_a_ × _b_) modulo 2^32.
    let product = a.wrapping_mul(b);
    // 4. If _product_ ≥ 2^31, return 𝔽(_product_ - 2^32).
    // 5. Return 𝔽(_product_).
    // Reinterpreting the 32-bit product as signed yields exactly that value.
    Ok(Value::Number(f64::from(product as i32)))
}
