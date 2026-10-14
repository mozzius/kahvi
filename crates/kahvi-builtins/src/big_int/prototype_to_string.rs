//! BigInt.prototype.toString ( [ _radix_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-bigint.prototype.tostring

use kahvi_core::prelude::*;

use crate::support::this_value::this_big_int_value;

/// BigInt.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The optional _radix_ should be an integral Number value in the inclusive interval from *2*_𝔽 to *36*_𝔽. If _radix_ is *undefined* then *10*_𝔽 is used as the value of _radix_.
    // This method performs the following steps when called:
    //
    let _ = new_target;
    // 1. Let _x_ be ? ThisBigIntValue(*this* value).
    let x = this_big_int_value(agent, this)?;
    let radix = args.get_or_undefined(0);
    let radix_mv = if radix.is_undefined() {
        // 2. If _radix_ is *undefined*, let _radixMV_ be 10.
        10
    } else {
        // 3. Else, let _radixMV_ be ? SnapToInteger(_radix_, ~truncate~, 2, 36).
        ops::snap_to_integer(agent, radix, true, Some(2.0), Some(36.0))? as u32
    };
    // 4. Return BigInt::toString(_x_, _radixMV_).
    Ok(Value::String(ops::bigint::to_js_string(agent, x, radix_mv)))
    //
    // This method is not generic; it throws a *TypeError* exception if its *this* value is not a BigInt or a BigInt object. Therefore, it cannot be transferred to other kinds of objects for use as a method.
}
