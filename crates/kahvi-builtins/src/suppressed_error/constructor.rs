//! SuppressedError ( _error_, _suppressed_, _message_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-suppressederror

use kahvi_core::prelude::*;

/// SuppressedError
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
    // 2. Let _obj_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%SuppressedError.prototype%"*, « [[ErrorData]] »).
    // 3. If _message_ is not *undefined*, then
    //    a. Let _messageString_ be ? ToString(_message_).
    //    b. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"message"*, _messageString_).
    // 4. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"error"*, _error_).
    // 5. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"suppressed"*, _suppressed_).
    // 6. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("SuppressedError"))
}
