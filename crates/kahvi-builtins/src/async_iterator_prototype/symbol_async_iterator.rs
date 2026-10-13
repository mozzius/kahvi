//! %AsyncIteratorPrototype% [ %Symbol.asyncIterator% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asynciteratorprototype%-%symbol.asynciterator%

use kahvi_core::prelude::*;

/// AsyncIteratorPrototype[Symbol.asyncIterator]
pub(crate) fn symbol_async_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Return the *this* value.
    //
    // The value of the *"name"* property of this function is *"[Symbol.asyncIterator]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncIteratorPrototype[Symbol.asyncIterator]"))
}
