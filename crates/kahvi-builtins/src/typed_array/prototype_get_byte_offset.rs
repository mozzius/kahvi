//! get %TypedArray%.prototype.byteOffset
//!
//! Spec: https://tc39.es/ecma262/#sec-get-%typedarray%.prototype.byteoffset

use kahvi_core::prelude::*;

/// get TypedArray.prototype.byteOffset
pub(crate) fn prototype_get_byte_offset(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // %TypedArray%`.prototype.byteOffset` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[TypedArrayName]]).
    // 3. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
    // 4. Let _taRecord_ be MakeTypedArrayWithBufferWitnessRecord(_obj_, ~seq-cst~).
    // 5. If IsTypedArrayOutOfBounds(_taRecord_) is *true*, return *+0*_𝔽.
    // 6. Let _offset_ be _obj_.[[ByteOffset]].
    // 7. Return 𝔽(_offset_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get TypedArray.prototype.byteOffset"))
}
