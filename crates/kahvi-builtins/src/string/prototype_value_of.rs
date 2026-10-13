//! String.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.valueof

use kahvi_core::prelude::*;

/// String.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Return ? ThisStringValue(*this* value).
    //
    // [subclause sec-thisstringvalue: ThisStringValue (
    // _arg_: an ECMAScript language value,
    // ): either a normal completion containing a String or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.valueOf"))
}
