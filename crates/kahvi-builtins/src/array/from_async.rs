//! Array.fromAsync ( _items_ [ , _mapper_ [ , _thisArg_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.fromasync

use kahvi_core::prelude::*;

/// Array.fromAsync
pub(crate) fn from_async(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This async function performs the following steps when called:
    //
    // 1. Let _ctor_ be the *this* value.
    // 2. Let _mapping_ be *false*.
    // 3. If _mapper_ is not *undefined*, then
    //    a. If IsCallable(_mapper_) is *false*, throw a *TypeError* exception.
    //    b. Set _mapping_ to *true*.
    // 4. Let _iteratorRecord_ be *undefined*.
    // 5. Let _usingAsyncIterator_ be ? GetMethod(_items_, %Symbol.asyncIterator%).
    // 6. If _usingAsyncIterator_ is *undefined*, then
    //    a. Let _usingSyncIterator_ be ? GetMethod(_items_, %Symbol.iterator%).
    //    b. If _usingSyncIterator_ is not *undefined*, then
    //       i. Set _iteratorRecord_ to CreateAsyncFromSyncIterator(? GetIteratorFromMethod(_items_, _usingSyncIterator_)).
    // 7. Else,
    //    a. Set _iteratorRecord_ to ? GetIteratorFromMethod(_items_, _usingAsyncIterator_).
    // 8. If _iteratorRecord_ is not *undefined*, then
    //    a. If IsConstructor(_ctor_) is *true*, then
    //       i. Let _array_ be ? Construct(_ctor_).
    //    b. Else,
    //       i. Let _array_ be ! ArrayCreate(0).
    //    c. Let _k_ be 0.
    //    d. Repeat,
    //       i. If _k_ ≥ 2^53 - 1, then
    //          1. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //          2. Return ? AsyncIteratorClose(_iteratorRecord_, _error_).
    //       ii. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //       iii. Let _nextResult_ be ? Call(_iteratorRecord_.[[NextMethod]], _iteratorRecord_.[[Iterator]]).
    //       iv. Set _nextResult_ to ? Await(_nextResult_).
    //       v. If _nextResult_ is not an Object, throw a *TypeError* exception.
    //       vi. Let _done_ be ? IteratorComplete(_nextResult_).
    //       vii. If _done_ is *true*, then
    //          1. Perform ? Set(_array_, *"length"*, 𝔽(_k_), *true*).
    //          2. Return _array_.
    //       viii. Let _nextValue_ be ? IteratorValue(_nextResult_).
    //       ix. If _mapping_ is *true*, then
    //          1. Let _mappedValue_ be Completion(Call(_mapper_, _thisArg_, « _nextValue_, 𝔽(_k_) »)).
    //          2. IfAbruptCloseAsyncIterator(_mappedValue_, _iteratorRecord_).
    //          3. Set _mappedValue_ to Completion(Await(_mappedValue_)).
    //          4. IfAbruptCloseAsyncIterator(_mappedValue_, _iteratorRecord_).
    //       x. Else,
    //          1. Let _mappedValue_ be _nextValue_.
    //       xi. Let _defineStatus_ be Completion(CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_)).
    //       xii. IfAbruptCloseAsyncIterator(_defineStatus_, _iteratorRecord_).
    //       xiii. Set _k_ to _k_ + 1.
    // 9. Else,
    //    a. NOTE: _items_ is neither async iterable nor iterable so assume it is an array-like object.
    //    b. Let _arrayLike_ be ! ToObject(_items_).
    //    c. Let _length_ be ? LengthOfArrayLike(_arrayLike_).
    //    d. If IsConstructor(_ctor_) is *true*, then
    //       i. Let _array_ be ? Construct(_ctor_, « 𝔽(_length_) »).
    //    e. Else,
    //       i. Let _array_ be ? ArrayCreate(_length_).
    //    f. Let _k_ be 0.
    //    g. Repeat, while _k_ < _length_,
    //       i. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //       ii. Let _kValue_ be ? Get(_arrayLike_, _propertyKey_).
    //       iii. Set _kValue_ to ? Await(_kValue_).
    //       iv. If _mapping_ is *true*, then
    //          1. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
    //          2. Set _mappedValue_ to ? Await(_mappedValue_).
    //       v. Else,
    //          1. Let _mappedValue_ be _kValue_.
    //       vi. Perform ? CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_).
    //       vii. Set _k_ to _k_ + 1.
    //    h. Perform ? Set(_array_, *"length"*, 𝔽(_length_), *true*).
    //    i. Return _array_.
    //
    // NOTE:
    // This method is an intentionally generic factory method; it does not require that its *this* value be the Array constructor. Therefore it can be transferred to or inherited by any other constructors that may be called with a single numeric argument.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.fromAsync"))
}
