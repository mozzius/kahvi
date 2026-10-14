//! Iterator.prototype.filter ( _predicate_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.filter

use kahvi_core::prelude::*;

use kahvi_core::ops::IteratorRecord;

use crate::support::iterator_helper::{
    HelperClosure, create_iterator_helper, if_abrupt_close_iterator,
};

/// Iterator.prototype.filter
pub(crate) fn prototype_filter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let predicate = args.get_or_undefined(0);
    // 1. Let obj be the this value.
    // 2. If obj is not an Object, throw a TypeError exception.
    let Value::Object(obj) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.filter called on a non-object"));
    };
    // 3. Let iterated be the Iterator Record { [[Iterator]]: obj, [[NextMethod]]: undefined, [[Done]]: false }.
    let iterated = IteratorRecord {
        iterator: obj,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. If IsCallable(predicate) is false, then
    if !ops::is_callable(agent, predicate) {
        // a. Let error be ThrowCompletion(a newly created TypeError object).
        let error = agent.throw_type_error("Iterator.prototype.filter: predicate is not callable");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 5. Set iterated to ? GetIteratorDirect(obj).
    let iterated = ops::get_iterator_direct(agent, obj)?;
    // 6. Let closure be a new Abstract Closure with no parameters that captures iterated and predicate
    //    and performs the following steps when called: (see FilterClosure::resume)
    let closure = FilterClosure {
        predicate,
        // a. Let counter be +0𝔽.
        counter: 0.0,
    };
    // 7. Let result be CreateIteratorFromClosure(closure, "Iterator Helper", %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 8. Set result.[[UnderlyingIterators]] to « iterated ».
    // 9. Return result.
    Ok(Value::Object(create_iterator_helper(
        agent,
        Box::new(closure),
        vec![iterated],
    )))
}

/// Step 6's closure: captures iterated and predicate.
#[derive(Debug)]
struct FilterClosure {
    predicate: Value,
    /// a. Let counter be +0𝔽.
    counter: f64,
}

impl HelperClosure for FilterClosure {
    // b. Repeat,
    fn resume(
        &mut self,
        agent: &mut Agent,
        underlying: &mut [IteratorRecord],
    ) -> JsResult<Option<Value>> {
        let iterated = &mut underlying[0];
        loop {
            // i. Let value be ? IteratorStepValue(iterated).
            // ii. If value is done, return ReturnCompletion(undefined).
            let Some(value) = ops::iterator_step_value(agent, iterated)? else {
                return Ok(None);
            };
            // iii. Let selected be Completion(Call(predicate, undefined, « value, counter »)).
            let selected = ops::call(
                agent,
                self.predicate,
                Value::Undefined,
                &[value, Value::Number(self.counter)],
            );
            // iv. IfAbruptCloseIterator(selected, iterated).
            let selected = if_abrupt_close_iterator(agent, selected, iterated)?;
            // vi. NOTE: The following step will not change counter once it reaches 2^53𝔽.
            // vii. Set counter to counter + 1𝔽.
            // (Done before the Yield below: counter is not observable while suspended.)
            if self.counter < 9007199254740992.0 {
                self.counter += 1.0;
            }
            // v. If ToBoolean(selected) is true, then
            if ops::to_boolean(agent, selected) {
                // 1. Let completion be Completion(Yield(value)).
                // 2. IfAbruptCloseIterator(completion, iterated): HelperClosure::on_return.
                return Ok(Some(value));
            }
        }
    }
}

impl kahvi_core::gc::Trace for FilterClosure {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        t.value(self.predicate);
    }
}
