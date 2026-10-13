//! Set.prototype.values ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.values

use kahvi_core::prelude::*;

/// Set.prototype.values
pub(crate) fn prototype_values(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Return ? CreateSetIterator(_set_, ~value~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.values"))
}
