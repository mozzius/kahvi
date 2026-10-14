//! Number.prototype.toString ( [ _radix_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.prototype.tostring

use crate::support::this_value::this_number_value;
use kahvi_core::prelude::*;

/// Number.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let x be ? ThisNumberValue(this value).
    let x = this_number_value(agent, this)?;
    // 2. If radix is undefined, let radixMV be 10.
    // 3. Else, let radixMV be ? ToIntegerOrInfinity(radix).
    let radix = match args.get_or_undefined(0) {
        Value::Undefined => 10.0,
        r => ops::to_integer_or_infinity(agent, r)?,
    };
    // 4. If radixMV is not in the inclusive interval from 2 to 36, throw a RangeError exception.
    if !(2.0..=36.0).contains(&radix) {
        return Err(agent.throw_range_error("toString() radix must be between 2 and 36"));
    }
    // 5. Return Number::toString(x, radixMV).
    if radix == 10.0 {
        return Ok(Value::String(ops::number_to_js_string(agent, x)));
    }
    Ok(Value::String(ops::number_to_js_string_radix(
        agent,
        x,
        radix as u32,
    )))
}
