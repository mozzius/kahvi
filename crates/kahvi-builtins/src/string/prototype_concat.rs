//! String.prototype.concat ( ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.concat

use kahvi_core::prelude::*;

/// String.prototype.concat
pub(crate) fn prototype_concat(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // When this method is called it returns the String value consisting of the code units of the *this* value (converted to a String) followed by the code units of each of the arguments converted to a String. The result is a String value, not a String object.
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _result_ be _string_.
    // 5. For each element _next_ of _args_, do
    //    a. Let _nextString_ be ? ToString(_next_).
    //    b. Set _result_ to the string-concatenation of _result_ and _nextString_.
    // 6. Return _result_.
    //
    // The *"length"* property of this method is *1*_𝔽.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.concat"))
}
