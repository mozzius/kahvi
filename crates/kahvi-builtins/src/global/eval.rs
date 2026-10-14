//! eval ( _source_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-eval-x

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// eval
pub(crate) fn eval(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? PerformEval(x, false, false).
    vm::perform_eval(agent, args.get_or_undefined(0), false, None)
}
