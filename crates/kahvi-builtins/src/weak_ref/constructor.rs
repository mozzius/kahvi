//! WeakRef ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-weak-ref-target

use kahvi_core::prelude::*;

/// WeakRef
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. If CanBeHeldWeakly(_target_) is *false*, throw a *TypeError* exception.
    // 3. Let _weakRef_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%WeakRef.prototype%"*, « [[WeakRefTarget]] »).
    // 4. Perform AddToKeptObjects(_target_).
    // 5. Set _weakRef_.[[WeakRefTarget]] to _target_.
    // 6. Return _weakRef_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WeakRef"))
}
