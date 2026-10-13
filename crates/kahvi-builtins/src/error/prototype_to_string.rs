//! Error.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-error.prototype.tostring

use kahvi_core::prelude::*;

/// Error.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    // 3. Let _name_ be ? Get(_obj_, *"name"*).
    // 4. If _name_ is *undefined*, set _name_ to *"Error"*; else set _name_ to ? ToString(_name_).
    // 5. Let _message_ be ? Get(_obj_, *"message"*).
    // 6. If _message_ is *undefined*, set _message_ to the empty String; else set _message_ to ? ToString(_message_).
    // 7. If _name_ is the empty String, return _message_.
    // 8. If _message_ is the empty String, return _name_.
    // 9. Return the string-concatenation of _name_, the code unit 0x003A (COLON), the code unit 0x0020 (SPACE), and _message_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Error.prototype.toString"))
}
