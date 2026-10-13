//! %AsyncIteratorPrototype% [ %Symbol.asyncIterator% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asynciteratorprototype%-%symbol.asynciterator%

use kahvi_core::prelude::*;

/// AsyncIteratorPrototype[Symbol.asyncIterator]
pub(crate) fn symbol_async_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return the this value.
    let _ = (agent, args);
    Ok(this)
}
