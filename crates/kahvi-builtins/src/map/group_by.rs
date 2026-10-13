//! Map.groupBy ( _items_, _callback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.groupby

use kahvi_core::prelude::*;

/// Map.groupBy
pub(crate) fn group_by(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that accepts two arguments. `groupBy` calls _callback_ once for each element in _items_, in ascending order, and constructs a new Map. Each value returned by _callback_ is used as a key in the Map. For each such key, the result Map has an entry whose key is that key and whose value is an array containing all the elements for which _callback_ returned that key.
    // _callback_ is called with two arguments: the value of the element and the index of the element.
    // The return value of `groupBy` is a Map.
    // This function performs the following steps when called:
    //
    // 1. Let _groups_ be ? GroupBy(_items_, _callback_, ~collection~).
    // 2. Let _map_ be ! Construct(%Map%).
    // 3. For each Record { [[Key]], [[Elements]] } _group_ of _groups_, do
    //    a. Let _elements_ be CreateArrayFromList(_group_.[[Elements]]).
    //    b. Let _entry_ be the Record { [[Key]]: _group_.[[Key]], [[Value]]: _elements_ }.
    //    c. Append _entry_ to _map_.[[MapData]].
    // 4. Return _map_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.groupBy"))
}
