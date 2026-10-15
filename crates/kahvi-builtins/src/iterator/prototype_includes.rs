//! Iterator.prototype.includes ( _searchElement_ [ , _skippedElements_ ] )
//!
//! Spec: https://tc39.es/proposal-iterator-includes/#sec-iterator.prototype.includes

use kahvi_core::ops::IteratorRecord;
use kahvi_core::prelude::*;

/// Iterator.prototype.includes
pub(crate) fn prototype_includes(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let search_element = args.get_or_undefined(0);
    let skipped_elements = args.get_or_undefined(1);
    // 1. Let obj be the this value.
    // 2. If obj is not an Object, throw a TypeError exception.
    let Value::Object(obj) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.includes called on a non-object"));
    };
    // 3. Let iterated be the Iterator Record { [[Iterator]]: obj, [[NextMethod]]: undefined, [[Done]]: false }.
    let iterated = IteratorRecord {
        iterator: obj,
        next_method: Value::Undefined,
        done: false,
    };
    let to_skip = match skipped_elements {
        // 4. If skippedElements is undefined, then
        //    a. Let toSkip be +0𝔽.
        Value::Undefined => 0.0,
        // 5. Else,
        //    a. If skippedElements is not one of +∞𝔽, -∞𝔽, or an integral Number, then
        //       i. Let error be ThrowCompletion(a newly created TypeError object).
        //       ii. Return ? IteratorClose(iterated, error).
        //    b. Let toSkip be skippedElements.
        Value::Number(n) if n.is_infinite() || n.trunc() == n => n,
        _ => {
            let error = agent.throw_type_error(
                "Iterator.prototype.includes: skippedElements must be an integral Number",
            );
            return ops::iterator_close(agent, &iterated, Err(error));
        }
    };
    // 6. If toSkip < -0𝔽, then
    if to_skip < 0.0 {
        // a. Let error be ThrowCompletion(a newly created RangeError object).
        let error =
            agent.throw_range_error("Iterator.prototype.includes: skippedElements is negative");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 7. If toSkip is finite and toSkip > 𝔽(2^53 - 1), then
    if to_skip.is_finite() && to_skip > 9007199254740991.0 {
        // a. Let error be ThrowCompletion(a newly created RangeError object).
        let error =
            agent.throw_range_error("Iterator.prototype.includes: skippedElements is too large");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 8. Let skipped be +0𝔽.
    let mut skipped = 0.0;
    // 9. Set iterated to ? GetIteratorDirect(obj).
    let mut iterated = ops::get_iterator_direct(agent, obj)?;
    // 10. Repeat,
    loop {
        // a. Let value be ? IteratorStepValue(iterated).
        let Some(value) = ops::iterator_step_value(agent, &mut iterated)? else {
            // b. If value is done, return false.
            return Ok(Value::Boolean(false));
        };
        // c. If skipped < toSkip, then
        if skipped < to_skip {
            // i. Set skipped to skipped + 1𝔽.
            skipped += 1.0;
        } else if ops::same_value_zero(agent, value, search_element) {
            // d. Else if SameValueZero(value, searchElement) is true, then
            //    i. Return ? IteratorClose(iterated, NormalCompletion(true)).
            return ops::iterator_close(agent, &iterated, Ok(Value::Boolean(true)));
        }
    }
}
