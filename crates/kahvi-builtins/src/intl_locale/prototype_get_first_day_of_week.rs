//! get Intl.Locale.prototype.firstDayOfWeek
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.firstDayOfWeek

use kahvi_core::prelude::*;

/// get Intl.Locale.prototype.firstDayOfWeek
pub(crate) fn prototype_get_first_day_of_week(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Intl.Locale.prototype.firstDayOfWeek` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return _loc_.[[FirstDayOfWeek]].
    let _ = (args, new_target);
    crate::support::intl_locale::locale_string_getter(agent, this, |loc| {
        loc.first_day_of_week.clone()
    })
}
