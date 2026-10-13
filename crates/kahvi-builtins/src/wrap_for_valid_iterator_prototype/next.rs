//! %WrapForValidIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%wrapforvaliditeratorprototype%.next

use kahvi_core::prelude::*;

/// WrapForValidIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[Iterated]]).
    // 3. Let _iteratorRecord_ be _obj_.[[Iterated]].
    // 4. Return ? Call(_iteratorRecord_.[[NextMethod]], _iteratorRecord_.[[Iterator]]).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WrapForValidIteratorPrototype.next"))
}
