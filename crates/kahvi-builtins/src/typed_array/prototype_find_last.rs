//! %TypedArray%.prototype.findLast ( _predicate_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.findlast

use kahvi_core::prelude::*;

/// TypedArray.prototype.findLast
pub(crate) fn prototype_find_last(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.findLast` as defined in [sec-array.prototype.findlast].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. Let _findRecord_ be ? FindViaPredicate(_obj_, _length_, ~descending~, _predicate_, _thisArg_).
    // 5. Return _findRecord_.[[Value]].
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.findLast"))
}
