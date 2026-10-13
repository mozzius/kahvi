//! FinalizationRegistry ( _cleanupCallback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-finalization-registry-cleanup-callback

use kahvi_core::prelude::*;

/// FinalizationRegistry
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. If IsCallable(_cleanupCallback_) is *false*, throw a *TypeError* exception.
    // 3. Let _finalizationRegistry_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%FinalizationRegistry.prototype%"*, « [[Realm]], [[CleanupCallback]], [[Cells]] »).
    // 4. Let _activeFunc_ be the active function object.
    // 5. Set _finalizationRegistry_.[[Realm]] to _activeFunc_.[[Realm]].
    // 6. Set _finalizationRegistry_.[[CleanupCallback]] to HostMakeJobCallback(_cleanupCallback_).
    // 7. Set _finalizationRegistry_.[[Cells]] to a new empty List.
    // 8. Return _finalizationRegistry_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("FinalizationRegistry"))
}
