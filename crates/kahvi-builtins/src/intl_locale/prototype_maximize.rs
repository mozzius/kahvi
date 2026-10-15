//! Intl.Locale.prototype.maximize ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.maximize

use kahvi_core::prelude::*;

/// Intl.Locale.prototype.maximize
pub(crate) fn prototype_maximize(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Let _maximal_ be the result of the Add Likely Subtags algorithm applied to _loc_.[[Locale]]. If an error is signaled, set _maximal_ to _loc_.[[Locale]].
    // 4. Return ! Construct(%Intl.Locale%, « _maximal_ »).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Locale.prototype.maximize"))
}
