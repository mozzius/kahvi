//! RegExp.prototype.compile ( _pattern_, _flags_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype.compile

use kahvi_core::prelude::*;

use crate::support::regexp::{RegExpData, has_regexp_matcher, regexp_initialize, require_regexp};

/// RegExp.prototype.compile
pub(crate) fn prototype_compile(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
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
    // 1. Let obj be the this value.
    // 2. Perform ? RequireInternalSlot(obj, [[RegExpMatcher]]).
    let obj = require_regexp(agent, this)?;
    let mut pattern = args.get_or_undefined(0);
    let mut flags = args.get_or_undefined(1);
    // 3. If pattern is an Object and pattern has a [[RegExpMatcher]] internal slot, then
    if let Value::Object(p) = pattern
        && has_regexp_matcher(agent, p)
    {
        // a. If flags is not undefined, throw a TypeError exception.
        if !flags.is_undefined() {
            return Err(agent.throw_type_error("flags must be undefined when pattern is a RegExp"));
        }
        let Some(data) = agent.native::<RegExpData>(p) else {
            return Err(agent.throw_type_error("not a RegExp object"));
        };
        // b. Set flags to pattern.[[OriginalFlags]].
        flags = Value::String(data.original_flags);
        // c. Set pattern to pattern.[[OriginalSource]].
        pattern = Value::String(data.original_source);
    }
    // 4. Return ? RegExpInitialize(obj, pattern, flags).
    regexp_initialize(agent, obj, pattern, flags).map(Value::Object)
}
