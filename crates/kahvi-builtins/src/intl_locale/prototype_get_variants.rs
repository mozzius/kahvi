//! get Intl.Locale.prototype.variants
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.variants

use kahvi_core::prelude::*;

/// get Intl.Locale.prototype.variants
pub(crate) fn prototype_get_variants(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Intl.Locale.prototype.variants` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Return GetLocaleVariants(_loc_.[[Locale]]).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Intl.Locale.prototype.variants"))
}
