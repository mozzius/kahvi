//! get Intl.Locale.prototype.numeric
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.numeric

use kahvi_core::prelude::*;

/// get Intl.Locale.prototype.numeric
pub(crate) fn prototype_get_numeric(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This property only exists if %Intl.Locale%.[[LocaleExtensionKeys]] contains *"kn"*.
    // `Intl.Locale.prototype.numeric` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return _loc_.[[Numeric]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Intl.Locale.prototype.numeric"))
}
