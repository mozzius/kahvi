//! Intl.DurationFormat.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DurationFormat.prototype.resolvedOptions

use kahvi_core::prelude::*;

/// Intl.DurationFormat.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides access to the locale and options computed during initialization of the object.
    //
    // 1. Let _df_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_df_, [[InitializedDurationFormat]]).
    // 3. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. For each row of [table-durationformat-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. Let _value_ be the value of _df_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. If _value_ is not *undefined*, then
    //       i. If there is a Conversion value in the current row, let _conversion_ be that value; else let _conversion_ be ~empty~.
    //       ii. If _conversion_ is ~number~, then
    //          1. Set _value_ to 𝔽(_value_).
    //       iii. Else if _conversion_ is not ~empty~, then
    //          1. Assert: _conversion_ is ~style+display~ and _value_ is a Duration Unit Options Record.
    //          2. NOTE: _value_.[[Style]] will be represented with a property named _propertyKey_ (a plural Temporal unit), then _value_.[[Display]] will be represented with a property whose name suffixes _propertyKey_ with *"Display"*.
    //          3. Let _style_ be _value_.[[Style]].
    //          4. If _style_ is *"fractional"*, then
    //             a. Assert: IsFractionalSecondUnitName(_propertyKey_) is *true*.
    //             b. Set _style_ to *"numeric"*.
    //          5. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _style_).
    //          6. Set _propertyKey_ to the string-concatenation of _propertyKey_ and *"Display"*.
    //          7. Set _value_ to _value_.[[Display]].
    //       iv. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    // 5. Return _options_.
    //
    // Resolved Options of DurationFormat Instances
    // Internal Slot |
    // Property |
    // Conversion |
    // [[Locale]] |
    // *"locale"* |
    // |
    // [[NumberingSystem]] |
    // *"numberingSystem"* |
    // |
    // [[Style]] |
    // *"style"* |
    // |
    // [[YearsOptions]] |
    // *"years"* |
    // ~style+display~ |
    // [[MonthsOptions]] |
    // *"months"* |
    // ~style+display~ |
    // [[WeeksOptions]] |
    // *"weeks"* |
    // ~style+display~ |
    // [[DaysOptions]] |
    // *"days"* |
    // ~style+display~ |
    // [[HoursOptions]] |
    // *"hours"* |
    // ~style+display~ |
    // [[MinutesOptions]] |
    // *"minutes"* |
    // ~style+display~ |
    // [[SecondsOptions]] |
    // *"seconds"* |
    // ~style+display~ |
    // [[MillisecondsOptions]] |
    // *"milliseconds"* |
    // ~style+display~ |
    // [[MicrosecondsOptions]] |
    // *"microseconds"* |
    // ~style+display~ |
    // [[NanosecondsOptions]] |
    // *"nanoseconds"* |
    // ~style+display~ |
    // [[FractionalDigits]] |
    // *"fractionalDigits"* |
    // ~number~ |
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DurationFormat.prototype.resolvedOptions"))
}
