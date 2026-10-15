//! Intl.Locale.prototype.getCalendars ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.getCalendars

use kahvi_core::prelude::*;

/// Intl.Locale.prototype.getCalendars
pub(crate) fn prototype_get_calendars(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `getCalendars` method is called, the following steps are taken:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return CalendarsOfLocale(_loc_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Locale.prototype.getCalendars"))
}
