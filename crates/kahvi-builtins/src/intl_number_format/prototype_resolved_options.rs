//! Intl.NumberFormat.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.numberformat.prototype.resolvedoptions

use kahvi_core::prelude::*;

use crate::support::intl_number_format::{NumberFormatSlots, unwrap_number_format};

/// Intl.NumberFormat.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _nf_ be the *this* value.
    // 2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Set _nf_ to ? UnwrapNumberFormat(_nf_).
    let nf = unwrap_number_format(agent, this)?;
    // 3. Perform ? RequireInternalSlot(_nf_, [[InitializedNumberFormat]]).
    let Some(slots) = agent.native::<NumberFormatSlots>(nf).cloned() else {
        return Err(agent.throw_type_error("not an Intl.NumberFormat"));
    };
    // 4. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let options = agent.ordinary_object_create(Some(proto));
    // 5. For each row of [table-numberformat-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. Let _value_ be the value of _nf_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. If _value_ is not *undefined*, then
    //       i. If there is a Conversion value in the current row, then
    //          1. Assert: The Conversion value of the current row is ~number~.
    //          2. Set _value_ to 𝔽(_value_).
    //       ii. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    let strings: [(&str, Option<&str>); 8] = [
        ("locale", Some(&slots.locale)),
        ("numberingSystem", Some(&slots.numbering_system)),
        ("style", Some(&slots.style)),
        ("currency", slots.currency.as_deref()),
        ("currencyDisplay", slots.currency_display.as_deref()),
        ("currencySign", slots.currency_sign.as_deref()),
        ("unit", slots.unit.as_deref()),
        ("unitDisplay", slots.unit_display.as_deref()),
    ];
    let numbers: [(&str, Option<f64>); 5] = [
        ("minimumIntegerDigits", Some(slots.minimum_integer_digits)),
        ("minimumFractionDigits", slots.minimum_fraction_digits),
        ("maximumFractionDigits", slots.maximum_fraction_digits),
        ("minimumSignificantDigits", slots.minimum_significant_digits),
        ("maximumSignificantDigits", slots.maximum_significant_digits),
    ];
    let tail: [(&str, Option<&str>); 2] = [
        ("notation", Some(&slots.notation)),
        ("compactDisplay", slots.compact_display.as_deref()),
    ];
    for (name, value) in strings.iter() {
        if let Some(v) = value {
            let v = Value::String(agent.string(v));
            let key = agent.key(name);
            ops::create_data_property_or_throw(agent, options, key, v)?;
        }
    }
    for (name, value) in numbers {
        if let Some(v) = value {
            let key = agent.key(name);
            ops::create_data_property_or_throw(agent, options, key, Value::Number(v))?;
        }
    }
    // [[UseGrouping]] is the String, or *false*.
    let use_grouping = match &slots.use_grouping {
        Some(s) => Value::String(agent.string(s)),
        None => Value::Boolean(false),
    };
    let key = agent.key("useGrouping");
    ops::create_data_property_or_throw(agent, options, key, use_grouping)?;
    for (name, value) in tail.iter() {
        if let Some(v) = value {
            let v = Value::String(agent.string(v));
            let key = agent.key(name);
            ops::create_data_property_or_throw(agent, options, key, v)?;
        }
    }
    let key = agent.key("signDisplay");
    let v = Value::String(agent.string(&slots.sign_display));
    ops::create_data_property_or_throw(agent, options, key, v)?;
    let key = agent.key("roundingIncrement");
    ops::create_data_property_or_throw(
        agent,
        options,
        key,
        Value::Number(slots.rounding_increment),
    )?;
    for (name, v) in [
        ("roundingMode", &slots.rounding_mode),
        ("roundingPriority", &slots.computed_rounding_priority),
        ("trailingZeroDisplay", &slots.trailing_zero_display),
    ] {
        let key = agent.key(name);
        let v = Value::String(agent.string(v));
        ops::create_data_property_or_throw(agent, options, key, v)?;
    }
    // 6. Return _options_.
    Ok(Value::Object(options))
}
