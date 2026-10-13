//! WeakMap.prototype.set ( _key_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakmap.prototype.set

use kahvi_core::prelude::*;

/// WeakMap.prototype.set
pub(crate) fn prototype_set(
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
    // 4. For each Record { [[Key]], [[Value]] } _entry_ of _weakMap_.[[WeakMapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, then
    //       i. Set _entry_.[[Value]] to _value_.
    //       ii. Return _weakMap_.
    // 5. Let _entry_ be the Record { [[Key]]: _key_, [[Value]]: _value_ }.
    // 6. Append _entry_ to _weakMap_.[[WeakMapData]].
    // 7. Return _weakMap_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WeakMap.prototype.set"))
}
