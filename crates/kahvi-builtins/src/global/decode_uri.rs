//! decodeURI ( _encodedURI_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-decodeuri-encodeduri

use kahvi_core::prelude::*;

/// decodeURI
pub(crate) fn decode_uri(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function computes a new version of a URI in which each escape sequence and UTF-8 encoding of the sort that might be introduced by the `encodeURI` function is replaced with the UTF-16 encoding of the code point that it represents. Escape sequences that could not have been introduced by `encodeURI` are not replaced.
    // It is the %decodeURI% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _uriString_ be ? ToString(_encodedURI_).
    // 2. Let _preserveEscapeSet_ be *";/?:@&=+$,#"*.
    // 3. Return ? Decode(_uriString_, _preserveEscapeSet_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("decodeURI"))
}
