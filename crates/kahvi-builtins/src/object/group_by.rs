//! Object.groupBy ( _items_, _callback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.groupby

use kahvi_core::prelude::*;

/// Object.groupBy
pub(crate) fn group_by(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that accepts two arguments. `groupBy` calls _callback_ once for each element in _items_, in ascending order, and constructs a new object. Each value returned by _callback_ is coerced to a property key. For each such property key, the result object has a property whose key is that property key and whose value is an array containing all the elements for which the _callback_ return value coerced to that key.
    // _callback_ is called with two arguments: the value of the element and the index of the element.
    // The return value of `groupBy` is an object that does not inherit from %Object.prototype%.
    // This function performs the following steps when called:
    //
    // 1. Let _groups_ be ? GroupBy(_items_, _callback_, ~property~).
    // 2. Let _obj_ be OrdinaryObjectCreate(*null*).
    // 3. For each Record { [[Key]], [[Elements]] } _group_ of _groups_, do
    //    a. Let _elements_ be CreateArrayFromList(_group_.[[Elements]]).
    //    b. Perform ! CreateDataPropertyOrThrow(_obj_, _group_.[[Key]], _elements_).
    // 4. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.groupBy"))
}
