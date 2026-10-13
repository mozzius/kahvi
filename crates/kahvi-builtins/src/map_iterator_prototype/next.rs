//! %MapIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%mapiteratorprototype%.next

use kahvi_core::prelude::*;

/// MapIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(*this* value, ~empty~, *"%MapIteratorPrototype%"*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("MapIteratorPrototype.next"))
}
