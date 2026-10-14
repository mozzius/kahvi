//! WeakMap.prototype.has ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakmap.prototype.has

use crate::support::collections::{
    can_be_held_weakly, collection_key, this_weak_map, weak_map_table,
};
use kahvi_core::prelude::*;

/// WeakMap.prototype.has
pub(crate) fn prototype_has(
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
    let key = args.get_or_undefined(0);
    if !can_be_held_weakly(agent, key) {
        return Ok(Value::Boolean(false));
    }
    // 4. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, return *true*.
    // 5. Return *false*.
    // (Keys are objects or non-registered symbols here, so the table lookup by
    // SameValueZero is the same as SameValue.)
    let key = collection_key(agent, key);
    let found = weak_map_table(agent, weak_map).has(&key);
    Ok(Value::Boolean(found))
}
