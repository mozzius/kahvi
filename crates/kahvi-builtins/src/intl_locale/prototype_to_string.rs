//! Intl.Locale.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.toString

use kahvi_core::prelude::*;

/// Intl.Locale.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return _loc_.[[Locale]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Locale.prototype.toString"))
}
