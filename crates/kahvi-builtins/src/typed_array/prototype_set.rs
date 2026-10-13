//! %TypedArray%.prototype.set ( _source_ [ , _offset_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.set

use kahvi_core::prelude::*;

/// TypedArray.prototype.set
pub(crate) fn prototype_set(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method sets multiple values in this _TypedArray_, reading the values from _source_. The details differ based upon the type of _source_. The optional _offset_ value indicates the first element index in this _TypedArray_ where values are written. If omitted, it is assumed to be 0.
    // It performs the following steps when called:
    //
    // 1. Let _target_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_target_, [[TypedArrayName]]).
    // 3. Assert: _target_ has a [[ViewedArrayBuffer]] internal slot.
    // 4. Let _targetOffset_ be ? ToIntegerOrInfinity(_offset_).
    // 5. If _targetOffset_ < 0, throw a *RangeError* exception.
    // 6. If _source_ is an Object that has a [[TypedArrayName]] internal slot, then
    //    a. Perform ? SetTypedArrayFromTypedArray(_target_, _targetOffset_, _source_).
    // 7. Else,
    //    a. Perform ? SetTypedArrayFromArrayLike(_target_, _targetOffset_, _source_).
    // 8. Return *undefined*.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    // [subclause sec-settypedarrayfromarraylike: SetTypedArrayFromArrayLike (
    // _target_: a TypedArray,
    // _targetOffset_: a non-negative integer or +∞,
    // _source_: an ECMAScript language value, but not a TypedArray,
    // ): either a normal completion containing ~unused~ or a throw completion]
    // [subclause sec-settypedarrayfromtypedarray: SetTypedArrayFromTypedArray (
    // _target_: a TypedArray,
    // _targetOffset_: a non-negative integer or +∞,
    // _source_: a TypedArray,
    // ): either a normal completion containing ~unused~ or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.set"))
}
