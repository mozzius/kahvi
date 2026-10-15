//! Intl.Locale.prototype.getWeekInfo ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.getWeekInfo

use kahvi_core::prelude::*;

/// Intl.Locale.prototype.getWeekInfo
pub(crate) fn prototype_get_week_info(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `getWeekInfo` method is called, the following steps are taken:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Let _info_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. Let _weekInfo_ be WeekInfoOfLocale(_loc_).
    // 5. Perform ! CreateDataPropertyOrThrow(_info_, *"firstDay"*, _weekInfo_.[[FirstDay]]).
    // 6. Perform ! CreateDataPropertyOrThrow(_info_, *"weekend"*, CreateArrayFromList(_weekInfo_.[[Weekend]])).
    // 7. Return _info_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Locale.prototype.getWeekInfo"))
}
