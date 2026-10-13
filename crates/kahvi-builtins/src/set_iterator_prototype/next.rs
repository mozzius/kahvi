//! %SetIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%setiteratorprototype%.next

use kahvi_core::prelude::*;

/// SetIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(*this* value, ~empty~, *"%SetIteratorPrototype%"*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("SetIteratorPrototype.next"))
}
