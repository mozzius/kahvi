//! Set.prototype.forEach ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.foreach

use kahvi_core::prelude::*;

/// Set.prototype.forEach
pub(crate) fn prototype_for_each(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 4. Let _entries_ be _set_.[[SetData]].
    // 5. Let _entriesCount_ be the number of elements in _entries_.
    // 6. Let _index_ be 0.
    // 7. Repeat, while _index_ < _entriesCount_,
    //    a. Let _entry_ be _entries_[_index_].
    //    b. Set _index_ to _index_ + 1.
    //    c. If _entry_ is not ~empty~, then
    //       i. Perform ? Call(_callback_, _thisArg_, « _entry_, _entry_, _set_ »).
    //       ii. NOTE: The number of elements in _entries_ may have increased during execution of _callback_.
    //       iii. Set _entriesCount_ to the number of elements in _entries_.
    // 8. Return *undefined*.
    //
    // NOTE:
    // _callback_ should be a function that accepts three arguments. `forEach` calls _callback_ once for each value present in the Set object, in value insertion order. _callback_ is called only for values of the Set which actually exist; it is not called for keys that have been deleted from the set.
    // If a _thisArg_ parameter is provided, it will be used as the *this* value for each invocation of _callback_. If it is not provided, *undefined* is used instead.
    // _callback_ is called with three arguments: the first two arguments are a value contained in the Set. The same value is passed for both arguments. The Set object being traversed is passed as the third argument.
    // The _callback_ is called with three arguments to be consistent with the call back functions used by `forEach` methods for Map and Array. For Sets, each item value is considered to be both the key and the value.
    // `forEach` does not directly mutate the object on which it is called but the object may be mutated by the calls to _callback_.
    // Each value is normally visited only once. However, a value will be revisited if it is deleted after it has been visited and then re-added before the `forEach` call completes. Values that are deleted after the call to `forEach` begins and before being visited are not visited unless the value is added again before the `forEach` call completes. New values added after the call to `forEach` begins are visited.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.forEach"))
}
