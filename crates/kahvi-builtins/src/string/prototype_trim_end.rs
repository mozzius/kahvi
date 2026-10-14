//! String.prototype.trimEnd ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.trimend

use kahvi_core::prelude::*;

/// String.prototype.trimEnd
pub(crate) fn prototype_trim_end(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
    // It performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    // 2. Return ? TrimString(_string_, ~end~).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.trimEnd"))
}
