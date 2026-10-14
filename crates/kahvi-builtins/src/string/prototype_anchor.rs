//! String.prototype.anchor ( _name_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.anchor

use kahvi_core::prelude::*;

/// String.prototype.anchor
pub(crate) fn prototype_anchor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    // 2. Return ? CreateHTML(_string_, *"a"*, *"name"*, _name_).
    //
    // [subclause sec-createhtml: CreateHTML (
    // _contents_: an ECMAScript language value,
    // _tag_: a String,
    // _attr_: a String,
    // _attrValue_: an ECMAScript language value,
    // ): either a normal completion containing a String or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.anchor"))
}
