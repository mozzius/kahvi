//! WeakMap.prototype.has ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakmap.prototype.has

use kahvi_core::prelude::*;

/// WeakMap.prototype.has
pub(crate) fn prototype_has(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _weakMap_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakMap_, [[WeakMapData]]).
    // 3. If CanBeHeldWeakly(_key_) is *false*, return *false*.
    // 4. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, return *true*.
    // 5. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WeakMap.prototype.has"))
}
