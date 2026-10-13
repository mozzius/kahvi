//! Object.setPrototypeOf ( _obj_, _proto_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.setprototypeof

use kahvi_core::prelude::*;

/// Object.setPrototypeOf
pub(crate) fn set_prototype_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Perform ? RequireObjectCoercible(_obj_).
    // 2. If _proto_ is not an Object and _proto_ is not *null*, throw a *TypeError* exception.
    // 3. If _obj_ is not an Object, return _obj_.
    // 4. Let _status_ be ? _obj_.[[SetPrototypeOf]](_proto_).
    // 5. If _status_ is *false*, throw a *TypeError* exception.
    // 6. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.setPrototypeOf"))
}
