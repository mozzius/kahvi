//! Promise.resolve ( _resolution_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.resolve

use kahvi_core::prelude::*;

/// Promise.resolve
pub(crate) fn resolve(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns either a new promise resolved with the passed argument, or the argument itself if the argument is a promise produced by this constructor.
    //
    // 1. Let _ctor_ be the *this* value.
    // 2. If _ctor_ is not an Object, throw a *TypeError* exception.
    // 3. Return ? PromiseResolve(_ctor_, _resolution_).
    //
    // NOTE:
    // This function expects its *this* value to be a constructor function that supports the parameter conventions of the Promise constructor.
    // [subclause sec-promise-resolve: PromiseResolve (
    // _ctor_: an Object,
    // _resolution_: an ECMAScript language value,
    // ): either a normal completion containing an Object or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise.resolve"))
}
