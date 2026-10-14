//! Object.preventExtensions ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.preventextensions

use kahvi_core::prelude::*;

/// Object.preventExtensions
pub(crate) fn prevent_extensions(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _obj_ is not an Object, return _obj_.
    let obj = args.get_or_undefined(0);
    let Value::Object(o) = obj else {
        return Ok(obj);
    };
    // 2. Let _status_ be ? _obj_.[[PreventExtensions]]().
    let status = o.internal_prevent_extensions(agent)?;
    // 3. If _status_ is *false*, throw a *TypeError* exception.
    if !status {
        return Err(
            agent.throw_type_error("Object.preventExtensions called on non-extensible object")
        );
    }
    // 4. Return _obj_.
    Ok(Value::Object(o))
}
