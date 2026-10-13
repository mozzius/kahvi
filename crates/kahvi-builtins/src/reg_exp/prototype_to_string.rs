//! RegExp.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype.tostring

use kahvi_core::prelude::*;

/// RegExp.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    // 3. Let _pattern_ be ? ToString(? Get(_regexp_, *"source"*)).
    // 4. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
    // 5. Let _result_ be the string-concatenation of *"/"*, _pattern_, *"/"*, and _flags_.
    // 6. Return _result_.
    //
    // NOTE:
    // The returned String has the form of a |RegularExpressionLiteral| that evaluates to another RegExp object with the same behaviour as this object.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype.toString"))
}
