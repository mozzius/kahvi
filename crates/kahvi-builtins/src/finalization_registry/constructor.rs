//! FinalizationRegistry ( _cleanupCallback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-finalization-registry-cleanup-callback

use kahvi_core::object::weak::{FinalizationRegistryData, track_finalization_registry};
use kahvi_core::prelude::*;

/// FinalizationRegistry
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let cleanup_callback = args.get_or_undefined(0);
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor FinalizationRegistry requires 'new'"));
    };
    // 2. If IsCallable(_cleanupCallback_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, cleanup_callback) {
        return Err(agent.throw_type_error("FinalizationRegistry: cleanup must be callable"));
    }
    // 4-5. Let _activeFunc_ be the active function object; [[Realm]] is _activeFunc_.[[Realm]].
    let realm = agent.current_realm();
    // 3. Let _finalizationRegistry_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%FinalizationRegistry.prototype%"*, « [[Realm]], [[CleanupCallback]], [[Cells]] »).
    // 6. Set _finalizationRegistry_.[[CleanupCallback]] to HostMakeJobCallback(_cleanupCallback_).
    // 7. Set _finalizationRegistry_.[[Cells]] to a new empty List.
    let registry = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%FinalizationRegistry.prototype%",
        ObjectKind::Native(Box::new(FinalizationRegistryData::new(
            realm,
            cleanup_callback,
        ))),
    )?;
    track_finalization_registry(agent, registry);
    // 8. Return _finalizationRegistry_.
    Ok(Value::Object(registry))
}
