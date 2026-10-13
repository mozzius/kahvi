//! %AsyncFromSyncIteratorPrototype%.next ( [ _value_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asyncfromsynciteratorprototype%.next

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// AsyncFromSyncIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1-14. See vm::async_from_sync_next (AsyncFromSyncIteratorContinuation).
    vm::async_from_sync_next(agent, this, args)
}
