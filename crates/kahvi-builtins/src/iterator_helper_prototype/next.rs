//! %IteratorHelperPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%iteratorhelperprototype%.next

use kahvi_core::prelude::*;

/// IteratorHelperPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(*this* value, *undefined*, *"Iterator Helper"*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("IteratorHelperPrototype.next"))
}
