//! WeakSet.prototype.add ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakset.prototype.add

use crate::support::collections::{
    can_be_held_weakly, collection_key, this_weak_set, weak_set_table,
};
use kahvi_core::prelude::*;

/// WeakSet.prototype.add
pub(crate) fn prototype_add(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let weakSet be the this value.
    // 2. Perform ? RequireInternalSlot(weakSet, [[WeakSetData]]).
    let weak_set = this_weak_set(agent, this)?;
    // 3. If CanBeHeldWeakly(value) is false, throw a TypeError exception.
    let value = args.get_or_undefined(0);
    if !can_be_held_weakly(agent, value) {
        return Err(agent.throw_type_error("WeakSet.prototype.add: value cannot be held weakly"));
    }
    // 4. For each element entry of weakSet.[[WeakSetData]], do
    //    a. If entry is not ~empty~ and SameValue(entry, value) is true, then
    //       i. Return weakSet.
    let key = collection_key(agent, value);
    if weak_set_table(agent, weak_set).has(&key) {
        return Ok(Value::Object(weak_set));
    }
    // 5. Append value to weakSet.[[WeakSetData]].
    weak_set_table(agent, weak_set).set(key, Value::Undefined);
    // 6. Return weakSet.
    Ok(Value::Object(weak_set))
}
