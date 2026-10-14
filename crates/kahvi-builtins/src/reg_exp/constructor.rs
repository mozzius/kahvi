//! RegExp ( _patternOrRegexp_, _flags_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp-pattern-flags

use kahvi_core::prelude::*;

use crate::support::regexp::{RegExpData, regexp_alloc, regexp_initialize};

/// RegExp
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let pattern_or_regexp = args.get_or_undefined(0);
    let mut flags = args.get_or_undefined(1);
    // 1. Let patternIsRegExp be ? IsRegExp(patternOrRegexp).
    let pattern_is_regexp = ops::is_regexp(agent, pattern_or_regexp)?;
    // 2. If NewTarget is undefined, then
    let new_target = match new_target {
        None => {
            // a. Let newTarget be the active function object.
            let active = agent
                .active_function()
                .expect("a built-in constructor runs as the active function");
            // b. If patternIsRegExp is true and flags is undefined, then
            if pattern_is_regexp && flags.is_undefined() {
                // i. Let patternCtor be ? Get(patternOrRegexp, "constructor").
                let Value::Object(p) = pattern_or_regexp else {
                    unreachable!("IsRegExp is only true for objects");
                };
                let key = PropertyKey::String(agent.names.constructor);
                let pattern_ctor = ops::get(agent, p, key)?;
                // ii. If SameValue(newTarget, patternCtor) is true, return patternOrRegexp.
                if ops::same_value(agent, Value::Object(active), pattern_ctor) {
                    return Ok(pattern_or_regexp);
                }
            }
            active
        }
        // 3. Else, let newTarget be NewTarget.
        Some(nt) => nt,
    };
    // 4. If patternOrRegexp is an Object and patternOrRegexp has a [[RegExpMatcher]] internal slot, then
    let existing = match pattern_or_regexp {
        Value::Object(o) => agent
            .native::<RegExpData>(o)
            .map(|d| (d.original_source, d.original_flags)),
        _ => None,
    };
    let pattern_source = if let Some((source, original_flags)) = existing {
        // a. Let P be patternOrRegexp.[[OriginalSource]].
        // b. If flags is undefined, let F be patternOrRegexp.[[OriginalFlags]]; else let F be flags.
        if flags.is_undefined() {
            flags = Value::String(original_flags);
        }
        Value::String(source)
    } else if pattern_is_regexp {
        // 5. Else if patternIsRegExp is true, then
        let Value::Object(p) = pattern_or_regexp else {
            unreachable!("IsRegExp is only true for objects");
        };
        // a. Let P be ? Get(patternOrRegexp, "source").
        let key = agent.key("source");
        let source = ops::get(agent, p, key)?;
        // b. If flags is undefined, then let F be ? Get(patternOrRegexp, "flags").
        if flags.is_undefined() {
            let key = agent.key("flags");
            flags = ops::get(agent, p, key)?;
        }
        source
    } else {
        // 6. Else, let P be patternOrRegexp and F be flags.
        pattern_or_regexp
    };
    // 7. Let O be ? RegExpAlloc(newTarget).
    let o = regexp_alloc(agent, new_target)?;
    // 8. Return ? RegExpInitialize(O, P, F).
    Ok(Value::Object(regexp_initialize(
        agent,
        o,
        pattern_source,
        flags,
    )?))
}
