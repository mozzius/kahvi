//! %SetIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%setiteratorprototype%.next

use crate::support::collections::{
    CollectionIterKind, CollectionIterator, collection_iterator_step,
};
use kahvi_core::prelude::*;

/// SetIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(*this* value, ~empty~, *"%SetIteratorPrototype%"*).
    // GeneratorValidate: the receiver must be a Set Iterator.
    let it = match this {
        Value::Object(o)
            if agent
                .native::<CollectionIterator>(o)
                .is_some_and(|s| s.is_set) =>
        {
            o
        }
        _ => return Err(agent.throw_type_error("not a Set Iterator")),
    };
    let kind = agent
        .native::<CollectionIterator>(it)
        .map(|s| s.kind)
        .unwrap_or(CollectionIterKind::Value);
    // CreateSetIterator closure: for each entry e that is not empty, yield per kind; then return undefined.
    // GeneratorResume step 2: a completed generator returns { value: undefined, done: true }.
    let Some((key, _value)) = collection_iterator_step(agent, it) else {
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    };
    let result = match kind {
        // i. If kind is key, let result be e.[[Key]].
        // ii. Else if kind is value, let result be e.[[Value]].
        // A Set stores each element as its key, with an undefined value.
        CollectionIterKind::Key | CollectionIterKind::Value => key,
        // iii. Else, let result be CreateArrayFromList(« e.[[Value]], e.[[Value]] »).
        CollectionIterKind::KeyValue => {
            Value::Object(ops::create_array_from_list(agent, &[key, key]))
        }
    };
    // iv. Perform ? GeneratorYield(CreateIteratorResultObject(result, false)).
    Ok(Value::Object(ops::create_iter_result_object(
        agent, result, false,
    )))
}
