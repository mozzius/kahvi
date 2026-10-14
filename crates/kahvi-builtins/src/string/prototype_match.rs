//! String.prototype.match ( _regexpOrPattern_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.match

use kahvi_core::prelude::*;

use crate::support::regexp::regexp_create;

/// String.prototype.match
pub(crate) fn prototype_match(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    let regexp_or_pattern = args.get_or_undefined(0);
    let match_key = PropertyKey::Symbol(agent.symbols_wk.r#match);
    // 3. If _regexpOrPattern_ is an Object, then
    if let Value::Object(_) = regexp_or_pattern {
        // a. Let _matcher_ be ? GetMethod(_regexpOrPattern_, %Symbol.match%).
        let matcher = ops::get_method(agent, regexp_or_pattern, match_key)?;
        // b. If _matcher_ is not *undefined*, then
        if !matcher.is_undefined() {
            // i. Return ? Call(_matcher_, _regexpOrPattern_, « _thisValue_ »).
            return ops::call(agent, matcher, regexp_or_pattern, &[this_value]);
        }
    }
    // 4. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 5. Let _regexp_ be ? RegExpCreate(_regexpOrPattern_, *undefined*).
    let regexp = regexp_create(agent, regexp_or_pattern, Value::Undefined)?;
    // 6. Return ? Invoke(_regexp_, %Symbol.match%, « _string_ »).
    ops::invoke(
        agent,
        Value::Object(regexp),
        match_key,
        &[Value::String(string)],
    )
}
