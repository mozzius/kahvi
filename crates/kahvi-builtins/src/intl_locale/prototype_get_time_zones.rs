//! Intl.Locale.prototype.getTimeZones ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.getTimeZones

use kahvi_core::prelude::*;

/// Intl.Locale.prototype.getTimeZones
pub(crate) fn prototype_get_time_zones(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `getTimeZones` method is called, the following steps are taken:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return TimeZonesOfLocale(_loc_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Locale.prototype.getTimeZones"))
}
