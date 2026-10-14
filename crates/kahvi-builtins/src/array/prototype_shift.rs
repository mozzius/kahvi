//! Array.prototype.shift ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.shift

use kahvi_core::prelude::*;

/// Array.prototype.shift
pub(crate) fn prototype_shift(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method removes the first element of the array and returns it.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If _length_ = 0, then
    //    a. Perform ? Set(_obj_, *"length"*, *+0*_𝔽, *true*).
    //    b. Return *undefined*.
    // 4. Let _first_ be ? Get(_obj_, *"0"*).
    // 5. Let _k_ be 1.
    // 6. Repeat, while _k_ < _length_,
    //    a. Let _from_ be ! ToString(𝔽(_k_)).
    //    b. Let _to_ be ! ToString(𝔽(_k_ - 1)).
    //    c. Let _fromPresent_ be ? HasProperty(_obj_, _from_).
    //    d. If _fromPresent_ is *true*, then
    //       i. Let _fromValue_ be ? Get(_obj_, _from_).
    //       ii. Perform ? Set(_obj_, _to_, _fromValue_, *true*).
    //    e. Else,
    //       i. Assert: _fromPresent_ is *false*.
    //       ii. Perform ? DeletePropertyOrThrow(_obj_, _to_).
    //    f. Set _k_ to _k_ + 1.
    // 7. Perform ? DeletePropertyOrThrow(_obj_, ! ToString(𝔽(_length_ - 1))).
    // 8. Perform ? Set(_obj_, *"length"*, 𝔽(_length_ - 1), *true*).
    // 9. Return _first_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.shift"))
}
