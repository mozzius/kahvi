//! WeakSet ( [ _iterable_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-weakset-iterable

use kahvi_core::prelude::*;

/// WeakSet
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _set_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%WeakSet.prototype%"*, « [[WeakSetData]] »).
    // 3. Set _set_.[[WeakSetData]] to a new empty List.
    // 4. If _iterable_ is either *undefined* or *null*, return _set_.
    // 5. Let _adder_ be ? Get(_set_, *"add"*).
    // 6. If IsCallable(_adder_) is *false*, throw a *TypeError* exception.
    // 7. Let _iteratorRecord_ be ? GetIterator(_iterable_, ~sync~).
    // 8. Repeat,
    //    a. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
    //    b. If _next_ is ~done~, return _set_.
    //    c. Let _status_ be Completion(Call(_adder_, _set_, « _next_ »)).
    //    d. IfAbruptCloseIterator(_status_, _iteratorRecord_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WeakSet"))
}
