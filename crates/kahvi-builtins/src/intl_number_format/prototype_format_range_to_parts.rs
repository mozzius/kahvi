//! Intl.NumberFormat.prototype.formatRangeToParts ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.numberformat.prototype.formatrangetoparts

use kahvi_core::prelude::*;

/// Intl.NumberFormat.prototype.formatRangeToParts
pub(crate) fn prototype_format_range_to_parts(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `formatRangeToParts` method is called with arguments _start_ and _end_, the following steps are taken:
    //
    // 1. Let _nf_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_nf_, [[InitializedNumberFormat]]).
    // 3. If _start_ is *undefined* or _end_ is *undefined*, throw a *TypeError* exception.
    // 4. Let _x_ be ? ToIntlMathematicalValue(_start_).
    // 5. Let _y_ be ? ToIntlMathematicalValue(_end_).
    // 6. Return ? FormatNumericRangeToParts(_nf_, _x_, _y_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.NumberFormat.prototype.formatRangeToParts"))
}
