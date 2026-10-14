//! WeakRef.prototype.deref ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-weak-ref.prototype.deref

use kahvi_core::object::weak::{WeakRefData, add_to_kept_objects};
use kahvi_core::prelude::*;

/// WeakRef.prototype.deref
pub(crate) fn prototype_deref(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _weakRef_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_weakRef_, [[WeakRefTarget]]).
    let target = match this {
        Value::Object(o) => match agent.native::<WeakRefData>(o) {
            Some(d) => d.target,
            None => return Err(agent.throw_type_error("not a WeakRef")),
        },
        _ => return Err(agent.throw_type_error("not a WeakRef")),
    };
    // 3. Return WeakRefDeref(_weakRef_).
    match target {
        // WeakRefDeref 2. If target is not ~empty~, then
        Some(t) => {
            // a. Perform AddToKeptObjects(target).
            add_to_kept_objects(agent, t);
            // b. Return target.
            Ok(t)
        }
        // 3. Return undefined.
        None => Ok(Value::Undefined),
    }
}
