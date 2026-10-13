//! Function ( ..._paramArgs_, _bodyArg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-function-p1-p2-pn-body

use kahvi_core::prelude::*;

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
    // 2. If _bodyArg_ is not present, set _bodyArg_ to the empty String.
    // 3. Return ? CreateDynamicFunction(_ctor_, NewTarget, ~normal~, _paramArgs_, _bodyArg_).
    //
    // NOTE:
    // It is permissible but not necessary to have one argument for each formal parameter to be specified. For example, all three of the following expressions produce the same result:
    // new Function("a", "b", "c", "return a+b+c")
    // new Function("a, b, c", "return a+b+c")
    // new Function("a,b", "c", "return a+b+c")
    // [subclause sec-createdynamicfunction: CreateDynamicFunction (
    // _ctor_: a constructor,
    // _newTarget_: a constructor or *undefined*,
    // _kind_: ~normal~, ~generator~, ~async~, or ~async-generator~,
    // _paramArgs_: a List of ECMAScript language values,
    // _bodyArg_: an ECMAScript language value,
    // ): either a normal completion containing an ECMAScript function object or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Function"))
}
