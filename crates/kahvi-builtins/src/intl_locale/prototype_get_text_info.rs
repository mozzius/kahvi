//! Intl.Locale.prototype.getTextInfo ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.getTextInfo

use kahvi_core::prelude::*;

/// Intl.Locale.prototype.getTextInfo
pub(crate) fn prototype_get_text_info(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `getTextInfo` method is called, the following steps are taken:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Let _info_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. Let _dir_ be TextDirectionOfLocale(_loc_).
    // 5. Perform ! CreateDataPropertyOrThrow(_info_, *"direction"*, _dir_).
    // 6. Return _info_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Locale.prototype.getTextInfo"))
}
