//! Intl.DateTimeFormat.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.datetimeformat.prototype.resolvedoptions

use kahvi_core::prelude::*;

/// Intl.DateTimeFormat.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides access to the locale and options computed during initialization of the object.
    //
    // 1. Let _dtf_ be the *this* value.
    // 2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Set _dtf_ to ? UnwrapDateTimeFormat(_dtf_).
    // 3. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
    // 4. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    // 5. For each row of [table-datetimeformat-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. If there is an Internal Slot value in the current row, then
    //       i. Let _value_ be the value of _dtf_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. Else,
    //       i. Let _format_ be _dtf_.[[DateTimeFormat]].
    //       ii. If _format_ has a field [[<_propertyKey_>]] and _dtf_.[[DateStyle]] is *undefined* and _dtf_.[[TimeStyle]] is *undefined*, then
    //          1. Let _value_ be _format_.[[<_propertyKey_>]].
    //       iii. Else,
    //          1. Let _value_ be *undefined*.
    //    d. If _value_ is not *undefined*, then
    //       i. If there is a Conversion value in the current row, then
    //          1. Let _conversion_ be the Conversion value of the current row.
    //          2. If _conversion_ is ~hour12~, then
    //             a. If _value_ is *"h11"* or *"h12"*, set _value_ to *true*. Otherwise, set _value_ to *false*.
    //          3. Else,
    //             a. Assert: _conversion_ is ~number~.
    //             b. Set _value_ to 𝔽(_value_).
    //       ii. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    // 6. Return _options_.
    //
    // Resolved Options of DateTimeFormat Instances
    // Internal Slot |
    // Property |
    // Conversion |
    // [[Locale]] |
    // *"locale"* |
    // |
    // [[Calendar]] |
    // *"calendar"* |
    // |
    // [[NumberingSystem]] |
    // *"numberingSystem"* |
    // |
    // [[TimeZone]] |
    // *"timeZone"* |
    // |
    // [[HourCycle]] |
    // *"hourCycle"* |
    // |
    // [[HourCycle]] |
    // *"hour12"* |
    // ~hour12~ |
    // |
    // *"weekday"* |
    // |
    // |
    // *"era"* |
    // |
    // |
    // *"year"* |
    // |
    // |
    // *"month"* |
    // |
    // |
    // *"day"* |
    // |
    // |
    // *"dayPeriod"* |
    // |
    // |
    // *"hour"* |
    // |
    // |
    // *"minute"* |
    // |
    // |
    // *"second"* |
    // |
    // |
    // *"fractionalSecondDigits"* |
    // ~number~ |
    // |
    // *"timeZoneName"* |
    // |
    // [[DateStyle]] |
    // *"dateStyle"* |
    // |
    // [[TimeStyle]] |
    // *"timeStyle"* |
    // |
    // For web compatibility reasons, if the property *"hourCycle"* is set, the *"hour12"* property should be set to *true* when *"hourCycle"* is *"h11"* or *"h12"*, or to *false* when *"hourCycle"* is *"h23"* or *"h24"*.
    // NOTE:
    // In this version of the API, the *"timeZone"* property will be the identifier of the host environment's time zone if no *"timeZone"* property was provided in the options object provided to the Intl.DateTimeFormat constructor. The first edition left the *"timeZone"* property *undefined* in this case.
    // NOTE:
    // For compatibility with versions prior to the fifth edition, the *"hour12"* property is set in addition to the *"hourCycle"* property.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DateTimeFormat.prototype.resolvedOptions"))
}
