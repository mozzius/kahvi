//! String.prototype.trimRight ( )
//!
//! Spec: https://tc39.es/ecma262/#String.prototype.trimright

use kahvi_core::prelude::*;

/// String.prototype.trimRight
pub(crate) fn prototype_trim_right(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The property *"trimEnd"* is preferred. The *"trimRight"* property is provided principally for compatibility with old code. It is recommended that the *"trimEnd"* property be used in new ECMAScript code.
    // The initial value of the *"trimRight"* property is %String.prototype.trimEnd%, defined in [sec-string.prototype.trimend].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.trimRight"))
}
