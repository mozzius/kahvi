//! get Intl.Locale.prototype.caseFirst
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.caseFirst

use kahvi_core::prelude::*;

/// get Intl.Locale.prototype.caseFirst
pub(crate) fn prototype_get_case_first(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This property only exists if %Intl.Locale%.[[LocaleExtensionKeys]] contains *"kf"*.
    // `Intl.Locale.prototype.caseFirst` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return _loc_.[[CaseFirst]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Intl.Locale.prototype.caseFirst"))
}
