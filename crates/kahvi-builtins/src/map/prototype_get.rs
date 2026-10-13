//! Map.prototype.get ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.get

use kahvi_core::prelude::*;

/// Map.prototype.get
pub(crate) fn prototype_get(
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
    // 5. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype.get"))
}
