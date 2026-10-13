//! Object.preventExtensions ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.preventextensions

use kahvi_core::prelude::*;

/// Object.preventExtensions
pub(crate) fn prevent_extensions(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _obj_ is not an Object, return _obj_.
    // 2. Let _status_ be ? _obj_.[[PreventExtensions]]().
    // 3. If _status_ is *false*, throw a *TypeError* exception.
    // 4. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.preventExtensions"))
}
