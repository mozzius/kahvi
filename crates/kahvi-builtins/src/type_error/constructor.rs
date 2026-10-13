//! TypeError ( _message_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-nativeerror

use kahvi_core::prelude::*;

/// TypeError
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // Each _NativeError_ function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
    // 2. Let _obj_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%NativeError.prototype%"*, « [[ErrorData]] »).
    // 3. If _message_ is not *undefined*, then
    //    a. Let _messageString_ be ? ToString(_message_).
    //    b. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"message"*, _messageString_).
    // 4. Perform ? InstallErrorCause(_obj_, _options_).
    // 5. Return _obj_.
    //
    // The actual value of the string passed in step [step-nativeerror-ordinarycreatefromconstructor] is either *"%EvalError.prototype%"*, *"%RangeError.prototype%"*, *"%ReferenceError.prototype%"*, *"%SyntaxError.prototype%"*, *"%TypeError.prototype%"*, or *"%URIError.prototype%"* corresponding to which _NativeError_ constructor is being defined.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypeError"))
}
