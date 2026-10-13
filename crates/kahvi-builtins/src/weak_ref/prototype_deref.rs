//! WeakRef.prototype.deref ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-weak-ref.prototype.deref

use kahvi_core::prelude::*;

/// WeakRef.prototype.deref
pub(crate) fn prototype_deref(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _weakRef_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakRef_, [[WeakRefTarget]]).
    // 3. Return WeakRefDeref(_weakRef_).
    //
    // NOTE:
    // If the WeakRef returns a _target_ value that is not *undefined*, then this _target_ value should not be garbage collected until the current execution of ECMAScript code has completed. The AddToKeptObjects operation makes sure read consistency is maintained.
    // let target = { foo() {} };
    // let weakRef = new WeakRef(target);
    // // ... later ...
    // if (weakRef.deref()) {
    // weakRef.deref().foo();
    // }
    // In the above example, if the first deref does not evaluate to *undefined* then the second deref cannot either.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WeakRef.prototype.deref"))
}
