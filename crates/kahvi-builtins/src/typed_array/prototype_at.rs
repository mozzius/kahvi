//! %TypedArray%.prototype.at ( _index_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.at

use kahvi_core::prelude::*;

/// TypedArray.prototype.at
pub(crate) fn prototype_at(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. Let _k_ be ? ToAbsoluteIndex(_index_, _length_).
    // 5. If _k_ < 0 or _k_ ≥ _length_, return *undefined*.
    // 6. Return TypedArrayGetElement(_obj_, 𝔽(_k_)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.at"))
}
