//! Intl.DisplayNames.prototype.of ( _code_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DisplayNames.prototype.of

use kahvi_core::prelude::*;

/// Intl.DisplayNames.prototype.of
pub(crate) fn prototype_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.DisplayNames.prototype.of` is called with an argument _code_, the following steps are taken:
    //
    // 1. Let _displayNames_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_displayNames_, [[InitializedDisplayNames]]).
    // 3. Set _code_ to ? ToString(_code_).
    // 4. Set _code_ to ? CanonicalCodeForDisplayNames(_displayNames_.[[Type]], _code_).
    // 5. Let _fields_ be _displayNames_.[[Fields]].
    // 6. If _fields_ has a field [[<_code_>]], return _fields_.[[<_code_>]].
    // 7. If _displayNames_.[[Fallback]] is *"code"*, return _code_.
    // 8. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DisplayNames.prototype.of"))
}
