//! encodeURIComponent ( _uriComponent_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-encodeuricomponent-uricomponent

use kahvi_core::prelude::*;

/// encodeURIComponent
pub(crate) fn encode_uri_component(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function computes a new version of a UTF-16 encoded ([sec-ecmascript-language-types-string-type]) URI in which each instance of certain code points is replaced by one, two, three, or four escape sequences representing the UTF-8 encoding of the code point.
    // It is the %encodeURIComponent% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _componentString_ be ? ToString(_uriComponent_).
    // 2. Let _extraUnescaped_ be the empty String.
    // 3. Return ? Encode(_componentString_, _extraUnescaped_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("encodeURIComponent"))
}
