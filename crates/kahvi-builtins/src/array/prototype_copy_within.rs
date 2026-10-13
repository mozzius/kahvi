//! Array.prototype.copyWithin ( _target_, _start_ [ , _end_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.copywithin

use kahvi_core::prelude::*;

/// Array.prototype.copyWithin
pub(crate) fn prototype_copy_within(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The _end_ argument is optional. If it is not provided, the length of the *this* value is used.
    // NOTE:
    // If _target_ is negative, it is treated as _length_ + _target_ where _length_ is the length of the array. If _start_ is negative, it is treated as _length_ + _start_. If _end_ is negative, it is treated as _length_ + _end_.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _to_ be ? ToClampedIndex(_target_, _length_).
    // 4. Let _from_ be ? ToClampedIndex(_start_, _length_).
    // 5. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
    // 6. Let _count_ be min(_final_ - _from_, _length_ - _to_).
    // 7. If _from_ < _to_ and _to_ < _from_ + _count_, then
    //    a. Let _direction_ be -1.
    //    b. Set _from_ to _from_ + _count_ - 1.
    //    c. Set _to_ to _to_ + _count_ - 1.
    // 8. Else,
    //    a. Let _direction_ be 1.
    // 9. Repeat, while _count_ > 0,
    //    a. Let _fromKey_ be ! ToString(𝔽(_from_)).
    //    b. Let _toKey_ be ! ToString(𝔽(_to_)).
    //    c. Let _fromPresent_ be ? HasProperty(_obj_, _fromKey_).
    //    d. If _fromPresent_ is *true*, then
    //       i. Let _fromValue_ be ? Get(_obj_, _fromKey_).
    //       ii. Perform ? Set(_obj_, _toKey_, _fromValue_, *true*).
    //    e. Else,
    //       i. Assert: _fromPresent_ is *false*.
    //       ii. Perform ? DeletePropertyOrThrow(_obj_, _toKey_).
    //    f. Set _from_ to _from_ + _direction_.
    //    g. Set _to_ to _to_ + _direction_.
    //    h. Set _count_ to _count_ - 1.
    // 10. Return _obj_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.copyWithin"))
}
