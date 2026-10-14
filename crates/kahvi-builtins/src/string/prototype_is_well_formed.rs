//! String.prototype.isWellFormed ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.iswellformed

use kahvi_core::prelude::*;

/// String.prototype.isWellFormed
pub(crate) fn prototype_is_well_formed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Return IsStringWellFormedUnicode(_string_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.isWellFormed"))
}
