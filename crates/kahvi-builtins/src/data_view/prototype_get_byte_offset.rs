//! get DataView.prototype.byteOffset
//!
//! Spec: https://tc39.es/ecma262/#sec-get-dataview.prototype.byteoffset

use kahvi_core::prelude::*;

/// get DataView.prototype.byteOffset
pub(crate) fn prototype_get_byte_offset(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `DataView.prototype.byteOffset` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[DataView]]).
    // 3. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
    // 4. Let _viewRecord_ be MakeDataViewWithBufferWitnessRecord(_obj_, ~seq-cst~).
    // 5. If IsViewOutOfBounds(_viewRecord_) is *true*, throw a *TypeError* exception.
    // 6. Let _offset_ be _obj_.[[ByteOffset]].
    // 7. Return 𝔽(_offset_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get DataView.prototype.byteOffset"))
}
