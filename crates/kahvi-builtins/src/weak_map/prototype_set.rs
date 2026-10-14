//! WeakMap.prototype.set ( _key_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakmap.prototype.set

use crate::support::collections::{
    can_be_held_weakly, collection_key, this_weak_map, weak_map_table,
};
use kahvi_core::prelude::*;

/// WeakMap.prototype.set
pub(crate) fn prototype_set(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _weakMap_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakMap_, [[WeakMapData]]).
    // 3. If CanBeHeldWeakly(_key_) is *false*, throw a *TypeError* exception.
    // 4. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, then
    //       i. Set _entry_.[[Value]] to _value_.
    //       ii. Return _weakMap_.
    // 5. Let _entry_ be the Record { [[Key]]: _key_, [[Value]]: _value_ }.
    // 6. Append _entry_ to _weakMap_.[[WeakMapData]].
    // 7. Return _weakMap_.
    // 1. Let weakMap be the this value.
    // 2. Perform ? RequireInternalSlot(weakMap, [[WeakMapData]]).
    let weak_map = this_weak_map(agent, this)?;
    // 3. If CanBeHeldWeakly(key) is false, throw a TypeError exception.
    let key_arg = args.get_or_undefined(0);
    if !can_be_held_weakly(agent, key_arg) {
        return Err(
            agent.throw_type_error("WeakMap key must be an object or a non-registered symbol")
        );
    }
    let key = collection_key(agent, key_arg);
    let value = args.get_or_undefined(1);
    // 4. For each Record { [[Key]], [[Value]] } entry of weakMap.[[WeakMapData]], do
    //    a. If entry.[[Key]] is not empty and SameValue(entry.[[Key]], key) is true, then
    //       i. Set entry.[[Value]] to value.
    //       ii. Return weakMap.
    // 5. Let entry be the Record { [[Key]]: key, [[Value]]: value }.
    // 6. Append entry to weakMap.[[WeakMapData]].
    //    (OrderedTable::set updates an existing entry in place or appends a new one.)
    weak_map_table(agent, weak_map).set(key, value);
    // 7. Return weakMap.
    Ok(Value::Object(weak_map))
}
