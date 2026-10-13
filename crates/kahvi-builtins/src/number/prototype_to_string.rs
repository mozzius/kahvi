//! Number.prototype.toString ( [ _radix_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.prototype.tostring

use kahvi_core::prelude::*;

/// Number.prototype.toString
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
    // 1. Let _x_ be ? ThisNumberValue(*this* value).
    // 2. If _radix_ is *undefined*, let _radixMV_ be 10.
    // 3. Else, let _radixMV_ be ? SnapToInteger(_radix_, ~truncate~, 2, 36).
    // 4. Return Number::toString(_x_, _radixMV_).
    //
    // This method is not generic; it throws a *TypeError* exception if its *this* value is not a Number or a Number object. Therefore, it cannot be transferred to other kinds of objects for use as a method.
    // The *"length"* property of this method is *1*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.prototype.toString"))
}
