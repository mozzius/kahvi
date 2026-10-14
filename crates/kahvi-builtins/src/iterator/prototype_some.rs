//! Iterator.prototype.some ( _predicate_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.some

use kahvi_core::ops::IteratorRecord;
use kahvi_core::prelude::*;

use crate::support::iterator_helper::if_abrupt_close_iterator;

/// Iterator.prototype.some
pub(crate) fn prototype_some(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let predicate = args.get_or_undefined(0);
    // 1. Let obj be the this value.
    // 2. If obj is not an Object, throw a TypeError exception.
    let Value::Object(obj) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.some called on a non-object"));
    };
    // 3. Let iterated be the Iterator Record { [[Iterator]]: obj, [[NextMethod]]: undefined, [[Done]]: false }.
    let mut iterated = IteratorRecord {
        iterator: obj,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. If IsCallable(predicate) is false, then
    if !ops::is_callable(agent, predicate) {
        // a. Let error be ThrowCompletion(a newly created TypeError object).
        let error = agent.throw_type_error("Iterator.prototype.some: predicate is not callable");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 5. Set iterated to ? GetIteratorDirect(obj).
    iterated = ops::get_iterator_direct(agent, obj)?;
    // 6. Let counter be +0𝔽.
    let mut counter: f64 = 0.0;
    // 7. Repeat,
    loop {
        // a. Let value be ? IteratorStepValue(iterated).
        let Some(value) = ops::iterator_step_value(agent, &mut iterated)? else {
            // b. If value is done, return false.
            return Ok(Value::Boolean(false));
        };
        // c. Let result be Completion(Call(predicate, undefined, « value, counter »)).
        let result = ops::call(
            agent,
            predicate,
            Value::Undefined,
            &[value, Value::Number(counter)],
        );
        // d. IfAbruptCloseIterator(result, iterated).
        let result = if_abrupt_close_iterator(agent, result, &iterated)?;
        // e. If ToBoolean(result) is true, return ? IteratorClose(iterated, NormalCompletion(true)).
        if ops::to_boolean(agent, result) {
            return ops::iterator_close(agent, &iterated, Ok(Value::Boolean(true)));
        }
        // f. NOTE: The following step will not change counter once it reaches 2^53𝔽.
        // g. Set counter to counter + 1𝔽.
        if counter < 9007199254740992.0 {
            counter += 1.0;
        }
    }
}
