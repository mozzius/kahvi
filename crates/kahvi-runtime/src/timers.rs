//! `setTimeout`, `setInterval`, their `clear*` functions and
//! `queueMicrotask`. The event loop (`crate::run`) runs due timers.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use kahvi::{Persistent, Runtime, Value};
use kahvi_core::agent::Job;
use kahvi_core::prelude::*;

/// The largest delay (2^31 - 1 ms); longer ones become 1 ms, as in browsers and Node.
const MAX_DELAY: f64 = 2_147_483_647.0;

pub struct Timer {
    pub callback: Persistent,
    pub args: Vec<Persistent>,
    /// The interval of a `setInterval` timer.
    pub repeat: Option<u64>,
    due: (u64, u64),
}

/// Pending timers, ordered by due time (then by creation, so equal delays
/// run in the order they were set).
pub struct Timers {
    start: Instant,
    next_id: u32,
    seq: u64,
    queue: BTreeMap<(u64, u64), u32>,
    timers: HashMap<u32, Timer>,
}

impl Default for Timers {
    fn default() -> Self {
        Timers::new()
    }
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            start: Instant::now(),
            next_id: 1,
            seq: 0,
            queue: BTreeMap::new(),
            timers: HashMap::new(),
        }
    }

    fn now_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    fn schedule(&mut self, id: u32, delay: u64, mut timer: Timer) {
        self.seq += 1;
        timer.due = (self.now_ms() + delay, self.seq);
        self.queue.insert(timer.due, id);
        self.timers.insert(id, timer);
    }

    fn add(
        &mut self,
        callback: Persistent,
        args: Vec<Persistent>,
        delay: u64,
        repeat: bool,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let timer = Timer {
            callback,
            args,
            repeat: repeat.then_some(delay),
            due: (0, 0),
        };
        self.schedule(id, delay, timer);
        id
    }

    fn clear(&mut self, id: u32) {
        if let Some(t) = self.timers.remove(&id) {
            self.queue.remove(&t.due);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.timers.is_empty()
    }

    /// How long until the next timer is due (`None` without timers).
    pub fn wait(&self) -> Option<Duration> {
        let (&(due, _), _) = self.queue.first_key_value()?;
        Some(Duration::from_millis(due.saturating_sub(self.now_ms())))
    }

    /// Removes the next due timer and returns its callback and arguments; a
    /// repeating one is scheduled again.
    pub fn take_due(&mut self) -> Option<(Persistent, Vec<Persistent>)> {
        let (&key, &id) = self.queue.first_key_value()?;
        if key.0 > self.now_ms() {
            return None;
        }
        self.queue.remove(&key);
        let timer = self.timers.remove(&id)?;
        let held = (timer.callback.clone(), timer.args.clone());
        if let Some(interval) = timer.repeat {
            self.schedule(id, interval, timer);
        }
        Some(held)
    }
}

/// A timer delay: ToNumber, then whole milliseconds of at least 1.
fn delay_ms(agent: &mut Agent, v: Value) -> JsResult<u64> {
    let d = if v.is_undefined() {
        0.0
    } else {
        ops::to_number(agent, v)?
    };
    Ok(if d.is_nan() || !(1.0..=MAX_DELAY).contains(&d) {
        1
    } else {
        d.trunc() as u64
    })
}

fn set_timer(
    timers: &Arc<Mutex<Timers>>,
    agent: &mut Agent,
    args: &[Value],
    repeat: bool,
    name: &str,
) -> JsResult<Value> {
    let callback = args.get_or_undefined(0);
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error(&format!("{name}: the callback must be a function")));
    }
    let delay = delay_ms(agent, args.get_or_undefined(1))?;
    let callback = agent.persist(callback);
    let extra = args
        .get(2..)
        .unwrap_or(&[])
        .iter()
        .map(|&v| agent.persist(v))
        .collect();
    let id = timers.lock().unwrap().add(callback, extra, delay, repeat);
    Ok(Value::Number(id as f64))
}

fn clear_timer(timers: &Arc<Mutex<Timers>>, args: &[Value]) -> JsResult<Value> {
    if let Value::Number(n) = args.get_or_undefined(0)
        && n.fract() == 0.0
        && (1.0..=u32::MAX as f64).contains(&n)
    {
        timers.lock().unwrap().clear(n as u32);
    }
    Ok(Value::Undefined)
}

pub fn install(rt: &mut Runtime, timers: &Arc<Mutex<Timers>>) {
    let t = timers.clone();
    rt.set_global_function("setTimeout", 2, move |agent, _, args| {
        set_timer(&t, agent, args, false, "setTimeout")
    });
    let t = timers.clone();
    rt.set_global_function("setInterval", 2, move |agent, _, args| {
        set_timer(&t, agent, args, true, "setInterval")
    });
    let t = timers.clone();
    rt.set_global_function("clearTimeout", 1, move |_, _, args| clear_timer(&t, args));
    let t = timers.clone();
    rt.set_global_function("clearInterval", 1, move |_, _, args| clear_timer(&t, args));
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
