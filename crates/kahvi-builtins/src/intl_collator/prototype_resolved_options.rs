//! Intl.Collator.prototype.resolvedOptions ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.collator.prototype.resolvedoptions

use kahvi_core::prelude::*;

use crate::support::intl_collator::CollatorSlots;

/// Intl.Collator.prototype.resolvedOptions
pub(crate) fn prototype_resolved_options(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (args, new_target);
    // 1. Let _collator_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_collator_, [[InitializedCollator]]).
    let slots = match this {
        Value::Object(o) => agent.native::<CollatorSlots>(o),
        _ => None,
    };
    let Some(slots) = slots else {
        return Err(agent.throw_type_error("this is not an Intl.Collator"));
    };
    let locale = slots.locale.clone();
    let usage = slots.usage.clone();
    let sensitivity = slots.sensitivity.clone();
    let ignore_punctuation = slots.ignore_punctuation;
    let collation = slots.collation.clone();
    let numeric = slots.numeric;
    let case_first = slots.case_first.clone();
    // 3. Let _options_ be OrdinaryObjectCreate(%Object.prototype%).
    let object_prototype = agent.intrinsic("%Object.prototype%");
    let options = agent.ordinary_object_create(Some(object_prototype));
    // 4. For each row of [table-collator-resolvedoptions-properties], except the header row, in table order, do
    //    a. Let _propertyKey_ be the Property value of the current row.
    //    b. Let _value_ be the value of _collator_'s internal slot whose name is the Internal Slot value of the current row.
    //    c. If the current row has an Extension Key value, then
    //       i. Let _extensionKey_ be the Extension Key value of the current row.
    //       ii. If %Intl.Collator%.[[RelevantExtensionKeys]] does not contain _extensionKey_, then
    //          1. Set _value_ to *undefined*.
    //       (Both "kn" and "kf" are relevant extension keys.)
    //    d. If _value_ is not *undefined*, then
    //       i. Perform ! CreateDataPropertyOrThrow(_options_, _propertyKey_, _value_).
    let rows = [
        ("locale", Value::String(agent.string(&locale))),
        ("usage", Value::String(agent.string(&usage))),
        ("sensitivity", Value::String(agent.string(&sensitivity))),
        ("ignorePunctuation", Value::Boolean(ignore_punctuation)),
        ("collation", Value::String(agent.string(&collation))),
        ("numeric", Value::Boolean(numeric)),
        ("caseFirst", Value::String(agent.string(&case_first))),
    ];
    for (name, value) in rows {
        let key = agent.key(name);
        ops::create_data_property_or_throw(agent, options, key, value)?;
    }
    // 5. Return _options_.
    Ok(Value::Object(options))
}
