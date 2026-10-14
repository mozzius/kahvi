//! Map.prototype.getOrInsert ( _key_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.getorinsert

use kahvi_core::prelude::*;

/// Map.prototype.getOrInsert
pub(crate) fn prototype_get_or_insert(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _map_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_map_, [[MapData]]).
    // 3. Set _key_ to CanonicalizeKeyedCollectionKey(_key_).
    // 4. For each Record { [[Key]], [[Value]] } _entry_ of _map_.[[MapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, return _entry_.[[Value]].
    // 5. Let _entry_ be the Record { [[Key]]: _key_, [[Value]]: _value_ }.
    // 6. Append _entry_ to _map_.[[MapData]].
    // 7. Return _value_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype.getOrInsert"))
}
