//! Array.prototype.push ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.push

use kahvi_core::prelude::*;

/// Array.prototype.push
pub(crate) fn prototype_push(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method appends the arguments to the end of the array, in the order in which they appear. It returns the new length of the array.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _argCount_ be the number of elements in _items_.
    // 4. If _length_ + _argCount_ > 2^53 - 1, throw a *TypeError* exception.
    // 5. For each element _item_ of _items_, do
    //    a. Perform ? Set(_obj_, ! ToString(𝔽(_length_)), _item_, *true*).
    //    b. Set _length_ to _length_ + 1.
    // 6. Perform ? Set(_obj_, *"length"*, 𝔽(_length_), *true*).
    // 7. Return 𝔽(_length_).
    //
    // The *"length"* property of this method is *1*_𝔽.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.push"))
}
