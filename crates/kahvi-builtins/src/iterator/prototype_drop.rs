//! Iterator.prototype.drop ( _limit_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.drop

use kahvi_core::ops::IteratorRecord;
use kahvi_core::prelude::*;

use crate::support::iterator_helper::{
    HelperClosure, create_iterator_helper, if_abrupt_close_iterator,
};

/// Iterator.prototype.drop
pub(crate) fn prototype_drop(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let limit = args.get_or_undefined(0);
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    let Value::Object(obj) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.drop called on a non-object"));
    };
    // 3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
    let iterated = IteratorRecord {
        iterator: obj,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. Let _numberLimit_ be Completion(ToNumber(_limit_)).
    let number_limit = ops::to_number(agent, limit);
    // 5. IfAbruptCloseIterator(_numberLimit_, _iterated_).
    let number_limit = if_abrupt_close_iterator(agent, number_limit, &iterated)?;
    // 6. If _numberLimit_ is *NaN*, then
    if number_limit.is_nan() {
        // a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
        let error = agent.throw_range_error("Iterator.prototype.drop: limit is NaN");
        // b. Return ? IteratorClose(_iterated_, _error_).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 7. If _numberLimit_ is finite and _numberLimit_ > 𝔽(2^53 - 1), then
    if number_limit.is_finite() && number_limit > 9007199254740991.0 {
        // a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
        let error = agent.throw_range_error("Iterator.prototype.drop: limit is too large");
        // b. Return ? IteratorClose(_iterated_, _error_).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 8. Let _intLimit_ be ! ToIntegerOrInfinity(_numberLimit_).
    let int_limit = ops::to_integer_or_infinity(agent, Value::Number(number_limit))?;
    // 9. If _intLimit_ < 0, then
    if int_limit < 0.0 {
        // a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
        let error = agent.throw_range_error("Iterator.prototype.drop: limit is negative");
        // b. Return ? IteratorClose(_iterated_, _error_).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 10. Set _iterated_ to ? GetIteratorDirect(_obj_).
    let iterated = ops::get_iterator_direct(agent, obj)?;
    // 11. Let _closure_ be a new Abstract Closure with no parameters that captures _iterated_ and _intLimit_ and performs the following steps when called:
    let closure = DropClosure {
        remaining: int_limit,
    };
    // 12. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 13. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
    // 14. Return _result_.
    Ok(Value::Object(create_iterator_helper(
        agent,
        Box::new(closure),
        vec![iterated],
    )))
}

/// Step 11's closure: captures _intLimit_ (as the remaining count to skip).
#[derive(Debug)]
struct DropClosure {
    /// a. Let _remaining_ be 𝔽(_intLimit_).
    remaining: f64,
}

impl HelperClosure for DropClosure {
    fn resume(
        &mut self,
        agent: &mut Agent,
        underlying: &mut [IteratorRecord],
    ) -> JsResult<Option<Value>> {
        let iterated = &mut underlying[0];
        // b. Repeat, while _remaining_ > *+0*_𝔽,
        while self.remaining > 0.0 {
            // i. Set _remaining_ to _remaining_ - *1*_𝔽.
            self.remaining -= 1.0;
            // ii. Let _next_ be ? IteratorStep(_iterated_).
            let next = ops::iterator_step(agent, iterated)?;
            // iii. If _next_ is ~done~, return ReturnCompletion(*undefined*).
            if next.is_none() {
                return Ok(None);
            }
        }
        // c. Repeat,
        //    i. Let _value_ be ? IteratorStepValue(_iterated_).
        let Some(value) = ops::iterator_step_value(agent, iterated)? else {
            // ii. If _value_ is ~done~, return ReturnCompletion(*undefined*).
            return Ok(None);
        };
        // iii. Let _completion_ be Completion(Yield(_value_)).
        // iv. IfAbruptCloseIterator(_completion_, _iterated_): HelperClosure::on_return.
        Ok(Some(value))
    }
}

impl kahvi_core::gc::Trace for DropClosure {
    fn trace(&self, _t: &mut kahvi_core::gc::Tracer) {}
}
