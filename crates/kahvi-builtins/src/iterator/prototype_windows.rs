//! Iterator.prototype.windows ( _windowSize_ [ , _undersized_ ] )
//!
//! Spec: https://tc39.es/proposal-iterator-chunking/#sec-iterator.prototype.windows

use kahvi_core::prelude::*;

/// Iterator.prototype.windows
pub(crate) fn prototype_windows(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _O_ be the *this* value.
    // 2. If _O_ is not an Object, throw a *TypeError* exception.
    // 3. Let _iterated_ be the Iterator Record { [[Iterator]]: _O_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
    // 4. If _windowSize_ is not an integral Number, then
    //    a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 5. If _windowSize_ is not in the inclusive interval from *1*_𝔽 to 𝔽(2^32 - 1), then
    //    a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 6. If _undersized_ is *undefined*, set _undersized_ to *"only-full"*.
    // 7. If _undersized_ is neither *"only-full"* nor *"allow-partial"*, then
    //    a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 8. Set _iterated_ to ? GetIteratorDirect(_O_).
    // 9. Let _closure_ be a new Abstract Closure with no parameters that captures _iterated_, _windowSize_, and _undersized_ and performs the following steps when called:
    //    a. Let _buffer_ be a new empty List.
    //    b. Repeat,
    //       i. Let _value_ be ? IteratorStepValue(_iterated_).
    //       ii. If _value_ is ~done~, then
    //          1. If _undersized_ is *"allow-partial"*, _buffer_ is not empty, and the number of elements in _buffer_ < ℝ(_windowSize_), then
    //             a. Perform Completion(Yield(CreateArrayFromList(_buffer_))).
    //          2. Return ReturnCompletion(*undefined*).
    //       iii. If the number of elements in _buffer_ is ℝ(_windowSize_), then
    //          1. Remove the first element from _buffer_.
    //       iv. Append _value_ to _buffer_.
    //       v. If the number of elements in _buffer_ is ℝ(_windowSize_), then
    //          1. Let _completion_ be Completion(Yield(CreateArrayFromList(_buffer_))).
    //          2. IfAbruptCloseIterator(_completion_, _iterated_).
    // 10. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 11. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
    // 12. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.windows"))
}
