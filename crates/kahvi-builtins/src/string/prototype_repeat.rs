//! String.prototype.repeat ( _count_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.repeat

use kahvi_core::prelude::*;

/// String.prototype.repeat
pub(crate) fn prototype_repeat(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _n_ be ? ToIntegerOrInfinity(_count_).
    // 5. If _n_ < 0 or _n_ = +∞, throw a *RangeError* exception.
    // 6. If _n_ = 0, return the empty String.
    // 7. Return the String value that is made from _n_ copies of _string_ appended together.
    //
    // NOTE:
    // This method creates the String value consisting of the code units of the *this* value (converted to String) repeated _count_ times.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.repeat"))
}
