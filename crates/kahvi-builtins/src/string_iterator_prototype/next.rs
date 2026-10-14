//! %StringIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%stringiteratorprototype%.next

use kahvi_core::prelude::*;

/// StringIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(*this* value, ~empty~, *"%StringIteratorPrototype%"*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("StringIteratorPrototype.next"))
}
