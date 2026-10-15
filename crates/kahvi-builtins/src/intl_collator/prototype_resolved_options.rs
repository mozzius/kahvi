//! Intl.Collator.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.collator.prototype.resolvedoptions

use kahvi_core::prelude::*;

/// Intl.Collator.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides access to the locale and options computed during initialization of the object.
    //
    // 1. Let _collator_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_collator_, [[InitializedCollator]]).
    // 3. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. For each row of [table-collator-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. Let _value_ be the value of _collator_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. If the current row has an Extension Key value, then
    //       i. Let _extensionKey_ be the Extension Key value of the current row.
    //       ii. If %Intl.Collator%.[[RelevantExtensionKeys]] does not contain _extensionKey_, then
    //          1. Set _value_ to *undefined*.
    //    d. If _value_ is not *undefined*, then
    //       i. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    // 5. Return _options_.
    //
    // Resolved Options of Collator Instances
    // Internal Slot |
    // Property |
    // Extension Key |
    // [[Locale]] |
    // *"locale"* |
    // |
    // [[Usage]] |
    // *"usage"* |
    // |
    // [[Sensitivity]] |
    // *"sensitivity"* |
    // |
    // [[IgnorePunctuation]] |
    // *"ignorePunctuation"* |
    // |
    // [[Collation]] |
    // *"collation"* |
    // |
    // [[Numeric]] |
    // *"numeric"* |
    // *"kn"* |
    // [[CaseFirst]] |
    // *"caseFirst"* |
    // *"kf"* |
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Collator.prototype.resolvedOptions"))
}
