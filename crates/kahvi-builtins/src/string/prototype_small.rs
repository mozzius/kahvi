//! String.prototype.small ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.small

use kahvi_core::prelude::*;

use super::prototype_anchor::create_html;

/// String.prototype.small
pub(crate) fn prototype_small(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    let string = this;
    // 2. Return ? CreateHTML(_string_, *"small"*, *""*, *""*).
    create_html(agent, string, "small", "", Value::Undefined)
}
