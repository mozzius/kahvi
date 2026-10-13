//! get %TypedArray%.prototype.length
//!
//! Spec: https://tc39.es/ecma262/#sec-get-%typedarray%.prototype.length

use kahvi_core::prelude::*;

/// get TypedArray.prototype.length
pub(crate) fn prototype_get_length(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // %TypedArray%`.prototype.length` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[TypedArrayName]]).
    // 3. Assert: _obj_ has [[ViewedArrayBuffer]] and [[ArrayLength]] internal slots.
    // 4. Let _taRecord_ be MakeTypedArrayWithBufferWitnessRecord(_obj_, ~seq-cst~).
    // 5. If IsTypedArrayOutOfBounds(_taRecord_) is *true*, return *+0*_𝔽.
    // 6. Let _length_ be TypedArrayLength(_taRecord_).
    // 7. Return 𝔽(_length_).
    //
    // This function is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get TypedArray.prototype.length"))
}
