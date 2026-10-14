//! String ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string-constructor-string-value

use crate::support::this_value::symbol_descriptive_string;
use kahvi_core::prelude::*;

/// String
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If value is not present, then let s be the empty String.
    let s = match args.first() {
        None => agent.names.empty,
        Some(&value) => {
            // 2.a If NewTarget is undefined and value is a Symbol, return SymbolDescriptiveString(value).
            if new_target.is_none()
                && let Value::Symbol(sym) = value
            {
                return Ok(Value::String(symbol_descriptive_string(agent, sym)));
            }
            // 2.b Let s be ? ToString(value).
            ops::to_string(agent, value)?
        }
    };
    // 3. If NewTarget is undefined, return s.
    let Some(new_target) = new_target else {
        return Ok(Value::String(s));
    };
    // 4. Return StringCreate(s, ? GetPrototypeFromConstructor(NewTarget, "%String.prototype%")).
    let proto = ops::get_prototype_from_constructor(agent, new_target, "%String.prototype%")?;
    Ok(Value::Object(ops::string_create(agent, s, proto)))
}
