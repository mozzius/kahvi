//! String.prototype.link ( _url_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.link

use kahvi_core::prelude::*;

use super::prototype_anchor::create_html;

/// String.prototype.link
pub(crate) fn prototype_link(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let url = args.get_or_undefined(0);
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    let string = this;
    // 2. Return ? CreateHTML(_string_, *"a"*, *"href"*, _url_).
    create_html(agent, string, "a", "href", url)
}
