//! Intl.Segmenter.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.segmenter.prototype.resolvedoptions

use kahvi_core::prelude::*;

use crate::support::intl_segmenter::SegmenterSlots;

/// Intl.Segmenter.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides access to the locale and options computed during initialization of the object.
    //
    // 1. Let _segmenter_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_segmenter_, [[InitializedSegmenter]]).
    // 3. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. For each row of [table-segmenter-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. Let _value_ be the value of _segmenter_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. Assert: _value_ is not *undefined*.
    //    d. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    // 5. Return _options_.
    //
    // Resolved Options of Segmenter Instances
    // Internal Slot |
    // Property |
    // [[Locale]] |
    // *"locale"* |
    // [[SegmenterGranularity]] |
    // *"granularity"* |
    let _ = (args, new_target);
    let Some((locale, granularity)) = (match this {
        Value::Object(o) => agent
            .native::<SegmenterSlots>(o)
            .map(|s| (s.locale.clone(), s.granularity.clone())),
        _ => None,
    }) else {
        return Err(agent.throw_type_error(
            "Intl.Segmenter.prototype.resolvedOptions called on an incompatible receiver",
        ));
    };
    let proto = agent.intrinsic("%Object.prototype%");
    let options = agent.ordinary_object_create(Some(proto));
    let locale = agent.string(&locale);
    let key = agent.key("locale");
    ops::create_data_property_or_throw(agent, options, key, Value::String(locale))?;
    let granularity = agent.string(&granularity);
    let key = agent.key("granularity");
    ops::create_data_property_or_throw(agent, options, key, Value::String(granularity))?;
    Ok(Value::Object(options))
}
