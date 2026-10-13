//! Object.isFrozen ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.isfrozen

use kahvi_core::prelude::*;

/// Object.isFrozen
pub(crate) fn is_frozen(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _obj_ is not an Object, return *true*.
    // 2. Return ? TestIntegrityLevel(_obj_, ~frozen~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.isFrozen"))
}
