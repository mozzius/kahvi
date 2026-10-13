//! Array.prototype.fill ( _value_ [ , _start_ [ , _end_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.fill

use kahvi_core::prelude::*;

/// Array.prototype.fill
pub(crate) fn prototype_fill(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The _start_ argument is optional. If it is not provided, *+0*_𝔽 is used.
    // The _end_ argument is optional. If it is not provided, the length of the *this* value is used.
    // NOTE:
    // If _start_ is negative, it is treated as _length_ + _start_ where _length_ is the length of the array. If _end_ is negative, it is treated as _length_ + _end_.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _k_ be ? ToClampedIndex(_start_, _length_).
    // 4. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
    // 5. Repeat, while _k_ < _final_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Perform ? Set(_obj_, _propertyKey_, _value_, *true*).
    //    c. Set _k_ to _k_ + 1.
    // 6. Return _obj_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.fill"))
}
