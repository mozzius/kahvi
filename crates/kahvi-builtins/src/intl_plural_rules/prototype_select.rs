//! Intl.PluralRules.prototype.select ( _value_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.pluralrules.prototype.select

use kahvi_core::prelude::*;

/// Intl.PluralRules.prototype.select
pub(crate) fn prototype_select(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `select` method is called with an argument _value_, the following steps are taken:
    //
    // 1. Let _pr_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_pr_, [[InitializedPluralRules]]).
    // 3. Let _n_ be ? ToIntlMathematicalValue(_value_).
    // 4. Return ResolvePlural(_pr_, _n_).[[PluralCategory]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.PluralRules.prototype.select"))
}
