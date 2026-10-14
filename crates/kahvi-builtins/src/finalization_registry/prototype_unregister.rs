//! FinalizationRegistry.prototype.unregister ( _unregisterToken_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-finalization-registry.prototype.unregister

use kahvi_core::object::weak::FinalizationRegistryData;
use kahvi_core::prelude::*;

use crate::support::collections::can_be_held_weakly;

/// FinalizationRegistry.prototype.unregister
pub(crate) fn prototype_unregister(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let unregister_token = args.get_or_undefined(0);
    // 1. Let _finalizationRegistry_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_finalizationRegistry_, [[Cells]]).
    let registry = match this {
        Value::Object(o) if agent.native::<FinalizationRegistryData>(o).is_some() => o,
        _ => return Err(agent.throw_type_error("not a FinalizationRegistry")),
    };
    // 3. If CanBeHeldWeakly(_unregisterToken_) is *false*, throw a *TypeError* exception.
    if !can_be_held_weakly(agent, unregister_token) {
        return Err(agent.throw_type_error("invalid unregister token"));
    }
    // 4. Let _removed_ be *false*.
    // 5. For each cell of _finalizationRegistry_.[[Cells]], if cell.[[UnregisterToken]] is not
    //    ~empty~ and SameValue(cell.[[UnregisterToken]], _unregisterToken_) is *true*, remove cell
    //    and set _removed_ to *true*. (Tokens are objects or symbols: SameValue is identity.)
    let cells = &mut agent
        .native_mut::<FinalizationRegistryData>(registry)
        .expect("[[Cells]]")
        .cells;
    let before = cells.len();
    cells.retain(|c| !matches!(c.unregister_token, Some(t) if same_handle(t, unregister_token)));
    let removed = cells.len() != before;
    // 6. Return _removed_.
    Ok(Value::Boolean(removed))
}

/// SameValue for two values that can be held weakly (objects or symbols).
fn same_handle(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => x == y,
        (Value::Symbol(x), Value::Symbol(y)) => x == y,
        _ => false,
    }
}
