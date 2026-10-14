//! Function ( ..._paramArgs_, _bodyArg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-function-p1-p2-pn-body

use kahvi_core::prelude::*;
use kahvi_core::vm::{DynamicFunctionKind, create_dynamic_function};

/// Function
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The last argument (if any) specifies the body (executable code) of a function; any preceding arguments specify formal parameters.
    // This function performs the following steps when called:
    //
    // 1. Let _ctor_ be the active function object.
    let ctor = agent
        .active_function()
        .expect("a built-in constructor runs as the active function");
    // 2. If _bodyArg_ is not present, set _bodyArg_ to the empty String.
    let (param_args, body_arg) = match args.split_last() {
        Some((&body, params)) => (params, body),
        None => (args, Value::String(agent.names.empty)),
    };
    // 3. Return ? CreateDynamicFunction(_ctor_, NewTarget, ~normal~, _paramArgs_, _bodyArg_).
    let _ = this;
    let f = create_dynamic_function(
        agent,
        ctor,
        new_target,
        DynamicFunctionKind::Normal,
        param_args,
        body_arg,
    )?;
    Ok(Value::Object(f))
}
