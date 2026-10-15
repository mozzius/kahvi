//! Intl.DateTimeFormat.prototype.formatToParts ( _date_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DateTimeFormat.prototype.formatToParts

use kahvi_core::prelude::*;

/// Intl.DateTimeFormat.prototype.formatToParts
pub(crate) fn prototype_format_to_parts(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `formatToParts` method is called with an argument _date_, the following steps are taken:
    //
    // 1. Let _dtf_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
    // 3. If _date_ is *undefined*, then
    //    a. Let _x_ be ! Call(%Date.now%, *undefined*).
    // 4. Else,
    //    a. Let _x_ be ? ToNumber(_date_).
    // 5. Return ? FormatDateTimeToParts(_dtf_, _x_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DateTimeFormat.prototype.formatToParts"))
}
