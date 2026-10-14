//! Set.prototype.symmetricDifference ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.symmetricdifference

use crate::support::collections::{SetData, collection_key, set_table, this_set};
use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// GetSetRecord(obj) (§24.2.1.2). Returns the fields of a Set Record
/// (§24.2.1.1) as ([[SetObject]], [[Size]], [[Has]], [[Keys]]). Duplicated
/// from `prototype_union.rs` (which keeps its copy private); move it into
/// `set/mod.rs` once more Set methods need it.
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

/// Set.prototype.symmetricDifference
pub(crate) fn prototype_symmetric_difference(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let set be the this value.
    // 2. Perform ? RequireInternalSlot(set, [[SetData]]).
    let set = this_set(agent, this)?;
    // 3. Let otherRecord be ? GetSetRecord(other).
    let (set_object, _size, _has, keys) = get_set_record(agent, args.get_or_undefined(0))?;
    // 4. Let keysIterator be ? GetIteratorFromMethod(otherRecord.[[SetObject]], otherRecord.[[Keys]]).
    let mut keys_iterator = ops::get_iterator_from_method(agent, Value::Object(set_object), keys)?;
    // The result object is created before the copy below (step 8 comes later
    // in the spec, but nothing observable happens in between), so that it can
    // be rooted while the keys iterator runs and may collect garbage.
    // 8. Let result be OrdinaryObjectCreate(%Set.prototype%, « [[SetData]] »).
    let set_proto = agent.intrinsic("%Set.prototype%");
    let result = agent.object_with_kind(
        Some(set_proto),
        ObjectKind::Native(Box::new(SetData::default())),
    );
    // 5. Let resultSetData be a copy of set.[[SetData]].
    let existing: Vec<Value> = {
        let table = set_table(agent, set);
        (0..table.slots())
            .filter_map(|i| table.entry(i))
            .map(|(k, _)| k)
            .collect()
    };
    for k in existing {
        // The keys of [[SetData]] are already canonical, so they are copied as they are.
        let key = collection_key(agent, k);
        set_table(agent, result).set(key, Value::Undefined);
    }
    // The iterator (and its next method) and the result stay rooted while
    // IteratorStepValue calls next(), which may collect garbage.
    let roots = [
        Value::Object(result),
        Value::Object(keys_iterator.iterator),
        keys_iterator.next_method,
    ];
    agent.with_roots(&roots, |agent| -> JsResult<Value> {
        // 6. Let next be not-started.
        // 7. Repeat, while next is not done,
        loop {
            // a. Set next to ? IteratorStepValue(keysIterator).
            let Some(next) = ops::iterator_step_value_rooted(agent, &mut keys_iterator)? else {
                // b. (next is done) the loop ends.
                break;
            };
            // b. If next is not done, then
            //    i. Set next to CanonicalizeKeyedCollectionKey(next).
            let key = collection_key(agent, next);
            //    ii. Let resultIndex be SetDataIndex(resultSetData, next).
            //    iii. If resultIndex is not-found, let alreadyInResult be false; else let alreadyInResult be true.
            let already_in_result = set_table(agent, result).has(&key);
            //    iv. If SetDataHas(set.[[SetData]], next) is true, then
            if set_table(agent, set).has(&key) {
                //    1. If alreadyInResult is true, set resultSetData[resultIndex] to empty.
                if already_in_result {
                    set_table(agent, result).delete(&key);
                }
            }
            //    v. Else,
            else {
                //    1. If alreadyInResult is false, append next to resultSetData.
                if !already_in_result {
                    set_table(agent, result).set(key, Value::Undefined);
                }
            }
        }
        // 9. Set result.[[SetData]] to resultSetData.
        // 10. Return result.
        Ok(Value::Object(result))
    })
}
