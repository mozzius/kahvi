//! Iterator.prototype.reduce ( _reducer_ [ , _initialValue_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.reduce

use kahvi_core::prelude::*;

use kahvi_core::ops::IteratorRecord;

use crate::support::iterator_helper::if_abrupt_close_iterator;

/// Iterator.prototype.reduce
pub(crate) fn prototype_reduce(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let reducer = args.get_or_undefined(0);
    let initial_value = args.get_or_undefined(1);
    // 1. Let obj be the this value.
    // 2. If obj is not an Object, throw a TypeError exception.
    let Value::Object(obj) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.reduce called on a non-object"));
    };
    // 3. Let iterated be the Iterator Record { [[Iterator]]: obj, [[NextMethod]]: undefined, [[Done]]: false }.
    let iterated = IteratorRecord {
        iterator: obj,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. If IsCallable(reducer) is false, then
    if !ops::is_callable(agent, reducer) {
        // a. Let error be ThrowCompletion(a newly created TypeError object).
        let error = agent.throw_type_error("Iterator.prototype.reduce: reducer is not callable");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 5. Set iterated to ? GetIteratorDirect(obj).
    let mut iterated = ops::get_iterator_direct(agent, obj)?;
    // The reducer may collect garbage: obj, reducer and the next method stay
    // rooted (the next method may be reachable from nothing else).
    let roots = [Value::Object(obj), reducer, iterated.next_method];
    agent.with_roots(&roots, |agent| {
        // 6. If initialValue is not present, then
        let (mut accumulator, mut counter) = if args.len() < 2 {
            // a. Let accumulator be ? IteratorStepValue(iterated).
            let Some(first) = ops::iterator_step_value_rooted(agent, &mut iterated)? else {
                // b. If accumulator is done, throw a TypeError exception.
                return Err(agent.throw_type_error(
                    "Iterator.prototype.reduce of an empty iterator with no initial value",
                ));
            };
            // c. Let counter be 1𝔽.
            (first, 1.0)
        // 7. Else,
        } else {
            // a. Let accumulator be initialValue.
            // b. Let counter be +0𝔽.
            (initial_value, 0.0)
        };
        // 8. Repeat,
        loop {
            // The accumulator stays rooted until the next call takes it.
            let mark = agent.gc_roots_len();
            agent.push_root(accumulator);
            // a. Let value be ? IteratorStepValue(iterated).
            let Some(value) = ops::iterator_step_value_rooted(agent, &mut iterated)? else {
                // b. If value is done, return accumulator.
                return Ok(accumulator);
            };
            agent.push_root(value);
            // c. Let result be Completion(Call(reducer, undefined, « accumulator, value, counter »)).
            let result = ops::call_rooted(
                agent,
                reducer,
                Value::Undefined,
                &[accumulator, value, Value::Number(counter)],
            );
            // d. IfAbruptCloseIterator(result, iterated).
            let result = if_abrupt_close_iterator(agent, result, &iterated)?;
            agent.pop_roots(mark);
            // e. Set accumulator to result.
            accumulator = result;
            // f. NOTE: The following step will not change counter once it reaches 2^53𝔽.
            // g. Set counter to counter + 1𝔽.
            if counter < 9007199254740992.0 {
                counter += 1.0;
            }
        }
    })
}
