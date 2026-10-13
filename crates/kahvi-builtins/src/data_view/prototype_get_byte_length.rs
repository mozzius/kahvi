//! get DataView.prototype.byteLength
//!
//! Spec: https://tc39.es/ecma262/#sec-get-dataview.prototype.bytelength

use kahvi_core::prelude::*;

/// get DataView.prototype.byteLength
pub(crate) fn prototype_get_byte_length(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `DataView.prototype.byteLength` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[DataView]]).
    // 3. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
    // 4. Let _viewRecord_ be MakeDataViewWithBufferWitnessRecord(_obj_, ~seq-cst~).
    // 5. If IsViewOutOfBounds(_viewRecord_) is *true*, throw a *TypeError* exception.
    // 6. Let _size_ be GetViewByteLength(_viewRecord_).
    // 7. Return 𝔽(_size_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get DataView.prototype.byteLength"))
}
