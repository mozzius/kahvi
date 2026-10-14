//! Object.isFrozen ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.isfrozen

use kahvi_core::prelude::*;

/// Object.isFrozen
pub(crate) fn is_frozen(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _obj_ is not an Object, return *true*.
    let Value::Object(o) = args.get_or_undefined(0) else {
        return Ok(Value::Boolean(true));
    };
    // 2. Return ? TestIntegrityLevel(_obj_, ~frozen~).
    let result = ops::test_integrity_level(agent, o, ops::IntegrityLevel::Frozen)?;
    Ok(Value::Boolean(result))
}
