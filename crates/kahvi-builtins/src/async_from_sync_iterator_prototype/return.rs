//! %AsyncFromSyncIteratorPrototype%.return ( [ _value_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asyncfromsynciteratorprototype%.return

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// AsyncFromSyncIteratorPrototype.return
pub(crate) fn r#return(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1-14. See vm::async_from_sync_return (AsyncFromSyncIteratorContinuation).
    vm::async_from_sync_return(agent, this, args)
}
