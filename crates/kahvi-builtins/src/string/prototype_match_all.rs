//! String.prototype.matchAll ( _regexpOrPattern_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.matchall

use kahvi_core::prelude::*;

use crate::support::regexp::regexp_create;

/// String.prototype.matchAll
pub(crate) fn prototype_match_all(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs a regular expression match of the String representing the *this* value against _regexpOrPattern_ and returns an iterator that yields match results. Each match result is an Array containing the matched portion of the String as the first element, followed by the portions matched by any capturing groups. If the regular expression never matches, the returned iterator does not yield any match results.
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    let regexp_or_pattern = args.get_or_undefined(0);
    let match_all_key = PropertyKey::Symbol(agent.symbols_wk.match_all);
    // 3. If _regexpOrPattern_ is an Object, then
    if let Value::Object(_) = regexp_or_pattern {
        // a. Let _isRegexp_ be ? IsRegExp(_regexpOrPattern_).
        let is_regexp = ops::is_regexp(agent, regexp_or_pattern)?;
        // b. If _isRegexp_ is *true*, then
        if is_regexp {
            // i. Let _flags_ be ? Get(_regexpOrPattern_, *"flags"*).
            let flags_key = agent.key("flags");
            let flags = ops::get_v(agent, regexp_or_pattern, flags_key)?;
            // ii. Perform ? RequireObjectCoercible(_flags_).
            ops::require_object_coercible(agent, flags)?;
            // iii. If ? ToString(_flags_) does not contain *"g"*, throw a *TypeError* exception.
            let flags = ops::to_string(agent, flags)?;
            if !agent.str(flags).contains(&(b'g' as u16)) {
                return Err(agent.throw_type_error(
                    "String.prototype.matchAll called with a non-global RegExp argument",
                ));
            }
        }
        // c. Let _matcher_ be ? GetMethod(_regexpOrPattern_, %Symbol.matchAll%).
        let matcher = ops::get_method(agent, regexp_or_pattern, match_all_key)?;
        // d. If _matcher_ is not *undefined*, then
        if !matcher.is_undefined() {
            // i. Return ? Call(_matcher_, _regexpOrPattern_, « _thisValue_ »).
            return ops::call(agent, matcher, regexp_or_pattern, &[this_value]);
        }
    }
    // 4. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 5. Let _regexp_ be ? RegExpCreate(_regexpOrPattern_, *"g"*).
    let g = agent.string("g");
    let regexp = regexp_create(agent, regexp_or_pattern, Value::String(g))?;
    // 6. Return ? Invoke(_regexp_, %Symbol.matchAll%, « _string_ »).
    ops::invoke(
        agent,
        Value::Object(regexp),
        match_all_key,
        &[Value::String(string)],
    )
}
