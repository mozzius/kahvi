//! Intl.ListFormat.prototype.formatToParts ( _list_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.ListFormat.prototype.formatToParts

use kahvi_core::prelude::*;

use crate::support::intl_list_format::{
    ListFormatSlots, format_list_to_parts, string_list_from_iterable,
};

/// Intl.ListFormat.prototype.formatToParts
pub(crate) fn prototype_format_to_parts(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `formatToParts` method is called with an argument _list_, the following steps are taken:
    //
    let _ = new_target;
    // 1. Let _lf_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_lf_, [[InitializedListFormat]]).
    let lf = match this {
        Value::Object(o) if agent.native::<ListFormatSlots>(o).is_some() => o,
        _ => return Err(agent.throw_type_error("not an Intl.ListFormat")),
    };
    // 3. Let _stringList_ be ? StringListFromIterable(_list_).
    let string_list = string_list_from_iterable(agent, args.get_or_undefined(0))?;
    // 4. Return FormatListToParts(_lf_, _stringList_).
    format_list_to_parts(agent, lf, &string_list)
}
