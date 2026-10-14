//! Object.isSealed ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.issealed

use kahvi_core::prelude::*;

/// Object.isSealed
pub(crate) fn is_sealed(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _obj_ is not an Object, return *true*.
    let Value::Object(o) = args.get_or_undefined(0) else {
        return Ok(Value::Boolean(true));
    };
    // 2. Return ? TestIntegrityLevel(_obj_, ~sealed~).
    let result = ops::test_integrity_level(agent, o, ops::IntegrityLevel::Sealed)?;
    Ok(Value::Boolean(result))
}
