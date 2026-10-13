//! eval ( _source_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-eval-x

use kahvi_core::prelude::*;

/// eval
pub(crate) fn eval(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function is the %eval% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Return ? PerformEval(_source_, *false*, *false*).
    //
    // [subclause sec-performeval: PerformEval (
    // _source_: an ECMAScript language value,
    // _strictCaller_: a Boolean,
    // _direct_: a Boolean,
    // ): either a normal completion containing an ECMAScript language value or a throw completion]
    // [subclause sec-hostensurecancompilestrings: HostEnsureCanCompileStrings (
    // _calleeRealm_: a Realm Record,
    // _paramStrings_: a List of Strings,
    // _bodyString_: a String,
    // _direct_: a Boolean,
    // ): either a normal completion containing ~unused~ or a throw completion]
    // [subclause sec-evaldeclarationinstantiation: EvalDeclarationInstantiation (
    // _body_: a |ScriptBody| Parse Node,
    // _variableEnv_: an Environment Record,
    // _lexicalEnv_: a Declarative Environment Record,
    // _privateEnv_: a PrivateEnvironment Record or *null*,
    // _strict_: a Boolean,
    // ): either a normal completion containing ~unused~ or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("eval"))
}
