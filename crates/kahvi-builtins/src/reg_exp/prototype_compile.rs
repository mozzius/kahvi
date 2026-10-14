//! RegExp.prototype.compile ( _pattern_, _flags_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype.compile

use kahvi_core::prelude::*;

/// RegExp.prototype.compile
pub(crate) fn prototype_compile(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[RegExpMatcher]]).
    // 3. If _pattern_ is an Object and _pattern_ has a [[RegExpMatcher]] internal slot, then
    //    a. If _flags_ is not *undefined*, throw a *TypeError* exception.
    //    b. Set _flags_ to _pattern_.[[OriginalFlags]].
    //    c. Set _pattern_ to _pattern_.[[OriginalSource]].
    // 4. Return ? RegExpInitialize(_obj_, _pattern_, _flags_).
    //
    // NOTE:
    // This method completely reinitializes the *this* value RegExp with a new pattern and flags. An implementation may interpret use of this method as an assertion that the resulting RegExp object will be used multiple times and hence is a candidate for extra optimization.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype.compile"))
}
