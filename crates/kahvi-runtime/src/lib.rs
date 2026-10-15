//! kahvi's runtime: a host for running ES modules from the file system, with
//! `console`, timers, `queueMicrotask`, `structuredClone`, `process.env` and an event loop (milestone M4).
//!
//! The entry file is a module (ESM only). The loop: evaluate the module, run
//! microtasks (promise jobs), then run due timers one at a time, each
//! followed by the microtasks it queued, until no timers are left. An
//! uncaught exception or an unhandled promise rejection ends the process.

pub mod clone;
pub mod console;
pub mod inspect;
pub mod loader;
pub mod package;
pub mod process;
pub mod timers;

use std::path::Path;
use std::sync::{Arc, Mutex};

use kahvi::{JsError, Runtime, Value};
use timers::Timers;

/// Exit status for an uncaught exception or unhandled rejection.
pub const EXIT_UNCAUGHT: u8 = 1;
/// Bad invocation or an unreadable entry file.
pub const EXIT_USAGE: u8 = 2;
/// The entry module's top-level await never settled (as in Node).
pub const EXIT_UNSETTLED: u8 = 13;

/// A runtime with the host's globals installed.
pub struct Host {
    pub rt: Runtime,
    timers: Arc<Mutex<Timers>>,
}

impl Host {
    pub fn new() -> Host {
        let mut rt = Runtime::new();
        let agent = rt.agent();
        agent.host_now = Some(now_ms);
        agent.host_load_module = Some(loader::load_module);
        agent.host_import_meta = Some(loader::import_meta);
        agent.gc_stress = kahvi_core::Agent::gc_stress_from_env();
        let timers = Arc::new(Mutex::new(Timers::new()));
        console::install(&mut rt);
        timers::install(&mut rt, &timers);
        clone::install(&mut rt);
        kahvi::encoding::install(&mut rt);
        process::install(&mut rt);
        let global = rt.global();
        rt.set_global("globalThis", Value::Object(global));
        Host { rt, timers }
    }

    /// Runs the module at `path` and then the event loop. Returns the exit status.
    pub fn run_main(&mut self, path: &Path) -> u8 {
        let key = match path.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("kahvi-run: cannot read {}: {e}", path.display());
                return EXIT_USAGE;
            }
        };
        let source = match std::fs::read(&key) {
            Ok(b) => String::from_utf8(b)
                .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned()),
            Err(e) => {
                eprintln!("kahvi-run: cannot read {}: {e}", path.display());
                return EXIT_USAGE;
            }
        };
        let evaluation = match self.rt.eval_module(&key.to_string_lossy(), &source) {
            Ok(p) => self.rt.persist(Value::Object(p)),
            Err(e) => return self.uncaught(&e),
        };
        let entry = match self.rt.persisted(&evaluation) {
            Value::Object(p) => p,
            _ => unreachable!(),
        };
        loop {
            if let Err(e) = self.rt.run_jobs() {
                return self.uncaught(&e);
            }
            // The entry module's own rejection is its uncaught exception.
            let mut unhandled = self.rt.take_unhandled_rejections();
            if let Some(Err(reason)) = self.rt.promise_result(entry) {
                return self.uncaught(&JsError::Throw(reason));
            }
            unhandled.retain(|&p| p != entry);
            if let Some(&p) = unhandled.first() {
                let reason = match self.rt.promise_result(p) {
                    Some(Err(r)) => r,
                    _ => Value::Undefined,
                };
                let msg = self.describe(reason);
                eprintln!("Uncaught (in promise) {msg}");
                return EXIT_UNCAUGHT;
            }
            let wait = self.timers.lock().unwrap().wait();
            let Some(wait) = wait else { break };
            if !wait.is_zero() {
                std::thread::sleep(wait);
            }
            let due = self.timers.lock().unwrap().take_due();
            if let Some((callback, args)) = due {
                let f = self.rt.persisted(&callback);
                let args: Vec<Value> = args.iter().map(|a| self.rt.persisted(a)).collect();
                if let Err(e) = self.rt.call(f, Value::Undefined, &args) {
                    return self.uncaught(&e);
                }
            }
        }
        if self.rt.promise_result(entry).is_none() {
            eprintln!("Warning: the entry module's top-level await never settled");
            return EXIT_UNSETTLED;
        }
        0
    }

    fn describe(&mut self, v: Value) -> String {
        self.rt.enter(|agent| inspect::format_arg(agent, v))
    }

    /// Reports an uncaught exception on stderr.
    fn uncaught(&mut self, e: &JsError) -> u8 {
        let msg = match e {
            JsError::Throw(v) => self.describe(*v),
            JsError::NotImplemented(_) => self.rt.describe_error(e),
        };
        eprintln!("Uncaught {msg}");
        EXIT_UNCAUGHT
    }
}

impl Default for Host {
    fn default() -> Self {
        Host::new()
    }
}

/// The host clock for Date.now and new Date().
pub fn now_ms() -> f64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_millis() as f64,
        Err(e) => -(e.duration().as_millis() as f64),
    }
}
