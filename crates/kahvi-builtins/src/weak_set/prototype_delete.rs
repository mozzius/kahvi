//! WeakSet.prototype.delete ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakset.prototype.delete

use crate::support::collections::{
    can_be_held_weakly, collection_key, this_weak_set, weak_set_table,
};
use kahvi_core::prelude::*;

/// WeakSet.prototype.delete
pub(crate) fn prototype_delete(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let weakSet be the this value.
    // 2. Perform ? RequireInternalSlot(weakSet, [[WeakSetData]]).
    let weak_set = this_weak_set(agent, this)?;
    // 3. If CanBeHeldWeakly(value) is false, return false.
    let value = args.get_or_undefined(0);
    if !can_be_held_weakly(agent, value) {
        return Ok(Value::Boolean(false));
    }
    // 4. For each element entry of weakSet.[[WeakSetData]], do
    //    a. If entry is not ~empty~ and SameValue(entry, value) is true, then
    //       i. Replace the element of weakSet.[[WeakSetData]] whose value is entry with an element whose value is ~empty~.
    //       ii. Return true.
    let key = collection_key(agent, value);
    // 5. Return false.
    Ok(Value::Boolean(weak_set_table(agent, weak_set).delete(&key)))
}
