//! RegExp.prototype.test ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype.test

use kahvi_core::prelude::*;

/// RegExp.prototype.test
pub(crate) fn prototype_test(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    // 3. Set _string_ to ? ToString(_string_).
    // 4. Let _match_ be ? RegExpExec(_regexp_, _string_).
    // 5. If _match_ is *null*, return *false*.
    // 6. Return *true*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype.test"))
}
