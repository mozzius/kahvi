//! Iterator.prototype.chunks ( _chunkSize_ )
//!
//! Spec: https://tc39.es/proposal-iterator-chunking/#sec-iterator.prototype.chunks

use kahvi_core::prelude::*;

/// Iterator.prototype.chunks
pub(crate) fn prototype_chunks(
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
    // 4. If _chunkSize_ is not an integral Number, then
    //    a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 5. If _chunkSize_ is not in the inclusive interval from *1*_𝔽 to 𝔽(2^32 - 1), then
    //    a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 6. Set _iterated_ to ? GetIteratorDirect(_O_).
    // 7. Let _closure_ be a new Abstract Closure with no parameters that captures _iterated_ and _chunkSize_ and performs the following steps when called:
    //    a. Let _buffer_ be a new empty List.
    //    b. Repeat,
    //       i. Let _value_ be ? IteratorStepValue(_iterated_).
    //       ii. If _value_ is ~done~, then
    //          1. If _buffer_ is not empty, then
    //             a. Perform Completion(Yield(CreateArrayFromList(_buffer_))).
    //          2. Return ReturnCompletion(*undefined*).
    //       iii. Append _value_ to _buffer_.
    //       iv. If the number of elements in _buffer_ is ℝ(_chunkSize_), then
    //          1. Let _completion_ be Completion(Yield(CreateArrayFromList(_buffer_))).
    //          2. IfAbruptCloseIterator(_completion_, _iterated_).
    //          3. Set _buffer_ to a new empty List.
    // 8. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 9. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
    // 10. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.chunks"))
}
