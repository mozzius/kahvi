//! WeakRef ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weak-ref-target

use kahvi_core::object::weak::{WeakRefData, add_to_kept_objects};
use kahvi_core::prelude::*;

use crate::support::collections::can_be_held_weakly;

/// WeakRef
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let target = args.get_or_undefined(0);
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor WeakRef requires 'new'"));
    };
    // 2. If CanBeHeldWeakly(_target_) is *false*, throw a *TypeError* exception.
    if !can_be_held_weakly(agent, target) {
        return Err(agent.throw_type_error("WeakRef: invalid target"));
    }
    // 3. Let _weakRef_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%WeakRef.prototype%"*, « [[WeakRefTarget]] »).
    // 5. Set _weakRef_.[[WeakRefTarget]] to _target_.
    let weak_ref = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%WeakRef.prototype%",
        ObjectKind::Native(Box::new(WeakRefData {
            target: Some(target),
        })),
    )?;
    // 4. Perform AddToKeptObjects(_target_).
    add_to_kept_objects(agent, target);
    // 6. Return _weakRef_.
    Ok(Value::Object(weak_ref))
}
