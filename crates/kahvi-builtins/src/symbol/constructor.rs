//! Symbol ( [ _description_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol-description

use kahvi_core::prelude::*;

/// Symbol
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is not undefined, throw a TypeError exception.
    if new_target.is_some() {
        return Err(agent.throw_type_error("Symbol is not a constructor"));
    }
    // 2. If description is undefined, let descString be undefined.
    // 3. Else, let descString be ? ToString(description).
    let desc = match args.get_or_undefined(0) {
        Value::Undefined => None,
        d => Some(ops::to_string(agent, d)?),
    };
    // 4. Return a new Symbol whose [[Description]] is descString.
    Ok(Value::Symbol(agent.new_symbol(desc)))
}
