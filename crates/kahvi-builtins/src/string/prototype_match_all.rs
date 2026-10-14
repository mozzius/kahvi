//! String.prototype.matchAll ( _regexpOrPattern_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.matchall

use kahvi_core::prelude::*;

/// String.prototype.matchAll
pub(crate) fn prototype_match_all(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs a regular expression match of the String representing the *this* value against _regexpOrPattern_ and returns an iterator that yields match results. Each match result is an Array containing the matched portion of the String as the first element, followed by the portions matched by any capturing groups. If the regular expression never matches, the returned iterator does not yield any match results.
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. If _regexpOrPattern_ is an Object, then
    //    a. Let _isRegexp_ be ? IsRegExp(_regexpOrPattern_).
    //    b. If _isRegexp_ is *true*, then
    //       i. Let _flags_ be ? Get(_regexpOrPattern_, *"flags"*).
    //       ii. Perform ? RequireObjectCoercible(_flags_).
    //       iii. If ? ToString(_flags_) does not contain *"g"*, throw a *TypeError* exception.
    //    c. Let _matcher_ be ? GetMethod(_regexpOrPattern_, %Symbol.matchAll%).
    //    d. If _matcher_ is not *undefined*, then
    //       i. Return ? Call(_matcher_, _regexpOrPattern_, « _thisValue_ »).
    // 4. Let _string_ be ? ToString(_thisValue_).
    // 5. Let _regexp_ be ? RegExpCreate(_regexpOrPattern_, *"g"*).
    // 6. Return ? Invoke(_regexp_, %Symbol.matchAll%, « _string_ »).
    //
    // NOTE: This method is intentionally generic, it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    // NOTE: Similarly to `String.prototype.split`, `String.prototype.matchAll` is designed to typically act without mutating its inputs.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.matchAll"))
}
