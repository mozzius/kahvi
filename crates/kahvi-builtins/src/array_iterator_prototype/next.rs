//! %ArrayIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%arrayiteratorprototype%.next

use kahvi_core::prelude::*;

/// ArrayIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _iteratorObj_ be the *this* value.
    // 2. If _iteratorObj_ is not an Object, throw a *TypeError* exception.
    // 3. If _iteratorObj_ does not have all of the internal slots of an Array Iterator Instance ([sec-properties-of-array-iterator-instances]), throw a *TypeError* exception.
    // 4. Let _array_ be _iteratorObj_.[[IteratedArrayLike]].
    // 5. If _array_ is *undefined*, return CreateIteratorResultObject(*undefined*, *true*).
    // 6. Let _index_ be _iteratorObj_.[[ArrayLikeNextIndex]].
    // 7. Let _kind_ be _iteratorObj_.[[ArrayLikeIterationKind]].
    // 8. If _array_ has a [[TypedArrayName]] internal slot, then
    //    a. Let _taRecord_ be ? ValidateTypedArrayBounds(_array_, ~seq-cst~).
    //    b. Let _length_ be TypedArrayLength(_taRecord_).
    // 9. Else,
    //    a. Let _length_ be ? LengthOfArrayLike(_array_).
    // 10. If _index_ ≥ _length_, then
    //    a. Set _iteratorObj_.[[IteratedArrayLike]] to *undefined*.
    //    b. Return CreateIteratorResultObject(*undefined*, *true*).
    // 11. Set _iteratorObj_.[[ArrayLikeNextIndex]] to _index_ + 1.
    // 12. Let _indexNumber_ be 𝔽(_index_).
    // 13. If _kind_ is ~key~, then
    //    a. Let _result_ be _indexNumber_.
    // 14. Else,
    //    a. Let _elementKey_ be ! ToString(_indexNumber_).
    //    b. Let _elementValue_ be ? Get(_array_, _elementKey_).
    //    c. If _kind_ is ~value~, then
    //       i. Let _result_ be _elementValue_.
    //    d. Else,
    //       i. Assert: _kind_ is ~key+value~.
    //       ii. Let _result_ be CreateArrayFromList(« _indexNumber_, _elementValue_ »).
    // 15. Return CreateIteratorResultObject(_result_, *false*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("ArrayIteratorPrototype.next"))
}
