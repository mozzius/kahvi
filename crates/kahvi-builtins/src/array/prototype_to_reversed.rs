//! Array.prototype.toReversed ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.toreversed

use kahvi_core::prelude::*;

/// Array.prototype.toReversed
pub(crate) fn prototype_to_reversed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _array_ be ? ArrayCreate(_length_).
    // 4. Let _k_ be 0.
    // 5. Repeat, while _k_ < _length_,
    //    a. Let _from_ be ! ToString(𝔽(_length_ - _k_ - 1)).
    //    b. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    c. Let _fromValue_ be ? Get(_obj_, _from_).
    //    d. Perform ! CreateDataPropertyOrThrow(_array_, _propertyKey_, _fromValue_).
    //    e. Set _k_ to _k_ + 1.
    // 6. Return _array_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.toReversed"))
}
