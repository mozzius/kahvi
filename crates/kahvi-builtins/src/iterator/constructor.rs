//! Iterator ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator

use kahvi_core::prelude::*;

/// Iterator
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is either *undefined* or the active function object, throw a *TypeError* exception.
    // 2. Return ? OrdinaryCreateFromConstructor(NewTarget, *"%Iterator.prototype%"*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator"))
}
