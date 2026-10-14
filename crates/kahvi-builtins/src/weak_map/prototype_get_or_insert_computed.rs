//! WeakMap.prototype.getOrInsertComputed ( _key_, _callback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakmap.prototype.getorinsertcomputed

use crate::support::collections::{
    can_be_held_weakly, collection_key, this_weak_map, weak_map_table,
};
use kahvi_core::prelude::*;

/// WeakMap.prototype.getOrInsertComputed
pub(crate) fn prototype_get_or_insert_computed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _weakMap_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakMap_, [[WeakMapData]]).
    let weak_map = this_weak_map(agent, this)?;
    // 3. If CanBeHeldWeakly(_key_) is *false*, throw a *TypeError* exception.
    let key_arg = args.get_or_undefined(0);
    if !can_be_held_weakly(agent, key_arg) {
        return Err(
            agent.throw_type_error("WeakMap key must be an object or a non-registered symbol")
        );
    }
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    let callback = args.get_or_undefined(1);
    if !ops::is_callable(agent, callback) {
        return Err(agent
            .throw_type_error("WeakMap.prototype.getOrInsertComputed callback is not a function"));
    }
    let key = collection_key(agent, key_arg);
    // 5. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, return _entry_.[[Value]].
    if let Some(existing) = weak_map_table(agent, weak_map).get(&key) {
        return Ok(existing);
    }
    // 6. Let _value_ be ? Call(_callback_, *undefined*, « _key_ »).
    let value = ops::call(agent, callback, Value::Undefined, &[key_arg])?;
    // 7. NOTE: The WeakMap may have been modified during execution of _callback_.
    // 8. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, then
    //       i. Set _entry_.[[Value]] to _value_.
    //       ii. Return _value_.
    let table = weak_map_table(agent, weak_map);
    if table.has(&key) {
        table.set(key, value);
        return Ok(value);
    }
    // 9. Let _entry_ be the Record { [[Key]]: _key_, [[Value]]: _value_ }.
    // 10. Append _entry_ to _weakMap_.[[WeakMapData]].
    table.set(key, value);
    // 11. Return _value_.
    Ok(value)
}
