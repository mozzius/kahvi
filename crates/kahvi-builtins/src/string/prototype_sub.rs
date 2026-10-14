//! String.prototype.sub ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.sub

use kahvi_core::prelude::*;

use super::prototype_anchor::create_html;

/// String.prototype.sub
pub(crate) fn prototype_sub(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = args;
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    let string = this;
    // 2. Return ? CreateHTML(_string_, *"sub"*, *""*, *""*).
    let empty = Value::String(agent.intern(&[]));
    create_html(agent, string, "sub", "", empty)
}
