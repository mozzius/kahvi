//! %ArrayIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%arrayiteratorprototype%.next

use crate::support::array_iterator::{ArrayIterator, IterationKind};
use kahvi_core::prelude::*;

/// ArrayIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // GeneratorResume(this value, empty, "%ArrayIteratorPrototype%"): the
    // receiver must be an Array Iterator.
    let Value::Object(it) = this else {
        return Err(agent.throw_type_error("not an Array Iterator"));
    };
    let Some(state) = agent.native::<ArrayIterator>(it) else {
        return Err(agent.throw_type_error("not an Array Iterator"));
    };
    let (iterated, index, kind) = (state.iterated, state.next_index, state.kind);
    // Once exhausted, the iterator stays done.
    let Some(array) = iterated else {
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    };
    // b. Let len be ? LengthOfArrayLike(array) (TypedArrays use TypedArrayLength;
    //    they are not supported yet).
    let len = ops::length_of_array_like(agent, array)?;
    // c. If index ≥ len, return NormalCompletion(undefined).
    if index >= len {
        if let Some(state) = agent.native_mut::<ArrayIterator>(it) {
            state.iterated = None;
        }
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    }
    // d. Let indexNumber be 𝔽(index).
    if let Some(state) = agent.native_mut::<ArrayIterator>(it) {
        state.next_index = index + 1.0;
    }
    let result = match kind {
        // e. If kind is key, let result be indexNumber.
        IterationKind::Key => Value::Number(index),
        _ => {
            // f.i Let elementKey be ! ToString(indexNumber).
            let key = agent.key_from_index(index);
            // f.ii Let elementValue be ? Get(array, elementKey).
            let element = ops::get(agent, array, key)?;
            match kind {
                // f.iii If kind is value, let result be elementValue.
                IterationKind::Value => element,
                // f.iv.2 Let result be CreateArrayFromList(« indexNumber, elementValue »).
                _ => Value::Object(ops::create_array_from_list(
                    agent,
                    &[Value::Number(index), element],
                )),
            }
        }
    };
    // g. Perform ? GeneratorYield(CreateIteratorResultObject(result, false)).
    Ok(Value::Object(ops::create_iter_result_object(
        agent, result, false,
    )))
}
