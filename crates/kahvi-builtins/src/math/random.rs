//! Math.random ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.random

use kahvi_core::prelude::*;

/// Math.random
pub(crate) fn random(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns a Number value with positive sign, greater than or equal to *+0*_𝔽 but strictly less than *1*_𝔽, chosen randomly or pseudo randomly with approximately uniform distribution over that range, using an implementation-defined algorithm or strategy.
    // Each `Math.random` function created for distinct realms must produce a distinct sequence of values from successive calls.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.random"))
}
