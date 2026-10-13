//! String.prototype.link ( _url_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.link

use kahvi_core::prelude::*;

/// String.prototype.link
pub(crate) fn prototype_link(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    // 2. Return ? CreateHTML(_string_, *"a"*, *"href"*, _url_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.link"))
}
