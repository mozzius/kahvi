//! Object.freeze ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.freeze

use kahvi_core::prelude::*;

/// Object.freeze
pub(crate) fn freeze(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _obj_ is not an Object, return _obj_.
    let obj = args.get_or_undefined(0);
    let Value::Object(o) = obj else {
        return Ok(obj.clone());
    };
    // 2. Let _status_ be ? SetIntegrityLevel(_obj_, ~frozen~).
    let status = ops::set_integrity_level(agent, o, ops::IntegrityLevel::Frozen)?;
    // 3. If _status_ is *false*, throw a *TypeError* exception.
    if !status {
        return Err(agent.throw_type_error("Object.freeze failed to freeze the object"));
    }
    // 4. Return _obj_.
    Ok(Value::Object(o))
}
