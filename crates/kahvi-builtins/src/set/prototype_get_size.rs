//! get Set.prototype.size
//!
//! Spec: https://tc39.es/ecma262/#sec-get-set.prototype.size

use kahvi_core::prelude::*;

/// get Set.prototype.size
pub(crate) fn prototype_get_size(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Set.prototype.size` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    // 3. Let _size_ be SetDataSize(_set_.[[SetData]]).
    // 4. Return 𝔽(_size_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Set.prototype.size"))
}
