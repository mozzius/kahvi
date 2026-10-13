//! encodeURI ( _uri_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-encodeuri-uri

use kahvi_core::prelude::*;

/// encodeURI
pub(crate) fn encode_uri(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function computes a new version of a UTF-16 encoded ([sec-ecmascript-language-types-string-type]) URI in which each instance of certain code points is replaced by one, two, three, or four escape sequences representing the UTF-8 encoding of the code point.
    // It is the %encodeURI% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _uriString_ be ? ToString(_uri_).
    // 2. Let _extraUnescaped_ be *";/?:@&=+$,#"*.
    // 3. Return ? Encode(_uriString_, _extraUnescaped_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("encodeURI"))
}
