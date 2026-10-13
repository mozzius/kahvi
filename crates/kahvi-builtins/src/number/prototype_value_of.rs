//! Number.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.prototype.valueof

use kahvi_core::prelude::*;

/// Number.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? ThisNumberValue(*this* value).
    //
    // [subclause sec-thisnumbervalue: ThisNumberValue (
    // _arg_: an ECMAScript language value,
    // ): either a normal completion containing a Number or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.prototype.valueOf"))
}
