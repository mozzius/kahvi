//! WeakRef and FinalizationRegistry slots (§26.1, §26.2): their weak edges
//! are cleared by the garbage collector, which also schedules the cleanup
//! jobs of registries whose targets died (HostEnqueueFinalizationRegistryCleanupJob).

use crate::agent::{Agent, Job};
use crate::error::JsResult;
use crate::gc::{Liveness, Trace, Tracer};
use crate::realm::RealmId;
use crate::value::{Object, Value};

/// [[WeakRefTarget]] of a WeakRef: `None` once the target was collected.
#[derive(Debug)]
pub struct WeakRefData {
    pub target: Option<Value>,
}

impl Trace for WeakRefData {
    // The target is held weakly.
    fn trace(&self, _t: &mut Tracer) {}

    fn sweep_weak(&mut self, live: &Liveness) {
        if self.target.is_some_and(|v| !live.is_live(v)) {
            self.target = None;
        }
    }
}

/// A FinalizationRegistry cell: { [[WeakRefTarget]], [[HeldValue]], [[UnregisterToken]] }.
#[derive(Debug, Clone, Copy)]
pub struct FinalizationCell {
    /// `None` once the target was collected (~empty~).
    pub target: Option<Value>,
    pub held_value: Value,
    /// `None` for ~empty~ (and once the token was collected).
    pub unregister_token: Option<Value>,
}

/// [[Realm]], [[CleanupCallback]] and [[Cells]] of a FinalizationRegistry.
#[derive(Debug)]
pub struct FinalizationRegistryData {
    pub realm: RealmId,
    pub cleanup_callback: Value,
    pub cells: Vec<FinalizationCell>,
    /// A cleanup job is queued and has not run yet.
    pub(crate) cleanup_queued: bool,
}

impl FinalizationRegistryData {
    pub fn new(realm: RealmId, cleanup_callback: Value) -> FinalizationRegistryData {
        FinalizationRegistryData {
            realm,
            cleanup_callback,
            cells: Vec::new(),
            cleanup_queued: false,
        }
    }
}

impl Trace for FinalizationRegistryData {
    fn trace(&self, t: &mut Tracer) {
        t.value(self.cleanup_callback);
        // A cell's [[HeldValue]] is live while the registry holds the cell;
        // its target and unregister token are held weakly.
        for c in &self.cells {
            t.value(c.held_value);
        }
    }

    fn sweep_weak(&mut self, live: &Liveness) {
        for c in &mut self.cells {
            if c.target.is_some_and(|v| !live.is_live(v)) {
                c.target = None;
            }
            if c.unregister_token.is_some_and(|v| !live.is_live(v)) {
                c.unregister_token = None;
            }
        }
    }
}

/// AddToKeptObjects(value) (§9.11): `value` stays alive until ClearKeptObjects.
pub fn add_to_kept_objects(agent: &mut Agent, value: Value) {
    if let Value::Object(o) = value {
        agent.kept_objects.push(o);
    } else {
        agent.kept_symbols.push(value);
    }
}

/// Tells the collector about a FinalizationRegistry object.
pub fn track_finalization_registry(agent: &mut Agent, registry: Object) {
    agent.finalization_registries.push(registry);
}

/// After a collection: forgets dead registries and queues the cleanup job
/// of each live one that has a cell whose target died.
pub(crate) fn schedule_cleanups(agent: &mut Agent, live: &Liveness) {
    let mut registries = std::mem::take(&mut agent.finalization_registries);
    registries.retain(|&r| live.is_live_object(r));
    for &r in &registries {
        let Some(data) = agent.native_mut::<FinalizationRegistryData>(r) else {
            continue;
        };
        if !data.cleanup_queued && data.cells.iter().any(|c| c.target.is_none()) {
            data.cleanup_queued = true;
            let realm = data.realm;
            agent.enqueue_job(Job::FinalizationRegistryCleanup { realm, registry: r });
        }
    }
    agent.finalization_registries = registries;
}

/// The cleanupJob of HostEnqueueFinalizationRegistryCleanupJob: performs
/// CleanupFinalizationRegistry(finalizationRegistry) (§9.13).
pub(crate) fn cleanup_finalization_registry(agent: &mut Agent, registry: Object) -> JsResult<()> {
    // 2. Let callback be finalizationRegistry.[[CleanupCallback]].
    let callback = match agent.native_mut::<FinalizationRegistryData>(registry) {
        Some(d) => {
            d.cleanup_queued = false;
            d.cleanup_callback
        }
        None => return Ok(()),
    };
    // 3. While finalizationRegistry.[[Cells]] contains a Record cell such that
    //    cell.[[WeakRefTarget]] is ~empty~, ...
    loop {
        let Some(d) = agent.native_mut::<FinalizationRegistryData>(registry) else {
            return Ok(());
        };
        // a. Choose any such cell.
        let Some(i) = d.cells.iter().position(|c| c.target.is_none()) else {
            return Ok(());
        };
        // b. Remove cell from finalizationRegistry.[[Cells]].
        let cell = d.cells.remove(i);
        // c. Perform ? HostCallJobCallback(callback, undefined, « cell.[[HeldValue]] »).
        //    (The registry is the running job's data; the held value is the argument.)
        crate::ops::call_rooted(agent, callback, Value::Undefined, &[cell.held_value])?;
    }
}
