//! String.prototype.strike ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.strike

use kahvi_core::prelude::*;

use super::prototype_anchor::create_html;

/// String.prototype.strike
pub(crate) fn prototype_strike(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    let string = this;
    // 2. Return ? CreateHTML(_string_, *"strike"*, *""*, *""*).
    create_html(agent, string, "strike", "", Value::Undefined)
}
