//! Set.prototype.keys ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.keys

use kahvi_core::prelude::*;

/// Set.prototype.keys
pub(crate) fn prototype_keys(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The initial value of the *"keys"* property is %Set.prototype.values%, defined in [sec-set.prototype.values].
    // NOTE:
    // For iteration purposes, a Set appears similar to a Map where each entry has the same value for its key and value.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.keys"))
}
