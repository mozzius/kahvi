//! Object.seal ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.seal

use kahvi_core::prelude::*;

/// Object.seal
pub(crate) fn seal(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _obj_ is not an Object, return _obj_.
    // 2. Let _status_ be ? SetIntegrityLevel(_obj_, ~sealed~).
    // 3. If _status_ is *false*, throw a *TypeError* exception.
    // 4. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.seal"))
}
