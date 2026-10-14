//! BigInt.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-bigint.prototype.valueof

use kahvi_core::prelude::*;

use crate::support::this_value::this_big_int_value;

/// BigInt.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (args, new_target);
    // 1. Return ? ThisBigIntValue(*this* value).
    Ok(Value::BigInt(this_big_int_value(agent, this)?))
    //
    // [subclause sec-thisbigintvalue: ThisBigIntValue (
    // _arg_: an ECMAScript language value,
    // ): either a normal completion containing a BigInt or a throw completion]
}
