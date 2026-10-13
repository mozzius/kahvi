//! Array.prototype.splice ( _start_, _deleteCount_, ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.splice

use kahvi_core::prelude::*;

/// Array.prototype.splice
pub(crate) fn prototype_splice(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method deletes the _deleteCount_ elements of the array starting at integer index _start_ and replaces them with the elements of _items_. It returns an Array containing the deleted elements (if any).
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _actualStart_ be ? ToClampedIndex(_start_, _length_).
    // 4. Let _itemCount_ be the number of elements in _items_.
    // 5. If _start_ is not present, then
    //    a. Let _actualDeleteCount_ be 0.
    // 6. Else if _deleteCount_ is not present, then
    //    a. Let _actualDeleteCount_ be _length_ - _actualStart_.
    //    b. Assert: _actualDeleteCount_ ≥ 0.
    // 7. Else,
    //    a. Let _dc_ be ? ToIntegerOrInfinity(_deleteCount_).
    //    b. Let _actualDeleteCount_ be the result of clamping _dc_ between 0 and _length_ - _actualStart_.
    // 8. If _length_ + _itemCount_ - _actualDeleteCount_ > 2^53 - 1, throw a *TypeError* exception.
    // 9. Let _deletedArray_ be ? ArraySpeciesCreate(_obj_, _actualDeleteCount_).
    // 10. Let _k_ be 0.
    // 11. Repeat, while _k_ < _actualDeleteCount_,
    //    a. Let _from_ be ! ToString(𝔽(_actualStart_ + _k_)).
    //    b. If ? HasProperty(_obj_, _from_) is *true*, then
    //       i. Let _fromValue_ be ? Get(_obj_, _from_).
    //       ii. Perform ? CreateDataPropertyOrThrow(_deletedArray_, ! ToString(𝔽(_k_)), _fromValue_).
    //    c. Set _k_ to _k_ + 1.
    // 12. Perform ? Set(_deletedArray_, *"length"*, 𝔽(_actualDeleteCount_), *true*).
    // 13. If _itemCount_ < _actualDeleteCount_, then
    //    a. Set _k_ to _actualStart_.
    //    b. Repeat, while _k_ < (_length_ - _actualDeleteCount_),
    //       i. Let _from_ be ! ToString(𝔽(_k_ + _actualDeleteCount_)).
    //       ii. Let _to_ be ! ToString(𝔽(_k_ + _itemCount_)).
    //       iii. If ? HasProperty(_obj_, _from_) is *true*, then
    //          1. Let _fromValue_ be ? Get(_obj_, _from_).
    //          2. Perform ? Set(_obj_, _to_, _fromValue_, *true*).
    //       iv. Else,
    //          1. Perform ? DeletePropertyOrThrow(_obj_, _to_).
    //       v. Set _k_ to _k_ + 1.
    //    c. Set _k_ to _length_.
    //    d. Repeat, while _k_ > (_length_ - _actualDeleteCount_ + _itemCount_),
    //       i. Perform ? DeletePropertyOrThrow(_obj_, ! ToString(𝔽(_k_ - 1))).
    //       ii. Set _k_ to _k_ - 1.
    // 14. Else if _itemCount_ > _actualDeleteCount_, then
    //    a. Set _k_ to (_length_ - _actualDeleteCount_).
    //    b. Repeat, while _k_ > _actualStart_,
    //       i. Let _from_ be ! ToString(𝔽(_k_ + _actualDeleteCount_ - 1)).
    //       ii. Let _to_ be ! ToString(𝔽(_k_ + _itemCount_ - 1)).
    //       iii. If ? HasProperty(_obj_, _from_) is *true*, then
    //          1. Let _fromValue_ be ? Get(_obj_, _from_).
    //          2. Perform ? Set(_obj_, _to_, _fromValue_, *true*).
    //       iv. Else,
    //          1. Perform ? DeletePropertyOrThrow(_obj_, _to_).
    //       v. Set _k_ to _k_ - 1.
    // 15. Set _k_ to _actualStart_.
    // 16. For each element _item_ of _items_, do
    //    a. Perform ? Set(_obj_, ! ToString(𝔽(_k_)), _item_, *true*).
    //    b. Set _k_ to _k_ + 1.
    // 17. Perform ? Set(_obj_, *"length"*, 𝔽(_length_ - _actualDeleteCount_ + _itemCount_), *true*).
    // 18. Return _deletedArray_.
    //
    // NOTE:
    // The explicit setting of the *"length"* property in steps [step-array-proto-splice-set-length] and [step-array-proto-splice-set-length-2] is intended to ensure the lengths are correct even when the objects are not built-in Arrays.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.splice"))
}
