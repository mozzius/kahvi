//! get Iterator.prototype [ %Symbol.toStringTag% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-get-iterator.prototype-%symbol.tostringtag%

use kahvi_core::prelude::*;

/// get Iterator.prototype[Symbol.toStringTag]
pub(crate) fn prototype_get_symbol_to_string_tag(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The value of the [[Getter]] attribute is a built-in function that requires no arguments. It performs the following steps when called:
    //
    // 1. Return *"Iterator"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Iterator.prototype[Symbol.toStringTag]"))
}
