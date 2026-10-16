//! Intl.DateTimeFormat.prototype.formatRange ( _startDate_, _endDate_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.datetimeformat.prototype.formatRange

use kahvi_core::prelude::*;

use crate::support::intl_date_time_format::DateTimeFormatSlots;

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
    let _ = new_target;
    // 1. Let _dtf_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
    match this {
        Value::Object(o) if agent.native::<DateTimeFormatSlots>(o).is_some() => {}
        _ => return Err(agent.throw_type_error("not an Intl.DateTimeFormat")),
    }
    // 3. If _startDate_ is *undefined* or _endDate_ is *undefined*, throw a *TypeError* exception.
    let (start_date, end_date) = (args.get_or_undefined(0), args.get_or_undefined(1));
    if matches!(start_date, Value::Undefined) || matches!(end_date, Value::Undefined) {
        return Err(agent.throw_type_error("formatRange requires a start and an end date"));
    }
    // 4. Let _x_ be ? ToNumber(_startDate_).
    let _x = ops::to_number(agent, start_date)?;
    // 5. Let _y_ be ? ToNumber(_endDate_).
    let _y = ops::to_number(agent, end_date)?;
    // 6. Return ? FormatDateTimeRange(_dtf_, _x_, _y_).
    Err(agent.not_implemented(
        "Intl.DateTimeFormat formatRange: PartitionDateTimeRangePattern (ICU4X has no date interval patterns)",
    ))
}
