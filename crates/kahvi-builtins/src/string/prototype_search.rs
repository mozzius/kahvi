//! String.prototype.search ( _regexpOrPattern_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.search

use crate::support::regexp::regexp_create;
use kahvi_core::prelude::*;

/// String.prototype.search
pub(crate) fn prototype_search(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. If _regexpOrPattern_ is an Object, then
    let regexp_or_pattern = args.get_or_undefined(0);
    if regexp_or_pattern.is_object() {
        // a. Let _searcher_ be ? GetMethod(_regexpOrPattern_, %Symbol.search%).
        let searcher = ops::get_method(
            agent,
            regexp_or_pattern,
            PropertyKey::Symbol(agent.symbols_wk.search),
        )?;
        // b. If _searcher_ is not *undefined*, then
        if !matches!(searcher, Value::Undefined) {
            // i. Return ? Call(_searcher_, _regexpOrPattern_, « _thisValue_ »).
            return ops::call(agent, searcher, regexp_or_pattern, &[this_value]);
        }
    }
    // 4. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 5. Let _regexp_ be ? RegExpCreate(_regexpOrPattern_, *undefined*).
    let regexp = regexp_create(agent, regexp_or_pattern, Value::Undefined)?;
    // 6. Return ? Invoke(_regexp_, %Symbol.search%, « _string_ »).
    ops::invoke(
        agent,
        Value::Object(regexp),
        PropertyKey::Symbol(agent.symbols_wk.search),
        &[Value::String(string)],
    )
}
