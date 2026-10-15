//! Iterator.prototype.includes ( _searchElement_ [ , _skippedElements_ ] )
//!
//! Spec: https://tc39.es/proposal-iterator-includes/#sec-iterator.prototype.includes

use kahvi_core::prelude::*;

/// Iterator.prototype.includes
pub(crate) fn prototype_includes(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    // 3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
    // 4. If _skippedElements_ is *undefined*, then
    //    a. Let _toSkip_ be *+0*_𝔽.
    // 5. Else,
    //    a. If _skippedElements_ is not one of *+∞_𝔽*, *-∞_𝔽*, or an integral Number, then
    //       i. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //       ii. Return ? IteratorClose(_iterated_, _error_).
    //    b. Let _toSkip_ be _skippedElements_.
    // 6. If _toSkip_ < *-0*_𝔽, then
    //    a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 7. If _toSkip_ is finite and _toSkip_ > 𝔽(2^53 - 1), then
    //    a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 8. Let _skipped_ be *+0*_𝔽.
    // 9. Set _iterated_ to ? GetIteratorDirect(_obj_).
    // 10. Repeat,
    //    a. Let _value_ be ? IteratorStepValue(_iterated_).
    //    b. If _value_ is ~done~, return *false*.
    //    c. If _skipped_ < _toSkip_, then
    //       i. Set _skipped_ to _skipped_ + *1*_𝔽.
    //    d. Else if SameValueZero(_value_, _searchElement_) is *true*, then
    //       i. Return ? IteratorClose(_iterated_, NormalCompletion(*true*)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.includes"))
}
