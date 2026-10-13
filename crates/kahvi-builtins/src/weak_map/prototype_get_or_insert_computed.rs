//! WeakMap.prototype.getOrInsertComputed ( _key_, _callback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakmap.prototype.getorinsertcomputed

use kahvi_core::prelude::*;

/// WeakMap.prototype.getOrInsertComputed
pub(crate) fn prototype_get_or_insert_computed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _weakMap_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakMap_, [[WeakMapData]]).
    // 3. If CanBeHeldWeakly(_key_) is *false*, throw a *TypeError* exception.
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 5. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, return _entry_.[[Value]].
    // 6. Let _value_ be ? Call(_callback_, *undefined*, « _key_ »).
    // 7. NOTE: The WeakMap may have been modified during execution of _callback_.
    // 8. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, then
    //       i. Set _entry_.[[Value]] to _value_.
    //       ii. Return _value_.
    // 9. Let _entry_ be the Record { [[Key]]: _key_, [[Value]]: _value_ }.
    // 10. Append _entry_ to _weakMap_.[[WeakMapData]].
    // 11. Return _value_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WeakMap.prototype.getOrInsertComputed"))
}
