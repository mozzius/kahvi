//! Map ( [ _iterable_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-map-iterable

use kahvi_core::prelude::*;

/// Map
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _map_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Map.prototype%"*, « [[MapData]] »).
    // 3. Set _map_.[[MapData]] to a new empty List.
    // 4. If _iterable_ is either *undefined* or *null*, return _map_.
    // 5. Let _adder_ be ? Get(_map_, *"set"*).
    // 6. If IsCallable(_adder_) is *false*, throw a *TypeError* exception.
    // 7. Return ? AddEntriesFromIterable(_map_, _iterable_, _adder_).
    //
    // NOTE:
    // If the parameter _iterable_ is present, it is expected to be an object that implements a %Symbol.iterator% method that returns an iterator object that produces a two element array-like object whose first element is a value that will be used as a Map key and whose second element is the value to associate with that key.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map"))
}
