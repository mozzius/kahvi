//! Function.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-function.prototype.tostring

use kahvi_core::prelude::*;

/// Function.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _func_ be the *this* value.
    // 2. If _func_ is an Object, _func_ has a [[SourceText]] internal slot, _func_.[[SourceText]] is a sequence of Unicode code points, and HostHasSourceTextAvailable(_func_) is *true*, then
    //    a. Return CodePointsToString(_func_.[[SourceText]]).
    // 3. If _func_ is a built-in function object, return an implementation-defined String source code representation of _func_. The representation must have the syntax of a |NativeFunction|. Additionally, if _func_ has an [[InitialName]] internal slot and _func_.[[InitialName]] is a String, the portion of the returned String that would be matched by |NativeFunctionAccessor?| |PropertyName| must be _func_.[[InitialName]].
    // 4. If _func_ is an Object and IsCallable(_func_) is *true*, return an implementation-defined String source code representation of _func_. The representation must have the syntax of a |NativeFunction|.
    // 5. Throw a *TypeError* exception.
    //
    // NativeFunction :
    // `function` NativeFunctionAccessor? PropertyName[~Yield, ~Await]? `(` FormalParameters[~Yield, ~Await] `)` `{` `[` `native` `code` `]` `}`
    // NativeFunctionAccessor :
    // `get`
    // `set`
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Function.prototype.toString"))
}
