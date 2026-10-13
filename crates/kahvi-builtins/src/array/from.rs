//! Array.from ( _items_ [ , _mapper_ [ , _thisArg_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.from

use kahvi_core::prelude::*;

/// Array.from
pub(crate) fn from(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _ctor_ be the *this* value.
    // 2. If _mapper_ is *undefined*, then
    //    a. Let _mapping_ be *false*.
    // 3. Else,
    //    a. If IsCallable(_mapper_) is *false*, throw a *TypeError* exception.
    //    b. Let _mapping_ be *true*.
    // 4. Let _usingIterator_ be ? GetMethod(_items_, %Symbol.iterator%).
    // 5. If _usingIterator_ is not *undefined*, then
    //    a. If IsConstructor(_ctor_) is *true*, then
    //       i. Let _array_ be ? Construct(_ctor_).
    //    b. Else,
    //       i. Let _array_ be ! ArrayCreate(0).
    //    c. Let _iteratorRecord_ be ? GetIteratorFromMethod(_items_, _usingIterator_).
    //    d. Let _k_ be 0.
    //    e. Repeat,
    //       i. If _k_ ≥ 2^53 - 1, then
    //          1. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //          2. Return ? IteratorClose(_iteratorRecord_, _error_).
    //       ii. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //       iii. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
    //       iv. If _next_ is ~done~, then
    //          1. Perform ? Set(_array_, *"length"*, 𝔽(_k_), *true*).
    //          2. Return _array_.
    //       v. If _mapping_ is *true*, then
    //          1. Let _mappedValue_ be Completion(Call(_mapper_, _thisArg_, « _next_, 𝔽(_k_) »)).
    //          2. IfAbruptCloseIterator(_mappedValue_, _iteratorRecord_).
    //       vi. Else,
    //          1. Let _mappedValue_ be _next_.
    //       vii. Let _defineStatus_ be Completion(CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_)).
    //       viii. IfAbruptCloseIterator(_defineStatus_, _iteratorRecord_).
    //       ix. Set _k_ to _k_ + 1.
    // 6. NOTE: _items_ is not iterable so assume it is an array-like object.
    // 7. Let _arrayLike_ be ! ToObject(_items_).
    // 8. Let _length_ be ? LengthOfArrayLike(_arrayLike_).
    // 9. If IsConstructor(_ctor_) is *true*, then
    //    a. Let _array_ be ? Construct(_ctor_, « 𝔽(_length_) »).
    // 10. Else,
    //    a. Let _array_ be ? ArrayCreate(_length_).
    // 11. Let _k_ be 0.
    // 12. Repeat, while _k_ < _length_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kValue_ be ? Get(_arrayLike_, _propertyKey_).
    //    c. If _mapping_ is *true*, then
    //       i. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
    //    d. Else,
    //       i. Let _mappedValue_ be _kValue_.
    //    e. Perform ? CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_).
    //    f. Set _k_ to _k_ + 1.
    // 13. Perform ? Set(_array_, *"length"*, 𝔽(_length_), *true*).
    // 14. Return _array_.
    //
    // NOTE:
    // This method is an intentionally generic factory method; it does not require that its *this* value be the Array constructor. Therefore it can be transferred to or inherited by any other constructors that may be called with a single numeric argument.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.from"))
}
