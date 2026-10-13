//! String.fromCharCode ( ..._codeUnits_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.fromcharcode

use kahvi_core::prelude::*;

/// String.fromCharCode
pub(crate) fn from_char_code(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function may be called with any number of arguments which form the rest parameter _codeUnits_.
    // It performs the following steps when called:
    //
    // 1. Let _result_ be the empty String.
    // 2. For each element _next_ of _codeUnits_, do
    //    a. Let _nextCU_ be the code unit whose numeric value is ℝ(? ToUint16(_next_)).
    //    b. Set _result_ to the string-concatenation of _result_ and _nextCU_.
    // 3. Return _result_.
    //
    // The *"length"* property of this function is *1*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.fromCharCode"))
}
