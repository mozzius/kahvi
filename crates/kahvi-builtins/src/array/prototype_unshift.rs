//! Array.prototype.unshift ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.unshift

use kahvi_core::prelude::*;

/// Array.prototype.unshift
pub(crate) fn prototype_unshift(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method prepends the arguments to the start of the array, such that their order within the array is the same as the order in which they appear in the argument list.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _argCount_ be the number of elements in _items_.
    // 4. If _argCount_ > 0, then
    //    a. If _length_ + _argCount_ > 2^53 - 1, throw a *TypeError* exception.
    //    b. Let _k_ be _length_.
    //    c. Repeat, while _k_ > 0,
    //       i. Let _from_ be ! ToString(𝔽(_k_ - 1)).
    //       ii. Let _to_ be ! ToString(𝔽(_k_ + _argCount_ - 1)).
    //       iii. Let _fromPresent_ be ? HasProperty(_obj_, _from_).
    //       iv. If _fromPresent_ is *true*, then
    //          1. Let _fromValue_ be ? Get(_obj_, _from_).
    //          2. Perform ? Set(_obj_, _to_, _fromValue_, *true*).
    //       v. Else,
    //          1. Assert: _fromPresent_ is *false*.
    //          2. Perform ? DeletePropertyOrThrow(_obj_, _to_).
    //       vi. Set _k_ to _k_ - 1.
    //    d. Let _j_ be 0.
    //    e. For each element _item_ of _items_, do
    //       i. Perform ? Set(_obj_, ! ToString(𝔽(_j_)), _item_, *true*).
    //       ii. Set _j_ to _j_ + 1.
    // 5. Perform ? Set(_obj_, *"length"*, 𝔽(_length_ + _argCount_), *true*).
    // 6. Return 𝔽(_length_ + _argCount_).
    //
    // The *"length"* property of this method is *1*_𝔽.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.unshift"))
}
