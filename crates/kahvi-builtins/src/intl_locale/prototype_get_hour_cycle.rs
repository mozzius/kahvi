//! get Intl.Locale.prototype.hourCycle
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.hourCycle

use kahvi_core::prelude::*;

/// get Intl.Locale.prototype.hourCycle
pub(crate) fn prototype_get_hour_cycle(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Intl.Locale.prototype.hourCycle` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return _loc_.[[HourCycle]].
    let _ = (args, new_target);
    crate::support::intl_locale::locale_string_getter(agent, this, |loc| loc.hour_cycle.clone())
}
