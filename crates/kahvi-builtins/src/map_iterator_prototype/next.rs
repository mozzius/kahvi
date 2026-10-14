//! %MapIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%mapiteratorprototype%.next

use crate::support::collections::{
    CollectionIterKind, CollectionIterator, collection_iterator_step,
};
use kahvi_core::prelude::*;

/// MapIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(this value, empty, "%MapIteratorPrototype%"): the
    //    receiver must be a Map Iterator.
    let it = match this {
        Value::Object(o)
            if agent
                .native::<CollectionIterator>(o)
                .is_some_and(|s| !s.is_set) =>
        {
            o
        }
        _ => return Err(agent.throw_type_error("not a Map Iterator")),
    };
    let kind = agent
        .native::<CollectionIterator>(it)
        .map(|s| s.kind)
        .unwrap_or(CollectionIterKind::KeyValue);
    // CreateMapIterator closure: for each entry e that is not empty, yield per kind; then return undefined.
    let Some((key, value)) = collection_iterator_step(agent, it) else {
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    };
    let result = match kind {
        // i. If kind is key, let result be e.[[Key]].
        CollectionIterKind::Key => key,
        // ii. Else if kind is value, let result be e.[[Value]].
        CollectionIterKind::Value => value,
        // iii. Else, let result be CreateArrayFromList(« e.[[Key]], e.[[Value]] »).
        CollectionIterKind::KeyValue => {
            Value::Object(ops::create_array_from_list(agent, &[key, value]))
        }
    };
    // iv. Perform ? GeneratorYield(CreateIteratorResultObject(result, false)).
    Ok(Value::Object(ops::create_iter_result_object(
        agent, result, false,
    )))
}
