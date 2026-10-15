//! Intl.PluralRules.prototype.selectRange ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.pluralrules.prototype.selectrange

use kahvi_core::prelude::*;

/// Intl.PluralRules.prototype.selectRange
pub(crate) fn prototype_select_range(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `selectRange` method is called with arguments _start_ and _end_, the following steps are taken:
    //
    // 1. Let _pr_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_pr_, [[InitializedPluralRules]]).
    // 3. If _start_ is *undefined* or _end_ is *undefined*, throw a *TypeError* exception.
    // 4. Let _x_ be ? ToIntlMathematicalValue(_start_).
    // 5. Let _y_ be ? ToIntlMathematicalValue(_end_).
    // 6. Return ? ResolvePluralRange(_pr_, _x_, _y_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.PluralRules.prototype.selectRange"))
}
