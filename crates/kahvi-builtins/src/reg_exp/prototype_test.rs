//! RegExp.prototype.test ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype.test

use kahvi_core::prelude::*;

use crate::support::regexp::regexp_exec;

/// RegExp.prototype.test
pub(crate) fn prototype_test(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let R be the this value.
    // 2. If R is not an Object, throw a TypeError exception.
    let Value::Object(r) = this else {
        return Err(agent.throw_type_error("RegExp.prototype.test called on a non-object"));
    };
    // 3. Let string be ? ToString(string).
    let s = ops::to_string(agent, args.get_or_undefined(0))?;
    // 4. Let match be ? RegExpExec(R, string).
    let m = regexp_exec(agent, r, s)?;
    // 5. If match is null, return false.
    // 6. Return true.
    Ok(Value::Boolean(!matches!(m, Value::Null)))
}
