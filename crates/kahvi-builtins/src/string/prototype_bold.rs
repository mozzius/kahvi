//! String.prototype.bold ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.bold

use kahvi_core::prelude::*;

use super::prototype_anchor::create_html;

/// String.prototype.bold
pub(crate) fn prototype_bold(
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
    // 2. Return ? CreateHTML(_string_, *"b"*, *""*, *""*).
    let empty = Value::String(agent.intern(&[]));
    create_html(agent, string, "b", "", empty)
}
