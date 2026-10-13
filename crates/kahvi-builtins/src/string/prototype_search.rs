//! String.prototype.search ( _regexpOrPattern_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.search

use kahvi_core::prelude::*;

/// String.prototype.search
pub(crate) fn prototype_search(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. If _regexpOrPattern_ is an Object, then
    //    a. Let _searcher_ be ? GetMethod(_regexpOrPattern_, %Symbol.search%).
    //    b. If _searcher_ is not *undefined*, then
    //       i. Return ? Call(_searcher_, _regexpOrPattern_, « _thisValue_ »).
    // 4. Let _string_ be ? ToString(_thisValue_).
    // 5. Let _regexp_ be ? RegExpCreate(_regexpOrPattern_, *undefined*).
    // 6. Return ? Invoke(_regexp_, %Symbol.search%, « _string_ »).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.search"))
}
