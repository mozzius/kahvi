//! Array.prototype.toSpliced ( _start_, _skipCount_, ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.tospliced

use kahvi_core::prelude::*;

/// Array.prototype.toSpliced
pub(crate) fn prototype_to_spliced(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _actualStart_ be ? ToClampedIndex(_start_, _length_).
    // 4. Let _insertCount_ be the number of elements in _items_.
    // 5. Let _maxSkipCount_ be _length_ - _actualStart_.
    // 6. If _start_ is not present, then
    //    a. Let _actualSkipCount_ be 0.
    // 7. Else if _skipCount_ is not present, then
    //    a. Let _actualSkipCount_ be _maxSkipCount_.
    // 8. Else,
    //    a. Let _actualSkipCount_ be the result of clamping ? ToIntegerOrInfinity(_skipCount_) between 0 and _maxSkipCount_.
    // 9. Let _newLength_ be _length_ + _insertCount_ - _actualSkipCount_.
    // 10. Assert: _newLength_ ≥ 0.
    // 11. If _newLength_ > 2^53 - 1, throw a *TypeError* exception.
    // 12. Let _newArray_ be ? ArrayCreate(_newLength_).
    // 13. Let _writeIndex_ be 0.
    // 14. Let _readIndex_ be _actualStart_ + _actualSkipCount_.
    // 15. Repeat, while _writeIndex_ < _actualStart_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_writeIndex_)).
    //    b. Let _iValue_ be ? Get(_obj_, _propertyKey_).
    //    c. Perform ! CreateDataPropertyOrThrow(_newArray_, _propertyKey_, _iValue_).
    //    d. Set _writeIndex_ to _writeIndex_ + 1.
    // 16. For each element _item_ of _items_, do
    //    a. Let _propertyKey_ be ! ToString(𝔽(_writeIndex_)).
    //    b. Perform ! CreateDataPropertyOrThrow(_newArray_, _propertyKey_, _item_).
    //    c. Set _writeIndex_ to _writeIndex_ + 1.
    // 17. Repeat, while _writeIndex_ < _newLength_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_writeIndex_)).
    //    b. Let _from_ be ! ToString(𝔽(_readIndex_)).
    //    c. Let _fromValue_ be ? Get(_obj_, _from_).
    //    d. Perform ! CreateDataPropertyOrThrow(_newArray_, _propertyKey_, _fromValue_).
    //    e. Set _writeIndex_ to _writeIndex_ + 1.
    //    f. Set _readIndex_ to _readIndex_ + 1.
    // 18. Return _newArray_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.toSpliced"))
}
