//! Intl.Locale.prototype.minimize ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.minimize

use kahvi_core::prelude::*;

/// Intl.Locale.prototype.minimize
pub(crate) fn prototype_minimize(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Let _minimal_ be the result of the Remove Likely Subtags algorithm applied to _loc_.[[Locale]]. If an error is signaled, set _minimal_ to _loc_.[[Locale]].
    // 4. Return ! Construct(%Intl.Locale%, « _minimal_ »).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Locale.prototype.minimize"))
}
