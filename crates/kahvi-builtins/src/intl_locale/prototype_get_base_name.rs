//! get Intl.Locale.prototype.baseName
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.baseName

use kahvi_core::prelude::*;

/// get Intl.Locale.prototype.baseName
pub(crate) fn prototype_get_base_name(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Intl.Locale.prototype.baseName` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return GetLocaleBaseName(_loc_.[[Locale]]).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Intl.Locale.prototype.baseName"))
}
