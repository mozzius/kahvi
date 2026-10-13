//! Error.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-error.prototype.tostring

use kahvi_core::prelude::*;

/// Error.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be the this value.
    // 2. If O is not an Object, throw a TypeError exception.
    let Value::Object(o) = this else {
        return Err(agent.throw_type_error("Error.prototype.toString called on non-object"));
    };
    // 3. Let name be ? Get(O, "name").
    let name_key = PropertyKey::String(agent.names.name);
    let name = ops::get(agent, o, name_key)?;
    // 4. If name is undefined, set name to "Error"; otherwise set name to ? ToString(name).
    let name = match name {
        Value::Undefined => agent.string("Error"),
        v => ops::to_string(agent, v)?,
    };
    // 5. Let msg be ? Get(O, "message").
    let message_key = PropertyKey::String(agent.names.message);
    let msg = ops::get(agent, o, message_key)?;
    // 6. If msg is undefined, set msg to the empty String; otherwise set msg to ? ToString(msg).
    let msg = match msg {
        Value::Undefined => agent.names.empty,
        v => ops::to_string(agent, v)?,
    };
    // 7. If name is the empty String, return msg.
    if agent.str_len(name) == 0 {
        return Ok(Value::String(msg));
    }
    // 8. If msg is the empty String, return name.
    if agent.str_len(msg) == 0 {
        return Ok(Value::String(name));
    }
    // 9. Return the string-concatenation of name, the code unit 0x003A (COLON), the code unit 0x0020 (SPACE), and msg.
    let sep = agent.string(": ");
    let head = agent.concat(name, sep);
    Ok(Value::String(agent.concat(head, msg)))
}
