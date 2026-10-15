//! Intl.NumberFormat.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.numberformat.prototype.resolvedoptions

use kahvi_core::prelude::*;

/// Intl.NumberFormat.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides access to the locale and options computed during initialization of the object.
    //
    // 1. Let _nf_ be the *this* value.
    // 2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Set _nf_ to ? UnwrapNumberFormat(_nf_).
    // 3. Perform ? RequireInternalSlot(_nf_, [[InitializedNumberFormat]]).
    // 4. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    // 5. For each row of [table-numberformat-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. Let _value_ be the value of _nf_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. If _value_ is not *undefined*, then
    //       i. If there is a Conversion value in the current row, then
    //          1. Assert: The Conversion value of the current row is ~number~.
    //          2. Set _value_ to 𝔽(_value_).
    //       ii. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    // 6. Return _options_.
    //
    // Resolved Options of NumberFormat Instances
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
    // [[Currency]] |
    // *"currency"* |
    // |
    // [[CurrencyDisplay]] |
    // *"currencyDisplay"* |
    // |
    // [[CurrencySign]] |
    // *"currencySign"* |
    // |
    // [[Unit]] |
    // *"unit"* |
    // |
    // [[UnitDisplay]] |
    // *"unitDisplay"* |
    // |
    // [[MinimumIntegerDigits]] |
    // *"minimumIntegerDigits"* |
    // ~number~ |
    // [[MinimumFractionDigits]] |
    // *"minimumFractionDigits"* |
    // ~number~ |
    // [[MaximumFractionDigits]] |
    // *"maximumFractionDigits"* |
    // ~number~ |
    // [[MinimumSignificantDigits]] |
    // *"minimumSignificantDigits"* |
    // ~number~ |
    // [[MaximumSignificantDigits]] |
    // *"maximumSignificantDigits"* |
    // ~number~ |
    // [[UseGrouping]] |
    // *"useGrouping"* |
    // |
    // [[Notation]] |
    // *"notation"* |
    // |
    // [[CompactDisplay]] |
    // *"compactDisplay"* |
    // |
    // [[SignDisplay]] |
    // *"signDisplay"* |
    // |
    // [[RoundingIncrement]] |
    // *"roundingIncrement"* |
    // ~number~ |
    // [[RoundingMode]] |
    // *"roundingMode"* |
    // |
    // [[ComputedRoundingPriority]] |
    // *"roundingPriority"* |
    // |
    // [[TrailingZeroDisplay]] |
    // *"trailingZeroDisplay"* |
    // |
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.NumberFormat.prototype.resolvedOptions"))
}
