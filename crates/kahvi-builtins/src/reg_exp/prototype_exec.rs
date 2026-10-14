//! RegExp.prototype.exec ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype.exec

use kahvi_core::prelude::*;

use crate::support::regexp::{regexp_builtin_exec, require_regexp};

/// RegExp.prototype.exec
pub(crate) fn prototype_exec(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let R be the this value.
    // 2. Perform ? RequireInternalSlot(R, [[RegExpMatcher]]).
    let r = require_regexp(agent, this)?;
    // 3. Let S be ? ToString(string).
    let s = ops::to_string(agent, args.get_or_undefined(0))?;
    // 4. Return ? RegExpBuiltinExec(R, S).
    regexp_builtin_exec(agent, r, s)
}
