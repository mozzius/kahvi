//! Set.prototype.isSubsetOf ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.issubsetof

use crate::support::collections::{set_table, this_set};
use kahvi_core::prelude::*;

/// GetSetRecord(obj) (§24.2.1.2). Returns the fields of a Set Record
/// (§24.2.1.1) as ([[SetObject]], [[Size]], [[Has]], [[Keys]]). Local to this
/// file until other Set methods need it too.
fn get_set_record(agent: &mut Agent, obj: Value) -> JsResult<(Object, f64, Value, Value)> {
    // 1. If obj is not an Object, throw a TypeError exception.
    let Value::Object(obj) = obj else {
        return Err(agent.throw_type_error("Set operation argument is not an object"));
    };
    // 2. Let rawSize be ? Get(obj, "size").
    let size_key = agent.key("size");
    let raw_size = ops::get(agent, obj, size_key)?;
    // 3. Let numberSize be ? ToNumber(rawSize).
    let number_size = ops::to_number(agent, raw_size)?;
    // 4. NOTE: If rawSize is undefined, then numberSize will be NaN.
    // 5. If numberSize is NaN, throw a TypeError exception.
    if number_size.is_nan() {
        return Err(agent.throw_type_error("Set operation size is NaN"));
    }
    // 6. Let intSize be ! ToIntegerOrInfinity(numberSize).
    let int_size = ops::to_integer_or_infinity(agent, Value::Number(number_size))?;
    // 7. If intSize < 0, throw a RangeError exception.
    if int_size < 0.0 {
        return Err(agent.throw_range_error("Set operation size is negative"));
    }
    // 8. Let has be ? Get(obj, "has").
    let has_key = agent.key("has");
    let has = ops::get(agent, obj, has_key)?;
    // 9. If IsCallable(has) is false, throw a TypeError exception.
    if !ops::is_callable(agent, has) {
        return Err(agent.throw_type_error("Set operation 'has' is not callable"));
    }
    // 10. Let keys be ? Get(obj, "keys").
    let keys_key = agent.key("keys");
    let keys = ops::get(agent, obj, keys_key)?;
    // 11. If IsCallable(keys) is false, throw a TypeError exception.
    if !ops::is_callable(agent, keys) {
        return Err(agent.throw_type_error("Set operation 'keys' is not callable"));
    }
    // 12. Return a new Set Record { [[SetObject]]: obj, [[Size]]: intSize, [[Has]]: has, [[Keys]]: keys }.
    Ok((obj, int_size, has, keys))
}

/// Set.prototype.isSubsetOf
pub(crate) fn prototype_is_subset_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let set be the this value.
    // 2. Perform ? RequireInternalSlot(set, [[SetData]]).
    let set = this_set(agent, this)?;
    // 3. Let otherRecord be ? GetSetRecord(other).
    let (set_object, size, has, _keys) = get_set_record(agent, args.get_or_undefined(0))?;
    // 4. If SetDataSize(set.[[SetData]]) > otherRecord.[[Size]], return false.
    let set_data_size = set_table(agent, set).len();
    if (set_data_size as f64) > size {
        return Ok(Value::Boolean(false));
    }
    // 5. Let thisSize be the number of elements in set.[[SetData]].
    let mut this_size = set_table(agent, set).slots();
    // 6. Let index be 0.
    let mut index = 0;
    // The set, the other set object and its has method stay rooted while
    // otherRecord.[[Has]] runs, since it may collect garbage.
    let roots = [Value::Object(set), Value::Object(set_object), has];
    agent.with_roots(&roots, |agent| {
        // 7. Repeat, while index < thisSize,
        while index < this_size {
            // a. Let entry be set.[[SetData]][index].
            let entry = set_table(agent, set).entry(index);
            // b. Set index to index + 1.
            index += 1;
            // c. If entry is not ~empty~, then
            if let Some((entry, _)) = entry {
                // i. Let inOther be ToBoolean(? Call(otherRecord.[[Has]], otherRecord.[[SetObject]], « entry »)).
                let mark = agent.push_root(entry);
                let in_other = ops::call_rooted(agent, has, Value::Object(set_object), &[entry])?;
                agent.pop_roots(mark);
                let in_other = ops::to_boolean(agent, in_other);
                // ii. If inOther is false, return false.
                if !in_other {
                    return Ok(Value::Boolean(false));
                }
                // iii. NOTE: The number of elements in set.[[SetData]] may have increased during execution of otherRecord.[[Has]].
                // iv. Set thisSize to the number of elements in set.[[SetData]].
                this_size = set_table(agent, set).slots();
            }
        }
        // 8. Return true.
        Ok(Value::Boolean(true))
    })
}
