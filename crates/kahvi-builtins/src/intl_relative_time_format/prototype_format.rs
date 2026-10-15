//! Intl.RelativeTimeFormat.prototype.format ( _value_, _unit_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.RelativeTimeFormat.prototype.format

use kahvi_core::prelude::*;

/// Intl.RelativeTimeFormat.prototype.format
pub(crate) fn prototype_format(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `format` method is called with arguments _value_ and _unit_, the following steps are taken:
    //
    // 1. Let _relativeTimeFormat_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_relativeTimeFormat_, [[InitializedRelativeTimeFormat]]).
    // 3. Set _value_ to ? ToNumber(_value_).
    // 4. Set _unit_ to ? ToString(_unit_).
    // 5. Return ? FormatRelativeTime(_relativeTimeFormat_, _value_, _unit_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.RelativeTimeFormat.prototype.format"))
}
