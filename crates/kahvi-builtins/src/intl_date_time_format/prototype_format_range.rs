//! Intl.DateTimeFormat.prototype.formatRange ( _startDate_, _endDate_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.datetimeformat.prototype.formatRange

use kahvi_core::prelude::*;

/// Intl.DateTimeFormat.prototype.formatRange
pub(crate) fn prototype_format_range(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `formatRange` method is called with arguments _startDate_ and _endDate_, the following steps are taken:
    //
    // 1. Let _dtf_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
    // 3. If _startDate_ is *undefined* or _endDate_ is *undefined*, throw a *TypeError* exception.
    // 4. Let _x_ be ? ToNumber(_startDate_).
    // 5. Let _y_ be ? ToNumber(_endDate_).
    // 6. Return ? FormatDateTimeRange(_dtf_, _x_, _y_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DateTimeFormat.prototype.formatRange"))
}
