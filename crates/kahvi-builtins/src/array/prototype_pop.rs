//! Array.prototype.pop ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.pop

use kahvi_core::prelude::*;

/// Array.prototype.pop
pub(crate) fn prototype_pop(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method removes the last element of the array and returns it.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If _length_ = 0, then
    //    a. Perform ? Set(_obj_, *"length"*, *+0*_𝔽, *true*).
    //    b. Return *undefined*.
    // 4. Assert: _length_ > 0.
    // 5. Let _newLength_ be 𝔽(_length_ - 1).
    // 6. Let _index_ be ! ToString(_newLength_).
    // 7. Let _element_ be ? Get(_obj_, _index_).
    // 8. Perform ? DeletePropertyOrThrow(_obj_, _index_).
    // 9. Perform ? Set(_obj_, *"length"*, _newLength_, *true*).
    // 10. Return _element_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.pop"))
}
