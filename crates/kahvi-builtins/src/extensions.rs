//! Implementation extensions (§17.1) that are not in any specification:
//! the V8 stack trace API that Hermes also provides and React Native code
//! relies on, `Error.captureStackTrace` and `Error.stackTraceLimit`.
//! Stacks are written in V8's format (`Agent::format_stack`).

use kahvi_core::function::create_builtin_function;
use kahvi_core::prelude::*;

/// Error.stackTraceLimit's initial value, as in V8 and Hermes.
const STACK_TRACE_LIMIT: f64 = 10.0;

pub(crate) fn install(agent: &mut Agent, realm: RealmId) {
    let Some(&error) = agent.realm(realm).intrinsics.get("%Error%") else {
        return;
    };
    // Error.stackTraceLimit: a plain data property, as if assigned.
    let key = PropertyKey::String(agent.names.stack_trace_limit);
    agent.obj_mut(error).properties.insert(
        key,
        Property::Data {
            value: Value::Number(STACK_TRACE_LIMIT),
            writable: true,
            enumerable: true,
            configurable: true,
        },
    );
    // Error.captureStackTrace, a method property.
    let key = agent.key("captureStackTrace");
    let f = create_builtin_function(
        agent,
        capture_stack_trace,
        2,
        key,
        Vec::new(),
        Some(realm),
        None,
        None,
        false,
    );
    agent.obj_mut(error).properties.insert(
        key,
        Property::Data {
            value: Value::Object(f),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
}

/// Error.captureStackTrace ( targetObject [ , constructorOpt ] ), as V8
/// defines it: gives targetObject an own "stack" data property holding
/// `String(Error.prototype.toString.call(targetObject))`-style header and
/// the current stack, leaving out the frames above and including the
/// innermost call of constructorOpt when it is a function.
fn capture_stack_trace(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let Some(&Value::Object(target)) = args.first() else {
        return Err(agent.throw_type_error("Error.captureStackTrace: invalid argument"));
    };
    // Without constructorOpt, captureStackTrace's own frame is the one left out.
    let skip_through = match args.get(1) {
        Some(&Value::Object(f)) if f.is_callable(agent) => Some(f),
        _ => agent.active_function(),
    };
    let frames = agent.capture_stack(skip_through);
    // The header, as Error.prototype.toString writes it.
    let name_key = PropertyKey::String(agent.names.name);
    let name = match ops::get(agent, target, name_key)? {
        Value::Undefined => "Error".to_string(),
        v => {
            let s = ops::to_string(agent, v)?;
            agent.str_lossy(s)
        }
    };
    let message_key = PropertyKey::String(agent.names.message);
    let message = match ops::get(agent, target, message_key)? {
        Value::Undefined => String::new(),
        v => {
            let s = ops::to_string(agent, v)?;
            agent.str_lossy(s)
        }
    };
    let mut text = match (name.is_empty(), message.is_empty()) {
        (_, true) => name,
        (true, false) => message,
        (false, false) => format!("{name}: {message}"),
    };
    let value = match frames {
        Some(frames) => {
            if !frames.is_empty() {
                text.push('\n');
                text.push_str(&agent.format_stack(&frames));
            }
            Value::String(agent.string(&text))
        }
        // Error.stackTraceLimit is not a number: no stack, as in V8.
        None => Value::Undefined,
    };
    let key = agent.key("stack");
    ops::define_property_or_throw(
        agent,
        target,
        key,
        PropertyDescriptor::data(value, true, false, true),
    )?;
    Ok(Value::Undefined)
}
