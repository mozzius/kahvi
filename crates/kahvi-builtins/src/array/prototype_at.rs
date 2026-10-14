//! Array.prototype.at ( _index_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.at

use kahvi_core::prelude::*;

/// Array.prototype.at
pub(crate) fn prototype_at(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _k_ be ? ToAbsoluteIndex(_index_, _length_).
    // 4. If _k_ < 0 or _k_ ≥ _length_, return *undefined*.
    // 5. Return ? Get(_obj_, ! ToString(𝔽(_k_))).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.at"))
}
