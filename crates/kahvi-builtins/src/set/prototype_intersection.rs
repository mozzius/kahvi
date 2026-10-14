//! Set.prototype.intersection ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.intersection

use crate::support::collections::{OrderedTable, SetData, collection_key, set_table, this_set};
use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// A Set Record (§24.2.1.2): the other object with its size, has and keys.
struct SetRecord {
    /// [[SetObject]]
    set_object: Object,
    /// [[Size]]
    size: f64,
    /// [[Has]]
    has: Value,
    /// [[Keys]]
    keys: Value,
}

/// GetSetRecord(obj) (§24.2.1.2). Not in the shared helpers yet; other Set
/// built-ins (union, difference, ...) will need it too.
fn get_set_record(agent: &mut Agent, obj: Value) -> JsResult<SetRecord> {
    // 1. If obj is not an Object, throw a TypeError exception.
    let Value::Object(set_object) = obj else {
        return Err(agent.throw_type_error("argument is not an object"));
    };
    // 2. Let rawSize be ? Get(obj, "size").
    let size_key = agent.key("size");
    let raw_size = ops::get(agent, set_object, size_key)?;
    // 3. Let numberSize be ? ToNumber(rawSize).
    let number_size = ops::to_number(agent, raw_size)?;
    // 4. NOTE: If rawSize is undefined, then numberSize will be NaN.
    // 5. If numberSize is NaN, throw a TypeError exception.
    if number_size.is_nan() {
        return Err(agent.throw_type_error("size is not a number"));
    }
    // 6. Let intSize be ! ToIntegerOrInfinity(numberSize).
    let int_size = ops::to_integer_or_infinity(agent, Value::Number(number_size))?;
    // 7. If intSize < 0, throw a RangeError exception.
    if int_size < 0.0 {
        return Err(agent.throw_range_error("size must not be negative"));
    }
    // 8. Let has be ? Get(obj, "has").
    let has_key = agent.key("has");
    let has = ops::get(agent, set_object, has_key)?;
    // 9. If IsCallable(has) is false, throw a TypeError exception.
    if !ops::is_callable(agent, has) {
        return Err(agent.throw_type_error("size.has is not a function"));
    }
    // 10. Let keys be ? Get(obj, "keys").
    let keys_key = agent.key("keys");
    let keys = ops::get(agent, set_object, keys_key)?;
    // 11. If IsCallable(keys) is false, throw a TypeError exception.
    if !ops::is_callable(agent, keys) {
        return Err(agent.throw_type_error("keys is not a function"));
    }
    // 12. Return a new Set Record { [[SetObject]]: obj, [[Size]]: intSize, [[Has]]: has, [[Keys]]: keys }.
    Ok(SetRecord {
        set_object,
        size: int_size,
        has,
        keys,
    })
}

/// Set.prototype.intersection
pub(crate) fn prototype_intersection(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let set be the this value.
    // 2. Perform ? RequireInternalSlot(set, [[SetData]]).
    let set = this_set(agent, this)?;
    // 3. Let otherRecord be ? GetSetRecord(other).
    let other_record = get_set_record(agent, args.get_or_undefined(0))?;
    // 4. Let resultSetData be a new empty List.
    let mut result_set_data = OrderedTable::default();
    // 5. If SetDataSize(set.[[SetData]]) ≤ otherRecord.[[Size]], then
    if (set_table(agent, set).len() as f64) <= other_record.size {
        // a. Let thisSize be the number of elements in set.[[SetData]].
        let mut this_size = set_table(agent, set).slots();
        // b. Let index be 0.
        let mut index = 0;
        // c. Repeat, while index < thisSize,
        while index < this_size {
            // i. Let entry be set.[[SetData]][index].
            let entry = set_table(agent, set).entry(index);
            // ii. Set index to index + 1.
            index += 1;
            // iii. If entry is not empty, then
            if let Some((entry, _)) = entry {
                // 1. Let inOther be ToBoolean(? Call(otherRecord.[[Has]], otherRecord.[[SetObject]], « entry »)).
                let in_other = ops::call(
                    agent,
                    other_record.has,
                    Value::Object(other_record.set_object),
                    &[entry],
                )?;
                let in_other = ops::to_boolean(agent, in_other);
                // 2. If inOther is true, then
                if in_other {
                    // a. NOTE: It is possible for earlier calls to otherRecord.[[Has]] to remove and re-add an element of set.[[SetData]], which can cause the same element to be visited twice during this iteration.
                    // b. If SetDataHas(resultSetData, entry) is false, then
                    let key = collection_key(agent, entry);
                    if !result_set_data.has(&key) {
                        // i. Append entry to resultSetData.
                        result_set_data.set(key, Value::Undefined);
                    }
                }
                // 3. NOTE: The number of elements in set.[[SetData]] may have increased during execution of otherRecord.[[Has]].
                // 4. Set thisSize to the number of elements in set.[[SetData]].
                this_size = set_table(agent, set).slots();
            }
        }
    } else {
        // a. Let keysIterator be ? GetIteratorFromMethod(otherRecord.[[SetObject]], otherRecord.[[Keys]]).
        let mut keys_iterator = ops::get_iterator_from_method(
            agent,
            Value::Object(other_record.set_object),
            other_record.keys,
        )?;
        // b. Let next be not-started.
        // c. Repeat, while next is not done,
        loop {
            // i. Set next to ? IteratorStepValue(keysIterator).
            // ii. If next is not done, then
            let Some(next) = ops::iterator_step_value(agent, &mut keys_iterator)? else {
                break;
            };
            // 1. Set next to CanonicalizeKeyedCollectionKey(next).
            let key = collection_key(agent, next);
            // 2. Let inThis be SetDataHas(set.[[SetData]], next).
            let in_this = set_table(agent, set).has(&key);
            // 3. If inThis is true, then
            if in_this {
                // a. NOTE: Because other is an arbitrary object, it is possible for its "keys" iterator to produce the same value more than once.
                // b. If SetDataHas(resultSetData, next) is false, then
                if !result_set_data.has(&key) {
                    // i. Append next to resultSetData.
                    result_set_data.set(key, Value::Undefined);
                }
            }
        }
    }
    // 7. Let result be OrdinaryObjectCreate(%Set.prototype%, « [[SetData]] »).
    let proto = agent.intrinsic("%Set.prototype%");
    let result = agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(SetData::default())),
    );
    // 8. Set result.[[SetData]] to resultSetData.
    *set_table(agent, result) = result_set_data;
    // 9. Return result.
    Ok(Value::Object(result))
}
