//! String.prototype.trimLeft ( )
//!
//! Spec: https://tc39.es/ecma262/#String.prototype.trimleft

use kahvi_core::prelude::*;

/// String.prototype.trimLeft
pub(crate) fn prototype_trim_left(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The property *"trimStart"* is preferred. The *"trimLeft"* property is provided principally for compatibility with old code. It is recommended that the *"trimStart"* property be used in new ECMAScript code.
    // The initial value of the *"trimLeft"* property is %String.prototype.trimStart%, defined in [sec-string.prototype.trimstart].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.trimLeft"))
}
