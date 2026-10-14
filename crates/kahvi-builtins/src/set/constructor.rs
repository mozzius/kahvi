//! Set ( [ _iterable_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-set-iterable

use crate::support::collections::{SetData, create_collection};
use kahvi_core::prelude::*;

/// Set
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, throw a TypeError exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor Set requires 'new'"));
    };
    // 2. Let set be ? OrdinaryCreateFromConstructor(NewTarget, "%Set.prototype%", « [[SetData]] »).
    // 3. Set set.[[SetData]] to a new empty List.
    let set = create_collection(agent, new_target, "%Set.prototype%", SetData::default())?;
    // 4. If iterable is either undefined or null, return set.
    let iterable = args.get_or_undefined(0);
    if iterable.is_nullish() {
        return Ok(Value::Object(set));
    }
    // 5. Let adder be ? Get(set, "add").
    let add_key = agent.key("add");
    let adder = ops::get(agent, set, add_key)?;
    // 6. If IsCallable(adder) is false, throw a TypeError exception.
    if !ops::is_callable(agent, adder) {
        return Err(agent.throw_type_error("Set.prototype.add is not a function"));
    }
    // 7. Return ? AddValuesFromIterable(set, iterable, adder).
    add_values_from_iterable(agent, set, iterable, adder)
}

/// AddValuesFromIterable(target, iterable, adder)
///
/// Only used by the Set constructor, so it lives here rather than in
/// `support::collections` (which has AddEntriesFromIterable for Map).
fn add_values_from_iterable(
    agent: &mut Agent,
    target: Object,
    iterable: Value,
    adder: Value,
) -> JsResult<Value> {
    // 1. Let iteratorRecord be ? GetIterator(iterable, sync).
    let mut record = ops::get_iterator(agent, iterable, ops::IteratorKind::Sync)?;
    // 2. Repeat,
    loop {
        // a. Let next be ? IteratorStepValue(iteratorRecord).
        // b. If next is done, return target.
        let Some(next) = ops::iterator_step_value(agent, &mut record)? else {
            return Ok(Value::Object(target));
        };
        // c. Let status be Completion(Call(adder, target, « next »)).
        // d. IfAbruptCloseIterator(status, iteratorRecord).
        if let Err(e) = ops::call(agent, adder, Value::Object(target), &[next]) {
            return ops::iterator_close(agent, &record, Err(e));
        }
    }
}
