//! Object.seal ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.seal

use kahvi_core::prelude::*;

/// Object.seal
pub(crate) fn seal(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    // 1. If _obj_ is not an Object, return _obj_.
    let obj = args.get_or_undefined(0).clone();
    let Value::Object(o) = obj else {
        return Ok(obj);
    };
    // 2. Let _status_ be ? SetIntegrityLevel(_obj_, ~sealed~).
    let status = ops::set_integrity_level(agent, o, ops::IntegrityLevel::Sealed)?;
    // 3. If _status_ is *false*, throw a *TypeError* exception.
    if !status {
        return Err(agent.throw_type_error("Object.seal could not seal the object"));
    }
    // 4. Return _obj_.
    Ok(Value::Object(o))
}
