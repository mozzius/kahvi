//! Map.prototype.getOrInsertComputed ( _key_, _callback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.getorinsertcomputed

use kahvi_core::prelude::*;

/// Map.prototype.getOrInsertComputed
pub(crate) fn prototype_get_or_insert_computed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _map_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_map_, [[MapData]]).
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 4. Set _key_ to CanonicalizeKeyedCollectionKey(_key_).
    // 5. For each Record { [[Key]], [[Value]] } _entry_ of _map_.[[MapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, return _entry_.[[Value]].
    // 6. Let _value_ be ? Call(_callback_, *undefined*, « _key_ »).
    // 7. NOTE: The Map may have been modified during execution of _callback_.
    // 8. For each Record { [[Key]], [[Value]] } _entry_ of _map_.[[MapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, then
    //       i. Set _entry_.[[Value]] to _value_.
    //       ii. Return _value_.
    // 9. Let _entry_ be the Record { [[Key]]: _key_, [[Value]]: _value_ }.
    // 10. Append _entry_ to _map_.[[MapData]].
    // 11. Return _value_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype.getOrInsertComputed"))
}
