//! get RegExp.prototype.global
//!
//! Spec: https://tc39.es/ecma262/#sec-get-regexp.prototype.global

use kahvi_core::prelude::*;

use crate::support::regexp::RegExpData;

/// get RegExp.prototype.global
pub(crate) fn prototype_get_global(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `RegExp.prototype.global` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    let regexp = this;
    // 2. Let _codeUnit_ be the code unit 0x0067 (LATIN SMALL LETTER G).
    let code_unit: u16 = 0x0067;
    // 3. Return ? RegExpHasFlag(_regexp_, _codeUnit_).
    let _ = (args, new_target);
    regexp_has_flag(agent, regexp, code_unit)
}

/// RegExpHasFlag(regexp, codeUnit) (§22.2.6.4.1)
fn regexp_has_flag(agent: &mut Agent, regexp: Value, code_unit: u16) -> JsResult<Value> {
    // 1. If _regexp_ is not an Object, throw a *TypeError* exception.
    let Value::Object(o) = regexp else {
        return Err(agent.throw_type_error("RegExp flag getter called on a non-object"));
    };
    // 2. If _regexp_ does not have an [[OriginalFlags]] internal slot, then
    let Some(data) = agent.native::<RegExpData>(o) else {
        // a. If SameValue(_regexp_, %RegExp.prototype%) is *true*, return *undefined*.
        let proto = agent.intrinsic("%RegExp.prototype%");
        if ops::same_value(agent, regexp, Value::Object(proto)) {
            return Ok(Value::Undefined);
        }
        // b. Throw a *TypeError* exception.
        return Err(agent.throw_type_error("RegExp flag getter called on a non-RegExp object"));
    };
    // 3. Let _flags_ be _regexp_.[[OriginalFlags]].
    let flags = data.original_flags;
    // 4. If _flags_ contains _codeUnit_, return *true*.
    // 5. Return *false*.
    let contains = agent.str(flags).contains(&code_unit);
    Ok(Value::Boolean(contains))
}
