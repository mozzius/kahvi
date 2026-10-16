//! Intl.DateTimeFormat.prototype.formatToParts ( _date_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DateTimeFormat.prototype.formatToParts

use kahvi_core::prelude::*;

use crate::support::date;
use crate::support::intl_date_time_format::DateTimeFormatSlots;

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
    let _ = new_target;
    // 1. Let _dtf_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
    match this {
        Value::Object(o) if agent.native::<DateTimeFormatSlots>(o).is_some() => {}
        _ => return Err(agent.throw_type_error("not an Intl.DateTimeFormat")),
    }
    // 3. If _date_ is *undefined*, then
    //    a. Let _x_ be ! Call(%Date.now%, *undefined*).
    // 4. Else,
    //    a. Let _x_ be ? ToNumber(_date_).
    let _x = match args.get_or_undefined(0) {
        Value::Undefined => date::now(agent),
        date => ops::to_number(agent, date)?,
    };
    // 5. Return ? FormatDateTimeToParts(_dtf_, _x_).
    Err(agent.not_implemented(
        "Intl.DateTimeFormat formatToParts: ICU4X's formatted parts need the writeable crate, which kahvi-builtins does not depend on",
    ))
}
