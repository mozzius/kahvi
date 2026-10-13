//! Object.assign ( _target_, ..._sources_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.assign

use kahvi_core::prelude::*;

/// Object.assign
pub(crate) fn assign(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function copies the values of all of the enumerable own properties from one or more source objects to a _target_ object.
    // It performs the following steps when called:
    //
    // 1. Let _targetObj_ be ? ToObject(_target_).
    // 2. If only one argument was passed, return _targetObj_.
    // 3. For each element _nextSource_ of _sources_, do
    //    a. If _nextSource_ is neither *undefined* nor *null*, then
    //       i. Let _from_ be ! ToObject(_nextSource_).
    //       ii. Let _keys_ be ? _from_.[[OwnPropertyKeys]]().
    //       iii. For each element _nextKey_ of _keys_, do
    //          1. Let _propertyDesc_ be ? _from_.[[GetOwnProperty]](_nextKey_).
    //          2. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Enumerable]] is *true*, then
    //             a. Let _propertyValue_ be ? Get(_from_, _nextKey_).
    //             b. Perform ? Set(_targetObj_, _nextKey_, _propertyValue_, *true*).
    // 4. Return _targetObj_.
    //
    // The *"length"* property of this function is *2*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.assign"))
}
