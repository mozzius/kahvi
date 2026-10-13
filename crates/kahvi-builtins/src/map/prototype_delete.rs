//! Map.prototype.delete ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.delete

use kahvi_core::prelude::*;

/// Map.prototype.delete
pub(crate) fn prototype_delete(
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
    //    a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, then
    //       i. Set _entry_.[[Key]] to ~empty~.
    //       ii. Set _entry_.[[Value]] to ~empty~.
    //       iii. Return *true*.
    // 5. Return *false*.
    //
    // NOTE:
    // The value ~empty~ is used as a specification device to indicate that an entry has been deleted. Actual implementations may take other actions such as physically removing the entry from internal data structures.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype.delete"))
}
