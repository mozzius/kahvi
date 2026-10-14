//! BigInt.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-bigint.prototype.valueof

use kahvi_core::prelude::*;

/// BigInt.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? ThisBigIntValue(*this* value).
    //
    // [subclause sec-thisbigintvalue: ThisBigIntValue (
    // _arg_: an ECMAScript language value,
    // ): either a normal completion containing a BigInt or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("BigInt.prototype.valueOf"))
}
