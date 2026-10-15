//! Intl.DisplayNames.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DisplayNames.prototype.resolvedOptions

use kahvi_core::prelude::*;

/// Intl.DisplayNames.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides access to the locale and options computed during initialization of the object.
    //
    // 1. Let _displayNames_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_displayNames_, [[InitializedDisplayNames]]).
    // 3. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. For each row of [table-displaynames-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. Let _value_ be the value of _displayNames_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. If _value_ is not *undefined*, then
    //       i. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    // 5. Return _options_.
    //
    // Resolved Options of DisplayNames Instances
    // Internal Slot |
    // Property |
    // [[Locale]] |
    // *"locale"* |
    // [[Style]] |
    // *"style"* |
    // [[Type]] |
    // *"type"* |
    // [[Fallback]] |
    // *"fallback"* |
    // [[LanguageDisplay]] |
    // *"languageDisplay"* |
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DisplayNames.prototype.resolvedOptions"))
}
