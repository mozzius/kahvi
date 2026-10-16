//! kahvi compiled to WebAssembly for the playground page.
//!
//! The page runs each program in a fresh Web Worker with a fresh instance of
//! this module, so one program cannot see another and a runaway one is
//! stopped by terminating its worker. The browser is the event loop:
//! `setTimeout` and friends ask the worker for a browser timer, and the
//! worker calls `kahvi_fire` when it is due.
//!
//! Exports: `kahvi_alloc(len)` (a buffer for the source), `kahvi_run(ptr,
//! len)` (evaluate it as a module), `kahvi_fire(id)` (run timer `id`).
//! Imports (module "host"): `write(stream, ptr, len)` for console output
//! (0 stdout, 1 stderr), `now()` for the clock, `set_timer(id, ms, repeat)` and
//! `clear_timer(id)`.

use std::cell::RefCell;
use std::collections::HashMap;

use kahvi::{JsError, Object, Persistent, Runtime, Value};
use kahvi_core::agent::Job;
use kahvi_core::prelude::*;
use kahvi_runtime::inspect::format_arg;

mod host {
    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        pub fn write(stream: u32, ptr: *const u8, len: usize);
        pub fn now() -> f64;
        pub fn set_timer(id: u32, ms: f64, repeat: u32);
        pub fn clear_timer(id: u32);
    }
}

struct Timer {
    callback: Persistent,
    args: Vec<Persistent>,
    repeat: bool,
}

thread_local! {
    static RUNTIME: RefCell<Option<Runtime>> = const { RefCell::new(None) };
    static TIMERS: RefCell<(u32, HashMap<u32, Timer>)> = RefCell::new((0, HashMap::new()));
}

fn write(stream: u32, text: &str) {
    unsafe { host::write(stream, text.as_ptr(), text.len()) }
}

fn now_ms() -> f64 {
    unsafe { host::now() }
}

fn new_runtime() -> Runtime {
    let mut rt = Runtime::new();
    rt.agent().host_now = Some(now_ms);
    install_console(&mut rt);
    install_timers(&mut rt);
    kahvi::encoding::install(&mut rt);
    let global = rt.global();
    rt.set_global("globalThis", Value::Object(global));
    rt
}

fn install_console(rt: &mut Runtime) {
    let console = rt.enter(|agent| {
        let proto = agent.intrinsic("%Object.prototype%");
        agent.ordinary_object_create(Some(proto))
    });
    let methods: [(&str, u32); 6] = [
        ("log", 0),
        ("info", 0),
        ("debug", 0),
        ("warn", 1),
        ("error", 1),
        ("trace", 1),
    ];
    for (name, stream) in methods {
        let f = rt.function(name, 0, move |agent, _, args| {
            let parts: Vec<String> = args.iter().map(|&a| format_arg(agent, a)).collect();
            write(stream, &(parts.join(" ") + "\n"));
            Ok(Value::Undefined)
        });
        kahvi::define_hidden(rt.agent(), console, name, Value::Object(f));
    }
    rt.set_global("console", Value::Object(console));
}

fn install_timers(rt: &mut Runtime) {
    for (name, repeat) in [("setTimeout", false), ("setInterval", true)] {
        rt.set_global_function(name, 1, move |agent, _, args| {
            let callback = args.get_or_undefined(0);
            if !ops::is_callable(agent, callback) {
                return Err(
                    agent.throw_type_error(&format!("{name}: the callback must be a function"))
                );
            }
            let delay = ops::to_number(agent, args.get_or_undefined(1))?;
            let delay = if delay.is_finite() && delay > 0.0 {
                delay
            } else {
                0.0
            };
            let callback = agent.persist(callback);
            let args = args.iter().skip(2).map(|&a| agent.persist(a)).collect();
            let id = TIMERS.with_borrow_mut(|(next, timers)| {
                *next += 1;
                timers.insert(
                    *next,
                    Timer {
                        callback,
                        args,
                        repeat,
                    },
                );
                *next
            });
            unsafe { host::set_timer(id, delay, u32::from(repeat)) };
            Ok(Value::Number(f64::from(id)))
        });
    }
    for name in ["clearTimeout", "clearInterval"] {
        rt.set_global_function(name, 1, |_, _, args| {
            if let Value::Number(n) = args.get_or_undefined(0)
                && n.fract() == 0.0
                && (1.0..=f64::from(u32::MAX)).contains(&n)
            {
                let id = n as u32;
                if TIMERS
                    .with_borrow_mut(|(_, timers)| timers.remove(&id))
                    .is_some()
                {
                    unsafe { host::clear_timer(id) };
                }
            }
            Ok(Value::Undefined)
        });
    }
    rt.set_global_function("queueMicrotask", 1, |agent, _, args| {
        let callback = args.get_or_undefined(0);
        if !ops::is_callable(agent, callback) {
            return Err(agent.throw_type_error("queueMicrotask: the callback must be a function"));
        }
        let realm = agent.current_realm();
        agent.enqueue_job(Job::Callback { realm, callback });
        Ok(Value::Undefined)
    });
}

