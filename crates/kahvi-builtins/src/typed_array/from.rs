//! %TypedArray%.from ( _source_ [ , _mapper_ [ , _thisArg_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.from

use kahvi_core::prelude::*;

/// TypedArray.from
pub(crate) fn from(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _ctor_ be the *this* value.
    // 2. If IsConstructor(_ctor_) is *false*, throw a *TypeError* exception.
    // 3. If _mapper_ is *undefined*, then
    //    a. Let _mapping_ be *false*.
    // 4. Else,
    //    a. If IsCallable(_mapper_) is *false*, throw a *TypeError* exception.
    //    b. Let _mapping_ be *true*.
    // 5. Let _usingIterator_ be ? GetMethod(_source_, %Symbol.iterator%).
    // 6. If _usingIterator_ is not *undefined*, then
    //    a. Let _values_ be ? IteratorToList(? GetIteratorFromMethod(_source_, _usingIterator_)).
    //    b. Let _length_ be the number of elements in _values_.
    //    c. Let _targetObj_ be ? TypedArrayCreateFromConstructor(_ctor_, « 𝔽(_length_) »).
    //    d. Let _k_ be 0.
    //    e. Repeat, while _k_ < _length_,
    //       i. Let _kValue_ be _values_[_k_].
    //       ii. If _mapping_ is *true*, then
    //          1. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
    //       iii. Else,
    //          1. Let _mappedValue_ be _kValue_.
    //       iv. Perform ? TypedArraySetElement(_targetObj_, 𝔽(_k_), _mappedValue_).
    //       v. Set _k_ to _k_ + 1.
    //    f. Return _targetObj_.
    // 7. NOTE: _source_ is not an iterable object, so assume it is already an array-like object.
    // 8. Let _arrayLike_ be ! ToObject(_source_).
    // 9. Let _length_ be ? LengthOfArrayLike(_arrayLike_).
    // 10. Let _targetObj_ be ? TypedArrayCreateFromConstructor(_ctor_, « 𝔽(_length_) »).
    // 11. Let _k_ be 0.
    // 12. Repeat, while _k_ < _length_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kValue_ be ? Get(_arrayLike_, _propertyKey_).
    //    c. If _mapping_ is *true*, then
    //       i. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
    //    d. Else,
    //       i. Let _mappedValue_ be _kValue_.
    //    e. Perform ? TypedArraySetElement(_targetObj_, 𝔽(_k_), _mappedValue_).
    //    f. Set _k_ to _k_ + 1.
    // 13. Return _targetObj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.from"))
}
