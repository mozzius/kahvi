//! get RegExp.prototype.dotAll
//!
//! Spec: https://tc39.es/ecma262/#sec-get-regexp.prototype.dotAll

use kahvi_core::prelude::*;

/// get RegExp.prototype.dotAll
pub(crate) fn prototype_get_dot_all(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `RegExp.prototype.dotAll` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. Let _codeUnit_ be the code unit 0x0073 (LATIN SMALL LETTER S).
    // 3. Return ? RegExpHasFlag(_regexp_, _codeUnit_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get RegExp.prototype.dotAll"))
}
