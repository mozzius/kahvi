//! %WrapForValidIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%wrapforvaliditeratorprototype%.next

use kahvi_core::prelude::*;

use crate::support::iterator_helper::wrapped_iterated;

/// WrapForValidIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be this value.
    // 2. Perform ? RequireInternalSlot(O, [[Iterated]]).
    // 3. Let iteratorRecord be O.[[Iterated]].
    let record = wrapped_iterated(agent, this)?;
    // 4. Return ? Call(iteratorRecord.[[NextMethod]], iteratorRecord.[[Iterator]]).
    ops::call(
        agent,
        record.next_method,
        Value::Object(record.iterator),
        &[],
    )
}
