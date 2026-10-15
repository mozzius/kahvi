//! Intl.ListFormat.prototype.formatToParts ( _list_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.ListFormat.prototype.formatToParts

use kahvi_core::prelude::*;

/// Intl.ListFormat.prototype.formatToParts
pub(crate) fn prototype_format_to_parts(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `formatToParts` method is called with an argument _list_, the following steps are taken:
    //
    // 1. Let _lf_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_lf_, [[InitializedListFormat]]).
    // 3. Let _stringList_ be ? StringListFromIterable(_list_).
    // 4. Return FormatListToParts(_lf_, _stringList_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.ListFormat.prototype.formatToParts"))
}
