//! Function.prototype.bind ( _thisArg_, ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-function.prototype.bind

use kahvi_core::function::{bound_function_create, set_function_length, set_function_name};
use kahvi_core::prelude::*;

/// Function.prototype.bind
pub(crate) fn prototype_bind(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let Target be the this value.
    // 2. If IsCallable(Target) is false, throw a TypeError exception.
    let target = match this {
        Value::Object(o) if o.is_callable(agent) => o,
        _ => return Err(agent.throw_type_error("Bind must be called on a function")),
    };
    let this_arg = args.get_or_undefined(0);
    let bound_args: Vec<Value> = args.get(1..).unwrap_or(&[]).to_vec();
    let arg_count = bound_args.len() as f64;
    // 3. Let F be ? BoundFunctionCreate(Target, thisArg, args).
    let f = bound_function_create(agent, target, this_arg, bound_args)?;
    // 4. Let L be 0.
    let mut l = 0.0;
    // 5. Let targetHasLength be ? HasOwnProperty(Target, "length").
    let length_key = PropertyKey::String(agent.names.length);
    // 6. If targetHasLength is true, then
    if ops::has_own_property(agent, target, length_key)? {
        // a. Let targetLen be ? Get(Target, "length").
        // b. If targetLen is a Number, then
        if let Value::Number(target_len) = ops::get(agent, target, length_key)? {
            if target_len == f64::INFINITY {
                // i. If targetLen is +∞𝔽, set L to +∞.
                l = f64::INFINITY;
            } else if target_len == f64::NEG_INFINITY {
                // ii. Else if targetLen is -∞𝔽, set L to 0.
                l = 0.0;
            } else {
                // iii.1 Let targetLenAsInt be ! ToIntegerOrInfinity(targetLen).
                let as_int = ops::integer_or_infinity(target_len);
                // iii.3 Set L to max(targetLenAsInt - argCount, 0).
                l = (as_int - arg_count).max(0.0);
            }
        }
    }
    // 7. Perform SetFunctionLength(F, L).
    set_function_length(agent, f, l);
    // 8. Let targetName be ? Get(Target, "name").
    let name_key = PropertyKey::String(agent.names.name);
    let target_name = ops::get(agent, target, name_key)?;
    // 9. If targetName is not a String, set targetName to the empty String.
    let target_name = target_name.as_string().unwrap_or(agent.names.empty);
    // 10. Perform SetFunctionName(F, targetName, "bound").
    set_function_name(agent, f, PropertyKey::String(target_name), Some("bound"));
    // 11. Return F.
    Ok(Value::Object(f))
}
