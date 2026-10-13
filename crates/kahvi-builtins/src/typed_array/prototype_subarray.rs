//! %TypedArray%.prototype.subarray ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.subarray

use kahvi_core::prelude::*;

/// TypedArray.prototype.subarray
pub(crate) fn prototype_subarray(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a new _TypedArray_ whose element type is the element type of this _TypedArray_ and whose ArrayBuffer is the ArrayBuffer of this _TypedArray_, referencing the elements in the interval from _start_ (inclusive) to _end_ (exclusive). If either _start_ or _end_ is negative, it refers to an index from the end of the array, as opposed to from the beginning.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[TypedArrayName]]).
    // 3. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
    // 4. Let _buffer_ be _obj_.[[ViewedArrayBuffer]].
    // 5. Let _sourceRecord_ be MakeTypedArrayWithBufferWitnessRecord(_obj_, ~seq-cst~).
    // 6. If IsTypedArrayOutOfBounds(_sourceRecord_) is *true*, then
    //    a. Let _sourceLength_ be 0.
    // 7. Else,
    //    a. Let _sourceLength_ be TypedArrayLength(_sourceRecord_).
    // 8. Let _startIndex_ be ? ToClampedIndex(_start_, _sourceLength_).
    // 9. Let _elementSize_ be TypedArrayElementSize(_obj_).
    // 10. Let _sourceByteOffset_ be _obj_.[[ByteOffset]].
    // 11. Let _beginByteOffset_ be _sourceByteOffset_ + (_startIndex_ × _elementSize_).
    // 12. If _obj_.[[ArrayLength]] is ~auto~ and _end_ is *undefined*, then
    //    a. Return ? TypedArraySpeciesCreate(_obj_, « _buffer_, 𝔽(_beginByteOffset_) »).
    // 13. If _end_ is *undefined*, let _endIndex_ be _sourceLength_; else let _endIndex_ be ? ToClampedIndex(_end_, _sourceLength_).
    // 14. Let _newLength_ be max(_endIndex_ - _startIndex_, 0).
    // 15. Return ? TypedArraySpeciesCreate(_obj_, « _buffer_, 𝔽(_beginByteOffset_), 𝔽(_newLength_) »).
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.subarray"))
}
