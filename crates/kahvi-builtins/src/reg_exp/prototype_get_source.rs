//! get RegExp.prototype.source
//!
//! Spec: https://tc39.es/ecma262/#sec-get-regexp.prototype.source

use kahvi_core::prelude::*;

/// get RegExp.prototype.source
pub(crate) fn prototype_get_source(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `RegExp.prototype.source` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    // 3. If _regexp_ does not have an [[OriginalSource]] internal slot, then
    //    a. If SameValue(_regexp_, %RegExp.prototype%) is *true*, return *"(?:)"*.
    //    b. Throw a *TypeError* exception.
    // 4. Assert: _regexp_ has an [[OriginalFlags]] internal slot.
    // 5. Let _source_ be _regexp_.[[OriginalSource]].
    // 6. Let _flags_ be _regexp_.[[OriginalFlags]].
    // 7. Return EscapeRegExpPattern(_source_, _flags_).
    //
    // [subclause sec-escaperegexppattern: EscapeRegExpPattern (
    // _pattern_: a String,
    // _flags_: a String,
    // ): a String]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get RegExp.prototype.source"))
}
