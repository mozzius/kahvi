//! FinalizationRegistry.prototype.register ( _target_, _heldValue_ [ , _unregisterToken_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-finalization-registry.prototype.register

use kahvi_core::object::weak::{FinalizationCell, FinalizationRegistryData};
use kahvi_core::prelude::*;

use crate::support::collections::can_be_held_weakly;

/// FinalizationRegistry.prototype.register
pub(crate) fn prototype_register(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let target = args.get_or_undefined(0);
    let held_value = args.get_or_undefined(1);
    let unregister_token = args.get_or_undefined(2);
    // 1. Let _finalizationRegistry_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_finalizationRegistry_, [[Cells]]).
    let registry = match this {
        Value::Object(o) if agent.native::<FinalizationRegistryData>(o).is_some() => o,
        _ => return Err(agent.throw_type_error("not a FinalizationRegistry")),
    };
    // 3. If CanBeHeldWeakly(_target_) is *false*, throw a *TypeError* exception.
    if !can_be_held_weakly(agent, target) {
        return Err(
            agent.throw_type_error("FinalizationRegistry.prototype.register: invalid target")
        );
    }
    // 4. If SameValue(_target_, _heldValue_) is *true*, throw a *TypeError* exception.
    if ops::same_value(agent, target, held_value) {
        return Err(agent.throw_type_error("the target and the held value must differ"));
    }
    // 5. If CanBeHeldWeakly(_unregisterToken_) is *false*, then
    let unregister_token = if can_be_held_weakly(agent, unregister_token) {
        Some(unregister_token)
    } else {
        // a. If _unregisterToken_ is not *undefined*, throw a *TypeError* exception.
        if !unregister_token.is_undefined() {
            return Err(agent.throw_type_error("invalid unregister token"));
        }
        // b. Set _unregisterToken_ to ~empty~.
        None
    };
    // 6. Let _cell_ be the Record { [[WeakRefTarget]]: _target_, [[HeldValue]]: _heldValue_, [[UnregisterToken]]: _unregisterToken_ }.
    let cell = FinalizationCell {
        target: Some(target),
        held_value,
        unregister_token,
    };
    // 7. Append _cell_ to _finalizationRegistry_.[[Cells]].
    agent
        .native_mut::<FinalizationRegistryData>(registry)
        .expect("[[Cells]]")
        .cells
        .push(cell);
    // 8. Return *undefined*.
    Ok(Value::Undefined)
}
