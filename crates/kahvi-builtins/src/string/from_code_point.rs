//! String.fromCodePoint ( ..._codePoints_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.fromcodepoint

use kahvi_core::prelude::*;

/// String.fromCodePoint
pub(crate) fn from_code_point(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function may be called with any number of arguments which form the rest parameter _codePoints_.
    // It performs the following steps when called:
    //
    // 1. Let _result_ be the empty String.
    // 2. For each element _next_ of _codePoints_, do
    //    a. Let _nextCP_ be ? ToNumber(_next_).
    //    b. If _nextCP_ is not an integral Number, throw a *RangeError* exception.
    //    c. If ℝ(_nextCP_) < 0 or ℝ(_nextCP_) > 0x10FFFF, throw a *RangeError* exception.
    //    d. Set _result_ to the string-concatenation of _result_ and UTF16EncodeCodePoint(ℝ(_nextCP_)).
    // 3. Assert: If _codePoints_ is empty, then _result_ is the empty String.
    // 4. Return _result_.
    //
    // The *"length"* property of this function is *1*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.fromCodePoint"))
}
