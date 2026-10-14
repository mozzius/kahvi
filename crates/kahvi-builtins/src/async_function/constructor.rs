//! AsyncFunction ( ..._paramArgs_, _bodyArg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-async-function-constructor-arguments

use kahvi_core::prelude::*;
use kahvi_core::vm::{DynamicFunctionKind, create_dynamic_function};

/// AsyncFunction
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The last argument (if any) specifies the body (executable code) of an async function. Any preceding arguments specify formal parameters.
    // This function performs the following steps when called:
    //
    // 1. Let _activeFunc_ be the active function object.
    let active_func = agent
        .active_function()
        .expect("a built-in constructor runs as the active function");
    // 2. If _bodyArg_ is not present, set _bodyArg_ to the empty String.
    let (param_args, body_arg) = match args.split_last() {
        Some((&body, params)) => (params, body),
        None => (args, Value::String(agent.names.empty)),
    };
    // 3. Return ? CreateDynamicFunction(_activeFunc_, NewTarget, ~async~, _paramArgs_, _bodyArg_).
    let _ = this;
    let f = create_dynamic_function(
        agent,
        active_func,
        new_target,
        DynamicFunctionKind::Async,
        param_args,
        body_arg,
    )?;
    Ok(Value::Object(f))
}
