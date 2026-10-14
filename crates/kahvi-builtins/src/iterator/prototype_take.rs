//! Iterator.prototype.take ( _limit_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.take

use kahvi_core::prelude::*;

use kahvi_core::ops::IteratorRecord;

use crate::support::iterator_helper::{
    HelperClosure, create_iterator_helper, if_abrupt_close_iterator,
};

/// Iterator.prototype.take
pub(crate) fn prototype_take(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let limit = args.get_or_undefined(0);
    // 1. Let obj be the this value.
    // 2. If obj is not an Object, throw a TypeError exception.
    let Value::Object(obj) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.take called on a non-object"));
    };
    // 3. Let iterated be the Iterator Record { [[Iterator]]: obj, [[NextMethod]]: undefined, [[Done]]: false }.
    let iterated = IteratorRecord {
        iterator: obj,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. Let numberLimit be Completion(ToNumber(limit)).
    let number_limit = ops::to_number(agent, limit);
    // 5. IfAbruptCloseIterator(numberLimit, iterated).
    let number_limit = if_abrupt_close_iterator(agent, number_limit, &iterated)?;
    // 6. If numberLimit is NaN, then
    if number_limit.is_nan() {
        // a. Let error be ThrowCompletion(a newly created RangeError object).
        let error = agent.throw_range_error("Iterator.prototype.take: limit is NaN");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 7. If numberLimit is finite and numberLimit > 𝔽(2^53 - 1), then
    if number_limit.is_finite() && number_limit > 9007199254740991.0 {
        // a. Let error be ThrowCompletion(a newly created RangeError object).
        let error = agent.throw_range_error("Iterator.prototype.take: limit is too large");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 8. Let intLimit be ! ToIntegerOrInfinity(numberLimit).
    let int_limit = ops::to_integer_or_infinity(agent, Value::Number(number_limit))?;
    // 9. If intLimit < 0, then
    if int_limit < 0.0 {
        // a. Let error be ThrowCompletion(a newly created RangeError object).
        let error = agent.throw_range_error("Iterator.prototype.take: limit is negative");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 10. Set iterated to ? GetIteratorDirect(obj).
    let iterated = ops::get_iterator_direct(agent, obj)?;
    // 11. Let closure be a new Abstract Closure with no parameters that captures iterated and intLimit
    //     and performs the following steps when called:
    let closure = TakeClosure {
        remaining: int_limit,
    };
    // 12-13. Let result be CreateIteratorFromClosure(closure, "Iterator Helper", %IteratorHelperPrototype%,
    //        « [[UnderlyingIterators]] »), with [[UnderlyingIterators]] « iterated ».
    // 14. Return result.
    Ok(Value::Object(create_iterator_helper(
        agent,
        Box::new(closure),
        vec![iterated],
    )))
}

/// Step 11's closure: captures iterated and intLimit.
#[derive(Debug)]
struct TakeClosure {
    /// a. Let remaining be 𝔽(intLimit). (Updated by step b.ii.)
    remaining: f64,
}

impl HelperClosure for TakeClosure {
    // b. Repeat,
    fn resume(
        &mut self,
        agent: &mut Agent,
        underlying: &mut [IteratorRecord],
    ) -> JsResult<Option<Value>> {
        let iterated = &mut underlying[0];
        // i. If remaining is +0𝔽, then
        if self.remaining == 0.0 {
            // 1. Return ? IteratorClose(iterated, ReturnCompletion(undefined)).
            ops::iterator_close(agent, iterated, Ok(Value::Undefined))?;
            return Ok(None);
        }
        // ii. Set remaining to remaining - 1𝔽.
        self.remaining -= 1.0;
        // iii. Let value be ? IteratorStepValue(iterated).
        let Some(value) = ops::iterator_step_value(agent, iterated)? else {
            // iv. If value is done, return ReturnCompletion(undefined).
            return Ok(None);
        };
        // v. Let completion be Completion(Yield(value)).
        // vi. IfAbruptCloseIterator(completion, iterated): HelperClosure::on_return.
        Ok(Some(value))
    }
}

impl kahvi_core::gc::Trace for TakeClosure {
    fn trace(&self, _t: &mut kahvi_core::gc::Tracer) {
        // Captures only numbers: nothing to report to the collector.
    }
}
