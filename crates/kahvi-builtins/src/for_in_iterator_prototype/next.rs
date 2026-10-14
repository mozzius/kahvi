//! %ForInIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%foriniteratorprototype%.next

use kahvi_core::prelude::*;

/// ForInIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _iterator_ be the *this* value.
    // 2. Assert: _iterator_ is an Object.
    // 3. Assert: _iterator_ has all of the internal slots of a For-In Iterator instance ([sec-properties-of-for-in-iterator-instances]).
    // 4. Let _obj_ be _iterator_.[[Object]].
    // 5. Repeat,
    //    a. If _iterator_.[[ObjectWasVisited]] is *false*, then
    //       i. Let _keys_ be ? _obj_.[[OwnPropertyKeys]]().
    //       ii. For each element _key_ of _keys_, do
    //          1. If _key_ is a String, then
    //             a. Append _key_ to _iterator_.[[RemainingKeys]].
    //       iii. Set _iterator_.[[ObjectWasVisited]] to *true*.
    //    b. Repeat, while _iterator_.[[RemainingKeys]] is not empty,
    //       i. Let _key_ be the first element of _iterator_.[[RemainingKeys]].
    //       ii. Remove the first element from _iterator_.[[RemainingKeys]].
    //       iii. If _iterator_.[[VisitedKeys]] does not contain _key_, then
    //          1. Let _propertyDesc_ be ? _obj_.[[GetOwnProperty]](_key_).
    //          2. If _propertyDesc_ is not *undefined*, then
    //             a. Append _key_ to _iterator_.[[VisitedKeys]].
    //             b. If _propertyDesc_.[[Enumerable]] is *true*, return CreateIteratorResultObject(_key_, *false*).
    //    c. Set _obj_ to ? _obj_.[[GetPrototypeOf]]().
    //    d. Set _iterator_.[[Object]] to _obj_.
    //    e. Set _iterator_.[[ObjectWasVisited]] to *false*.
    //    f. If _obj_ is *null*, return CreateIteratorResultObject(*undefined*, *true*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("ForInIteratorPrototype.next"))
}
