//! Reflect.ownKeys ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.ownkeys

use kahvi_core::prelude::*;

/// Reflect.ownKeys
pub(crate) fn own_keys(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Let _keys_ be ? _target_.[[OwnPropertyKeys]]().
    // 3. Return CreateArrayFromList(_keys_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.ownKeys"))
}
