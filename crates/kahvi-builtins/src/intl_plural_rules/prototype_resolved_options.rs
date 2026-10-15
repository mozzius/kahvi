//! Intl.PluralRules.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.pluralrules.prototype.resolvedoptions

use kahvi_core::prelude::*;

/// Intl.PluralRules.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides access to the locale and options computed during initialization of the object.
    //
    // 1. Let _pr_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_pr_, [[InitializedPluralRules]]).
    // 3. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. Let _pluralCategories_ be a List of Strings containing all possible results of PluralRuleSelect for the selected locale _pr_.[[Locale]], sorted according to the following order: *"zero"*, *"one"*, *"two"*, *"few"*, *"many"*, *"other"*.
    // 5. For each row of [table-pluralrules-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. If _propertyKey_ is *"pluralCategories"*, then
    //       i. Let _value_ be CreateArrayFromList(_pluralCategories_).
    //    c. Else,
    //       i. Let _value_ be the value of _pr_'s internal slot whose name is the Internal Slot value of the current row.
    //    d. If _value_ is not *undefined*, then
    //       i. If there is a Conversion value in the current row, then
    //          1. Assert: The Conversion value of the current row is ~number~.
    //          2. Set _value_ to 𝔽(_value_).
    //       ii. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    // 6. Return _options_.
    //
    // Resolved Options of PluralRules Instances
    // Internal Slot |
    // Property |
    // Conversion |
    // [[Locale]] |
    // *"locale"* |
    // |
    // [[Type]] |
    // *"type"* |
    // |
    // [[Notation]] |
    // *"notation"* |
    // |
    // [[CompactDisplay]] |
    // *"compactDisplay"* |
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
    // |
    // *"pluralCategories"* |
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
    Err(agent.not_implemented("Intl.PluralRules.prototype.resolvedOptions"))
}
