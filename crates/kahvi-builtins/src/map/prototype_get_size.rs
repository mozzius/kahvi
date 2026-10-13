//! get Map.prototype.size
//!
//! Spec: https://tc39.es/ecma262/#sec-get-map.prototype.size

use kahvi_core::prelude::*;

/// get Map.prototype.size
pub(crate) fn prototype_get_size(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Map.prototype.size` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _map_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_map_, [[MapData]]).
    // 3. Let _count_ be 0.
    // 4. For each Record { [[Key]], [[Value]] } _entry_ of _map_.[[MapData]], do
    //    a. If _entry_.[[Key]] is not ~empty~, set _count_ to _count_ + 1.
    // 5. Return 𝔽(_count_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Map.prototype.size"))
}
