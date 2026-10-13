//! RegExp ( _patternOrRegexp_, _flags_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp-pattern-flags

use kahvi_core::prelude::*;

/// RegExp
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _patternIsRegExp_ be ? IsRegExp(_patternOrRegexp_).
    // 2. If NewTarget is *undefined*, then
    //    a. Let _newTarget_ be the active function object.
    //    b. If _patternIsRegExp_ is *true* and _flags_ is *undefined*, then
    //       i. Let _patternCtor_ be ? Get(_patternOrRegexp_, *"constructor"*).
    //       ii. If SameValue(_newTarget_, _patternCtor_) is *true*, return _patternOrRegexp_.
    // 3. Else,
    //    a. Let _newTarget_ be NewTarget.
    // 4. If _patternOrRegexp_ is an Object and _patternOrRegexp_ has a [[RegExpMatcher]] internal slot, then
    //    a. Let _patternSource_ be _patternOrRegexp_.[[OriginalSource]].
    //    b. If _flags_ is *undefined*, set _flags_ to _patternOrRegexp_.[[OriginalFlags]].
    // 5. Else if _patternIsRegExp_ is *true*, then
    //    a. Let _patternSource_ be ? Get(_patternOrRegexp_, *"source"*).
    //    b. If _flags_ is *undefined*, then
    //       i. Set _flags_ to ? Get(_patternOrRegexp_, *"flags"*).
    // 6. Else,
    //    a. Let _patternSource_ be _patternOrRegexp_.
    // 7. Let _obj_ be ? RegExpAlloc(_newTarget_).
    // 8. Return ? RegExpInitialize(_obj_, _patternSource_, _flags_).
    //
    // NOTE:
    // If pattern is supplied using a |StringLiteral|, the usual escape sequence substitutions are performed before the String is processed by this function. If pattern must contain an escape sequence to be recognized by this function, any U+005C (REVERSE SOLIDUS) code points must be escaped within the |StringLiteral| to prevent them being removed when the contents of the |StringLiteral| are formed.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp"))
}