/// Reports an uncaught exception the way kahvi-run does.
fn report(rt: &mut Runtime, e: &JsError) {
    let msg = match e {
        JsError::Throw(v) => {
            let v = *v;
            rt.enter(|agent| format_arg(agent, v))
        }
        JsError::NotImplemented(_) => rt.describe_error(e),
    };
    write(1, &format!("Uncaught {msg}\n"));
}

/// Runs queued promise jobs and reports anything left unhandled. Returns
/// false once the program has failed.
fn settle(rt: &mut Runtime, entry: Option<Object>) -> bool {
    if let Err(e) = rt.run_jobs() {
        report(rt, &e);
        return false;
    }
    let mut unhandled = rt.take_unhandled_rejections();
    // The module's own rejection is its uncaught exception.
    if let Some(entry) = entry {
        if let Some(Err(reason)) = rt.promise_result(entry) {
            report(rt, &JsError::Throw(reason));
            return false;
        }
        unhandled.retain(|&p| p != entry);
    }
    if let Some(&p) = unhandled.first() {
        let reason = match rt.promise_result(p) {
            Some(Err(r)) => r,
            _ => Value::Undefined,
        };
        let msg = rt.enter(|agent| format_arg(agent, reason));
        write(1, &format!("Uncaught (in promise) {msg}\n"));
        return false;
    }
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn kahvi_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Evaluates the program as the module `main.mjs`. Returns 1 when it ran
/// without an uncaught error so far (timers may still be pending), else 0.
///
/// # Safety
///
/// `ptr` and `len` must come from one `kahvi_alloc(len)`, filled with the
/// source; this takes ownership of that buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_run(ptr: *mut u8, len: usize) -> u32 {
    let bytes = unsafe { Vec::from_raw_parts(ptr, len, len) };
    let source = String::from_utf8_lossy(&bytes).into_owned();
    RUNTIME.with_borrow_mut(|slot| {
        let rt = slot.insert(new_runtime());
        let entry = match rt.eval_module("main.mjs", &source) {
            Ok(p) => p,
            Err(e) => {
                report(rt, &e);
                return 0;
            }
        };
        u32::from(settle(rt, Some(entry)))
    })
}

/// Runs timer `id`. Returns 1 to keep going, 0 once the program has failed.
#[unsafe(no_mangle)]
pub extern "C" fn kahvi_fire(id: u32) -> u32 {
    RUNTIME.with_borrow_mut(|slot| {
        let Some(rt) = slot.as_mut() else { return 0 };
        let timer = TIMERS.with_borrow_mut(|(_, timers)| {
            let t = timers.get(&id)?;
            let callback = rt.persisted(&t.callback);
            let args: Vec<Value> = t.args.iter().map(|a| rt.persisted(a)).collect();
            if !t.repeat {
                timers.remove(&id);
            }
            Some((callback, args))
        });
        let Some((callback, args)) = timer else {
            return 1;
        };
        if let Err(e) = rt.call(callback, Value::Undefined, &args) {
            report(rt, &e);
            return 0;
        }
        u32::from(settle(rt, None))
    })
}
