//! Intl.NumberFormat.prototype.formatToParts ( _value_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.numberformat.prototype.formattoparts

use kahvi_core::prelude::*;

/// Intl.NumberFormat.prototype.formatToParts
pub(crate) fn prototype_format_to_parts(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `formatToParts` method is called with an optional argument _value_, the following steps are taken:
    //
    // 1. Let _nf_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_nf_, [[InitializedNumberFormat]]).
    // 3. Let _x_ be ? ToIntlMathematicalValue(_value_).
    // 4. Return FormatNumericToParts(_nf_, _x_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.NumberFormat.prototype.formatToParts"))
}
