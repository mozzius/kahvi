//! String.prototype.big ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.big

use kahvi_core::prelude::*;

use super::prototype_anchor::create_html;

/// String.prototype.big
pub(crate) fn prototype_big(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    let string = this;
    // 2. Return ? CreateHTML(_string_, *"big"*, *""*, *""*).
    let empty = Value::String(agent.intern(&[]));
    create_html(agent, string, "big", "", empty)
}
