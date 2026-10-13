//! decodeURIComponent ( _encodedURIComponent_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-decodeuricomponent-encodeduricomponent

use kahvi_core::prelude::*;

/// decodeURIComponent
pub(crate) fn decode_uri_component(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function computes a new version of a URI in which each escape sequence and UTF-8 encoding of the sort that might be introduced by the `encodeURIComponent` function is replaced with the UTF-16 encoding of the code point that it represents.
    // It is the %decodeURIComponent% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _componentString_ be ? ToString(_encodedURIComponent_).
    // 2. Let _preserveEscapeSet_ be the empty String.
    // 3. Return ? Decode(_componentString_, _preserveEscapeSet_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("decodeURIComponent"))
}
