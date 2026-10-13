//! Iterator.concat ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.concat

use kahvi_core::prelude::*;

/// Iterator.concat
pub(crate) fn concat(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _iterables_ be a new empty List.
    // 2. For each element _item_ of _items_, do
    //    a. If _item_ is not an Object, throw a *TypeError* exception.
    //    b. Let _method_ be ? GetMethod(_item_, %Symbol.iterator%).
    //    c. If _method_ is *undefined*, throw a *TypeError* exception.
    //    d. Append the Record { [[OpenMethod]]: _method_, [[Iterable]]: _item_ } to _iterables_.
    // 3. Let _closure_ be a new Abstract Closure with no parameters that captures _iterables_ and performs the following steps when called:
    //    a. For each Record _iterable_ of _iterables_, do
    //       i. Let _iterator_ be ? Call(_iterable_.[[OpenMethod]], _iterable_.[[Iterable]]).
    //       ii. If _iterator_ is not an Object, throw a *TypeError* exception.
    //       iii. Let _iteratorRecord_ be ? GetIteratorDirect(_iterator_).
    //       iv. Let _innerAlive_ be *true*.
    //       v. Repeat, while _innerAlive_ is *true*,
    //          1. Let _innerValue_ be ? IteratorStepValue(_iteratorRecord_).
    //          2. If _innerValue_ is ~done~, then
    //             a. Set _innerAlive_ to *false*.
    //          3. Else,
    //             a. Let _completion_ be Completion(Yield(_innerValue_)).
    //             b. If _completion_ is an abrupt completion, then
    //                i. Return ? IteratorClose(_iteratorRecord_, _completion_).
    //    b. Return ReturnCompletion(*undefined*).
    // 4. Let _gen_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 5. Set _gen_.[[UnderlyingIterators]] to a new empty List.
    // 6. Return _gen_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.concat"))
}
