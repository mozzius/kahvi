//! WeakSet.prototype.has ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakset.prototype.has

use kahvi_core::prelude::*;

/// WeakSet.prototype.has
pub(crate) fn prototype_has(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _weakSet_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakSet_, [[WeakSetData]]).
    // 3. If CanBeHeldWeakly(_value_) is *false*, return *false*.
    // 4. For each element _entry_ of _weakSet_.[[WeakSetData]], do
    //    a. If _entry_ is not ~empty~ and SameValue(_entry_, _value_) is *true*, return *true*.
    // 5. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WeakSet.prototype.has"))
}
