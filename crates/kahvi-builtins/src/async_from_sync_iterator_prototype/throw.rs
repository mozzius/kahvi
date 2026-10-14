//! %AsyncFromSyncIteratorPrototype%.throw ( [ _value_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asyncfromsynciteratorprototype%.throw

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// AsyncFromSyncIteratorPrototype.throw
pub(crate) fn throw(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1-14. See vm::async_from_sync_throw (AsyncFromSyncIteratorContinuation).
    vm::async_from_sync_throw(agent, this, args)
}
