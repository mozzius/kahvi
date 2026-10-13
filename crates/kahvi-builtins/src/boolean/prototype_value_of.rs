//! Boolean.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-boolean.prototype.valueof

use kahvi_core::prelude::*;

/// Boolean.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Return ? ThisBooleanValue(*this* value).
    //
    // [subclause sec-thisbooleanvalue: ThisBooleanValue (
    // _arg_: an ECMAScript language value,
    // ): either a normal completion containing a Boolean or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Boolean.prototype.valueOf"))
}
