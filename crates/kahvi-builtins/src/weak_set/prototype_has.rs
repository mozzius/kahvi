//! WeakSet.prototype.has ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakset.prototype.has

use crate::support::collections::{
    can_be_held_weakly, collection_key, this_weak_set, weak_set_table,
};
use kahvi_core::prelude::*;

/// WeakSet.prototype.has
pub(crate) fn prototype_has(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _weakSet_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakSet_, [[WeakSetData]]).
    let weak_set = this_weak_set(agent, this)?;
    // 3. If CanBeHeldWeakly(_value_) is *false*, return *false*.
    let value = args.get_or_undefined(0);
    if !can_be_held_weakly(agent, value) {
        return Ok(Value::Boolean(false));
    }
    // 4. For each element _entry_ of _weakSet_.[[WeakSetData]], do
    //    a. If _entry_ is not ~empty~ and SameValue(_entry_, _value_) is *true*, return *true*.
    let key = collection_key(agent, value);
    if weak_set_table(agent, weak_set).has(&key) {
        return Ok(Value::Boolean(true));
    }
    // 5. Return *false*.
    Ok(Value::Boolean(false))
}
