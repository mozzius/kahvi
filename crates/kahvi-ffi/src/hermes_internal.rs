//! A `HermesInternal` global with the parts React Native uses, so that it
//! keeps the engine's own promises instead of swapping in its JS polyfill:
//! `hasPromise()`, `enablePromiseRejectionTracker(options)` and
//! `getRuntimeProperties()`.
//!
//! Rejection tracking follows the `promise` package's rejection-tracking
//! module that Hermes's own implementation mirrors: `options.onUnhandled(id,
//! reason)` for a promise rejected with no handler, `options.onHandled(id,
//! reason)` when such a promise gets one later, and without
//! `options.allRejections` only ReferenceErrors and RangeErrors are
//! reported. kahvi reports at each microtask checkpoint
//! (`kahvi_run_jobs`) rather than after Hermes's delay.

use std::sync::{Arc, Mutex};

use kahvi::{Agent, JsResult, Object, Persistent, Value};
use kahvi_core::object::PropertyKey;
use kahvi_core::ops;

/// What `enablePromiseRejectionTracker` asked for, and the rejections
/// reported so far that may still be handled.
#[derive(Default)]
pub struct RejectionTracking {
    options: Option<TrackerOptions>,
    reported: Vec<(Persistent, u32)>,
    next_id: u32,
}

struct TrackerOptions {
    all_rejections: bool,
    on_unhandled: Option<Persistent>,
    on_handled: Option<Persistent>,
}

pub fn install(agent: &mut Agent, global: Object, tracking: &Arc<Mutex<RejectionTracking>>) {
    let object_proto = agent.intrinsic("%Object.prototype%");
    let hermes = agent.ordinary_object_create(Some(object_proto));
    let has_promise = agent.host_function("hasPromise", 0, |_, _, _| Ok(Value::Boolean(true)));
    define(agent, hermes, "hasPromise", Value::Object(has_promise));
    let t = tracking.clone();
    let enable = agent.host_function("enablePromiseRejectionTracker", 1, move |agent, _, args| {
        let options = match args.first() {
            Some(&Value::Object(o)) => {
                let all = get(agent, o, "allRejections")?;
                let all_rejections = ops::to_boolean(agent, all);
                let on_unhandled = callback(agent, o, "onUnhandled")?;
                let on_handled = callback(agent, o, "onHandled")?;
                TrackerOptions {
                    all_rejections,
                    on_unhandled,
                    on_handled,
                }
            }
            _ => TrackerOptions {
                all_rejections: false,
                on_unhandled: None,
                on_handled: None,
            },
        };
        t.lock().unwrap().options = Some(options);
        Ok(Value::Undefined)
    });
    define(
        agent,
        hermes,
        "enablePromiseRejectionTracker",
        Value::Object(enable),
    );
    let properties = agent.host_function("getRuntimeProperties", 0, |agent, _, _| {
        let proto = agent.intrinsic("%Object.prototype%");
        let o = agent.ordinary_object_create(Some(proto));
        let version = agent.string(concat!("kahvi ", env!("CARGO_PKG_VERSION")));
        define(agent, o, "OSS Release Version", Value::String(version));
        let build = agent.string(if cfg!(debug_assertions) {
            "Debug"
        } else {
            "Release"
        });
        define(agent, o, "Build", Value::String(build));
        Ok(Value::Object(o))
    });
    define(
        agent,
        hermes,
        "getRuntimeProperties",
        Value::Object(properties),
    );
    kahvi::define_hidden(agent, global, "HermesInternal", Value::Object(hermes));
}

/// Reports the rejections tracked since the last call to the callbacks
/// `enablePromiseRejectionTracker` was given (dropping them when it was not
/// called). Returns whether any callback ran, as it may have queued jobs.
pub fn report_rejections(agent: &mut Agent, tracking: &Mutex<RejectionTracking>) -> JsResult<bool> {
    let events = std::mem::take(&mut agent.rejection_tracker);
    let mut calls: Vec<(Value, u32, Value)> = Vec::new();
    {
        let mut t = tracking.lock().unwrap();
        let Some(options) = &t.options else {
            return Ok(false);
        };
        let all_rejections = options.all_rejections;
        let on_unhandled = options.on_unhandled.as_ref().map(|p| agent.persisted(p));
        let on_handled = options.on_handled.as_ref().map(|p| agent.persisted(p));
        // Rejected with no handler, unless one was added in the same checkpoint.
        let mut unhandled: Vec<Object> = Vec::new();
        for (p, handled) in events {
            if !handled {
                if !unhandled.contains(&p) {
                    unhandled.push(p);
                }
                continue;
            }
            if let Some(i) = unhandled.iter().position(|&q| q == p) {
                unhandled.remove(i);
            } else if let Some(i) = t
                .reported
                .iter()
                .position(|(r, _)| matches!(agent.persisted(r), Value::Object(q) if q == p))
            {
                let (_, id) = t.reported.remove(i);
                if let Some(f) = on_handled {
                    calls.push((f, id, rejection_reason(agent, p)));
                }
            }
        }
        for p in unhandled {
            let reason = rejection_reason(agent, p);
            if !all_rejections && !is_reported_by_default(agent, reason) {
                continue;
            }
            let id = t.next_id;
            t.next_id += 1;
            let persisted = agent.persist(Value::Object(p));
            t.reported.push((persisted, id));
            if let Some(f) = on_unhandled {
                calls.push((f, id, reason));
            }
        }
    }
    let called = !calls.is_empty();
    // A callback can collect garbage: keep the others' values alive.
    let mark = agent.gc_roots_len();
    for &(f, _, reason) in &calls {
        agent.push_root(f);
        agent.push_root(reason);
    }
    let mut result = Ok(called);
    for (f, id, reason) in calls {
        if let Err(e) = ops::call(
            agent,
            f,
            Value::Undefined,
            &[Value::Number(id.into()), reason],
        ) {
            result = Err(e);
            break;
        }
    }
    agent.pop_roots(mark);
    result
}

fn rejection_reason(agent: &Agent, p: Object) -> Value {
    match ops::promise::promise_result(agent, p) {
        Some(Err(reason)) => reason,
        _ => Value::Undefined,
    }
}

/// The rejection-tracking module's default whitelist: ReferenceError and
/// RangeError instances.
fn is_reported_by_default(agent: &Agent, reason: Value) -> bool {
    let Value::Object(o) = reason else {
        return false;
    };
    let protos = [
        agent.intrinsic("%ReferenceError.prototype%"),
        agent.intrinsic("%RangeError.prototype%"),
    ];
    let mut cur = agent.obj(o).prototype;
    while let Some(p) = cur {
        if protos.contains(&p) {
            return true;
        }
        cur = agent.obj(p).prototype;
    }
    false
}

fn get(agent: &mut Agent, o: Object, name: &str) -> JsResult<Value> {
    let key = agent.key(name);
    ops::get(agent, o, key)
}

/// `options[name]` when it is a function.
fn callback(agent: &mut Agent, o: Object, name: &str) -> JsResult<Option<Persistent>> {
    let v = get(agent, o, name)?;
    Ok(ops::is_callable(agent, v).then(|| agent.persist(v)))
}

fn define(agent: &mut Agent, o: Object, name: &str, value: Value) {
    let key: PropertyKey = agent.key(name);
    ops::create_data_property_or_throw(agent, o, key, value).expect("a new ordinary object");
}
