//! AsyncGeneratorFunction ( ..._paramArgs_, _bodyArg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncgeneratorfunction

use kahvi_core::prelude::*;

/// AsyncGeneratorFunction
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The last argument (if any) specifies the body (executable code) of an async generator function; any preceding arguments specify formal parameters.
    // This function performs the following steps when called:
    //
    // 1. Let _activeFunc_ be the active function object.
    // 2. If _bodyArg_ is not present, set _bodyArg_ to the empty String.
    // 3. Return ? CreateDynamicFunction(_activeFunc_, NewTarget, ~async-generator~, _paramArgs_, _bodyArg_).
    //
    // NOTE:
    // See NOTE for [sec-function-p1-p2-pn-body].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncGeneratorFunction"))
}
