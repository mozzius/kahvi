//! Boolean ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-boolean-constructor-boolean-value

use kahvi_core::prelude::*;

/// Boolean
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _bool_ be ToBoolean(_value_).
    // 2. If NewTarget is *undefined*, return _bool_.
    // 3. Let _obj_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Boolean.prototype%"*, « [[BooleanData]] »).
    // 4. Set _obj_.[[BooleanData]] to _bool_.
    // 5. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Boolean"))
}
