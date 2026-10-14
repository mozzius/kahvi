//! set Iterator.prototype [ %Symbol.toStringTag% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-set-iterator.prototype-%symbol.tostringtag%

use kahvi_core::prelude::*;

use crate::support::iterator_helper::setter_that_ignores_prototype_properties;

/// set Iterator.prototype[Symbol.toStringTag]
pub(crate) fn prototype_set_symbol_to_string_tag(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The value of the [[Setter]] attribute is a built-in function that takes an argument _v_. It performs the following steps when called:
    //
    let _ = new_target;
    let v = args.first().copied().unwrap_or(Value::Undefined);
    // 1. Perform ? SetterThatIgnoresPrototypeProperties(*this* value, %Iterator.prototype%, %Symbol.toStringTag%, _v_).
    let home = agent.intrinsic("%Iterator.prototype%");
    let key = PropertyKey::Symbol(agent.symbols_wk.to_string_tag);
    setter_that_ignores_prototype_properties(agent, this, home, key, v)?;
    // 2. Return *undefined*.
    Ok(Value::Undefined)
}
