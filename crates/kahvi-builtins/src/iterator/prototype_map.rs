//! Iterator.prototype.map ( _mapper_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.map

use kahvi_core::prelude::*;

use kahvi_core::ops::IteratorRecord;

use crate::support::iterator_helper::{
    HelperClosure, create_iterator_helper, if_abrupt_close_iterator,
};

/// Iterator.prototype.map
pub(crate) fn prototype_map(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let mapper = args.get_or_undefined(0);
    // 1. Let O be the this value.
    // 2. If O is not an Object, throw a TypeError exception.
    let Value::Object(o) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.map called on a non-object"));
    };
    // 3. Let iterated be the Iterator Record { [[Iterator]]: O, [[NextMethod]]: undefined, [[Done]]: false }.
    let iterated = IteratorRecord {
        iterator: o,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. If IsCallable(mapper) is false, then
    if !ops::is_callable(agent, mapper) {
        // a. Let error be ThrowCompletion(a newly created TypeError object).
        let error = agent.throw_type_error("Iterator.prototype.map: mapper is not callable");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 5. Set iterated to ? GetIteratorDirect(O).
    let iterated = ops::get_iterator_direct(agent, o)?;
    // 6-9. Let result be CreateIteratorFromClosure(closure, "Iterator Helper", %IteratorHelperPrototype%,
    //      « [[UnderlyingIterators]] ») with [[UnderlyingIterators]] « iterated ».
    let closure = MapClosure {
        mapper,
        counter: 0.0,
    };
    Ok(Value::Object(create_iterator_helper(
        agent,
        Box::new(closure),
        vec![iterated],
    )))
}

/// Step 6's closure: captures iterated and mapper.
#[derive(Debug)]
struct MapClosure {
    mapper: Value,
    /// a. Let counter be +0𝔽.
    counter: f64,
}

impl HelperClosure for MapClosure {
    // b. Repeat,
    fn resume(
        &mut self,
        agent: &mut Agent,
        underlying: &mut [IteratorRecord],
    ) -> JsResult<Option<Value>> {
        let iterated = &mut underlying[0];
        // i. Let value be ? IteratorStepValue(iterated).
        // ii. If value is done, return ReturnCompletion(undefined).
        let Some(value) = ops::iterator_step_value(agent, iterated)? else {
            return Ok(None);
        };
        // iii. Let mapped be Completion(Call(mapper, undefined, « value, counter »)).
        let mapped = ops::call(
            agent,
            self.mapper,
            Value::Undefined,
            &[value, Value::Number(self.counter)],
        );
        // iv. IfAbruptCloseIterator(mapped, iterated).
        let mapped = if_abrupt_close_iterator(agent, mapped, iterated)?;
        // vii-viii. Set counter to counter + 1𝔽 (it stops changing at 2^53).
        if self.counter < 9007199254740992.0 {
            self.counter += 1.0;
        }
        // v. Let completion be Completion(Yield(mapped)).
        // vi. IfAbruptCloseIterator(completion, iterated): HelperClosure::on_return.
        Ok(Some(mapped))
    }
}

impl kahvi_core::gc::Trace for MapClosure {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        t.value(self.mapper);
    }
}
