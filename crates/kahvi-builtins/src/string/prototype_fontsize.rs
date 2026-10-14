//! String.prototype.fontsize ( _size_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.fontsize

use kahvi_core::prelude::*;

use crate::generated::string::prototype_anchor::create_html;

/// String.prototype.fontsize
pub(crate) fn prototype_fontsize(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let size = args.get_or_undefined(0);
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    let string = this;
    // 2. Return ? CreateHTML(_string_, *"font"*, *"size"*, _size_).
    create_html(agent, string, "font", "size", size)
}
