//! String.prototype.trim ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.trim

use kahvi_core::prelude::*;

/// String.prototype.trim
pub(crate) fn prototype_trim(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Return ? TrimString(_thisValue_, ~start+end~).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    // [subclause sec-trimstring: TrimString (
    // _arg_: an ECMAScript language value,
    // _where_: ~start~, ~end~, or ~start+end~,
    // ): either a normal completion containing a String or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.trim"))
}
