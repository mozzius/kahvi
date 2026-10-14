//! WeakMap.prototype.delete ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakmap.prototype.delete

use crate::support::collections::{
    can_be_held_weakly, collection_key, this_weak_map, weak_map_table,
};
use kahvi_core::prelude::*;

/// WeakMap.prototype.delete
pub(crate) fn prototype_delete(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _weakMap_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakMap_, [[WeakMapData]]).
    let weak_map = this_weak_map(agent, this)?;
    // 3. If CanBeHeldWeakly(_key_) is *false*, return *false*.
    let key_arg = args.get_or_undefined(0);
    if !can_be_held_weakly(agent, key_arg) {
        return Ok(Value::Boolean(false));
    }
    let key = collection_key(agent, key_arg);
    // 4. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, then
    //       i. Set _entry_.[[Key]] to ~empty~.
    //       ii. Set _entry_.[[Value]] to ~empty~.
    //       iii. Return *true*.
    // 5. Return *false*.
    //    (OrderedTable::delete empties the matching entry, if any, and reports whether one was found.)
    let deleted = weak_map_table(agent, weak_map).delete(&key);
    Ok(Value::Boolean(deleted))
}
