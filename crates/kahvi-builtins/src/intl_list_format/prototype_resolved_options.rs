//! Intl.ListFormat.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.ListFormat.prototype.resolvedoptions

use kahvi_core::prelude::*;

use crate::support::intl_list_format::ListFormatSlots;

/// Intl.ListFormat.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides access to the locale and options computed during initialization of the object.
    //
    // 1. Let _lf_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_lf_, [[InitializedListFormat]]).
    // 3. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. For each row of [table-listformat-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. Let _value_ be the value of _lf_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. Assert: _value_ is not *undefined*.
    //    d. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    // 5. Return _options_.
    //
    // Resolved Options of ListFormat Instances
    // Internal Slot |
    // Property |
    // [[Locale]] |
    // *"locale"* |
    // [[Type]] |
    // *"type"* |
    // [[Style]] |
    // *"style"* |
    // 1. Let _lf_ be the *this* value.
    let _ = (args, new_target);
    // 2. Perform ? RequireInternalSlot(_lf_, [[InitializedListFormat]]).
    let slots = match this {
        Value::Object(o) => agent.native::<ListFormatSlots>(o),
        _ => None,
    };
    let Some(slots) = slots else {
        return Err(agent.throw_type_error("not an Intl.ListFormat"));
    };
    let values = [
        ("locale", slots.locale.clone()),
        ("type", slots.r#type.clone()),
        ("style", slots.style.clone()),
    ];
    // 3. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let options = agent.ordinary_object_create(Some(proto));
    // 4. For each row of [table-listformat-resolvedoptions-properties], except the header row, in table order, do
    for (property, value) in values {
        // a. Let _propertyKey_ be the Property value of the current row.
        let key = agent.key(property);
        // b. Let _value_ be the value of _lf_'s internal slot whose name is the Internal Slot value of the current row.
        // c. Assert: _value_ is not *undefined*.
        let value = agent.string(&value);
        // d. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
        ops::create_data_property_or_throw(agent, options, key, Value::String(value))?;
    }
    // 5. Return _options_.
    Ok(Value::Object(options))
}
