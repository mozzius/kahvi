//! RegExp.prototype.exec ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype.exec

use kahvi_core::prelude::*;

/// RegExp.prototype.exec
pub(crate) fn prototype_exec(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method searches _string_ for an occurrence of the regular expression pattern and returns an Array containing the results of the match, or *null* if _string_ did not match.
    // It performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_regexp_, [[RegExpMatcher]]).
    // 3. Set _string_ to ? ToString(_string_).
    // 4. Return ? RegExpBuiltinExec(_regexp_, _string_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype.exec"))
}
