//! get Error.prototype.stack
//!
//! Spec: https://tc39.es/proposal-error-stack-accessor/#sec-get-error.prototype.stack

use kahvi_core::prelude::*;

/// get Error.prototype.stack
pub(crate) fn prototype_get_stack(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let E be the this value.
    // 2. If E is not an Object, throw a TypeError exception.
    let Value::Object(e) = this else {
        return Err(agent.throw_type_error("Error.prototype.stack getter called on a non-object"));
    };
    // 3. If E does not have an [[ErrorData]] internal slot, return undefined.
    let Some(frames) = agent.error_stack(e).map(|f| agent.format_stack(f)) else {
        return Ok(Value::Undefined);
    };
    // 4. Return an implementation-defined string that represents the stack trace of E.
    // Here: "name: message" and then the frames captured when E was created.
    // name and message come from data properties only, so reading the stack
    // runs no code.
    let name = string_data_property(agent, e, "name").unwrap_or_else(|| "Error".to_string());
    let message = string_data_property(agent, e, "message").unwrap_or_default();
    let mut text = match (name.is_empty(), message.is_empty()) {
        (_, true) => name,
        (true, false) => message,
        (false, false) => format!("{name}: {message}"),
    };
    if !frames.is_empty() {
        text.push('\n');
        text.push_str(&frames);
    }
    Ok(Value::String(agent.string(&text)))
}

/// The string value of the first data property `name` found on `o` or its
/// prototype chain, stopping at anything else.
fn string_data_property(agent: &mut Agent, o: Object, name: &str) -> Option<String> {
    let key = agent.key(name);
    let mut cur = Some(o);
    while let Some(c) = cur {
        match agent.obj(c).properties.get(&key) {
            Some(Property::Data {
                value: Value::String(s),
                ..
            }) => return Some(agent.str_lossy(s)),
            Some(_) => return None,
            None => cur = agent.obj(c).prototype,
        }
    }
    None
}
