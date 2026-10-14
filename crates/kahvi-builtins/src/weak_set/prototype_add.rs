//! WeakSet.prototype.add ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakset.prototype.add

use kahvi_core::prelude::*;

/// WeakSet.prototype.add
pub(crate) fn prototype_add(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _weakSet_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakSet_, [[WeakSetData]]).
    // 3. If CanBeHeldWeakly(_value_) is *false*, throw a *TypeError* exception.
    // 4. For each element _entry_ of _weakSet_.[[WeakSetData]], do
    //    a. If _entry_ is not ~empty~ and SameValue(_entry_, _value_) is *true*, then
    //       i. Return _weakSet_.
    // 5. Append _value_ to _weakSet_.[[WeakSetData]].
    // 6. Return _weakSet_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WeakSet.prototype.add"))
}
