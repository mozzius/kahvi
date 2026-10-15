//! kahvi's C ABI (`include/kahvi.h`), for hosts written in C or C++: the
//! React Native JSI runtime (`react-native/`) is built on it.
//!
//! Values cross the boundary as `KahviValue`: primitives inline, strings,
//! symbols, bigints and objects as a `KahviHandle` that keeps the value alive
//! until `kahvi_value_release`. Every `KahviValue` the API returns is owned
//! by the caller; values passed in are borrowed.
//!
//! Functions that can throw return `false` and leave the exception for
//! `kahvi_take_exception`. Host callbacks may call back into the API with
//! the `KahviRuntime*` they are given.
#![allow(clippy::missing_safety_doc)] // the safety contract is in kahvi.h

use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::sync::Arc;

use kahvi::{Agent, JsError, JsResult, Object, Persistent, Runtime, Value};
use kahvi_core::agent::{ErrorKind, ExecutionContext};
use kahvi_core::object::host_object::{HostObject, create_host_object, host_of};
use kahvi_core::object::{ObjectKind, PropertyKey};
use kahvi_core::ops;
use kahvi_core::ops::bigint::BigIntValue;

/// One runtime. Its agent is reached through `agent`: the runtime's own
/// agent at the top level, or (while a host callback runs) the reference
/// the engine gave the callback, so a callback's calls back into the API
/// reborrow it instead of aliasing it.
pub struct KahviRuntime {
    /// Owns the agent; reached only through `agent` once created.
    _runtime: Box<Runtime>,
    realm: kahvi::RealmId,
    agent: Cell<*mut Agent>,
    exception: RefCell<Option<Persistent>>,
    /// The context of each `kahvi_function_new` function, by its index
    /// (`Agent::host_function_index`).
    function_contexts: RefCell<std::collections::HashMap<usize, usize>>,
}

/// A reference to a string, symbol, bigint or object.
pub struct KahviHandle(Persistent);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KahviTag {
    Undefined = 0,
    Null = 1,
    Boolean = 2,
    Number = 3,
    String = 4,
    Symbol = 5,
    BigInt = 6,
    Object = 7,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KahviValue {
    pub tag: KahviTag,
    pub boolean: bool,
    pub number: f64,
    pub handle: *mut KahviHandle,
}

const UNDEFINED: KahviValue = KahviValue {
    tag: KahviTag::Undefined,
    boolean: false,
    number: 0.0,
    handle: std::ptr::null_mut(),
};

pub type KahviHostFunction = unsafe extern "C" fn(
    rt: *mut KahviRuntime,
    context: *mut c_void,
    this: KahviValue,
    args: *const KahviValue,
    argc: usize,
    result: *mut KahviValue,
) -> bool;

pub type KahviFinalizer = Option<unsafe extern "C" fn(context: *mut c_void)>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KahviHostObjectClass {
    pub get: unsafe extern "C" fn(
        rt: *mut KahviRuntime,
        context: *mut c_void,
        name: KahviValue,
        result: *mut KahviValue,
    ) -> bool,
    pub set: unsafe extern "C" fn(
        rt: *mut KahviRuntime,
        context: *mut c_void,
        name: KahviValue,
        value: KahviValue,
        exception: *mut KahviValue,
    ) -> bool,
    pub property_names: unsafe extern "C" fn(
        rt: *mut KahviRuntime,
        context: *mut c_void,
        result: *mut KahviValue,
    ) -> bool,
    pub finalize: KahviFinalizer,
}

// ---- plumbing ---------------------------------------------------------------------------

unsafe fn agent<'a>(rt: *mut KahviRuntime) -> &'a mut Agent {
    unsafe { &mut *(*rt).agent.get() }
}

/// Runs `f` with the agent, inside an execution context for the runtime's
/// realm when no JavaScript is running.
unsafe fn with_agent<T>(rt: *mut KahviRuntime, f: impl FnOnce(&mut Agent) -> T) -> T {
    let agent = unsafe { agent(rt) };
    let realm = unsafe { (*rt).realm };
    let top = agent.contexts.is_empty();
    if top {
        agent.contexts.push(ExecutionContext {
            function: None,
            realm,
            script_or_module: None,
        });
    }
    let r = f(agent);
    if top {
        agent.contexts.pop();
    }
    r
}

/// Records an exception for `kahvi_take_exception`; returns false.
unsafe fn fail(rt: *mut KahviRuntime, agent: &mut Agent, e: JsError) -> bool {
    let v = thrown_value(agent, e);
    let p = agent.persist(v);
    unsafe { *(*rt).exception.borrow_mut() = Some(p) };
    false
}

fn thrown_value(agent: &mut Agent, e: JsError) -> Value {
    match e {
        JsError::Throw(v) => v,
        JsError::NotImplemented(what) => {
            Value::Object(agent.create_error(ErrorKind::Error, &format!("not implemented: {what}")))
        }
    }
}

fn to_c(agent: &mut Agent, v: Value) -> KahviValue {
    let tag = match v {
        Value::Undefined => return UNDEFINED,
        Value::Null => KahviTag::Null,
        Value::Boolean(b) => {
            return KahviValue {
                tag: KahviTag::Boolean,
                boolean: b,
                ..UNDEFINED
            };
        }
        Value::Number(n) => {
            return KahviValue {
                tag: KahviTag::Number,
                number: n,
                ..UNDEFINED
            };
        }
        Value::String(_) => KahviTag::String,
        Value::Symbol(_) => KahviTag::Symbol,
        Value::BigInt(_) => KahviTag::BigInt,
        Value::Object(_) => KahviTag::Object,
    };
    if tag == KahviTag::Null {
        return KahviValue { tag, ..UNDEFINED };
    }
    let handle = Box::into_raw(Box::new(KahviHandle(agent.persist(v))));
    KahviValue {
        tag,
        handle,
        ..UNDEFINED
    }
}

unsafe fn from_c(agent: &Agent, v: &KahviValue) -> Value {
    match v.tag {
        KahviTag::Undefined => Value::Undefined,
        KahviTag::Null => Value::Null,
        KahviTag::Boolean => Value::Boolean(v.boolean),
        KahviTag::Number => Value::Number(v.number),
        _ => agent.persisted(unsafe { &(*v.handle).0 }),
    }
}

unsafe fn object_arg(agent: &mut Agent, v: &KahviValue) -> JsResult<Object> {
    match unsafe { from_c(agent, v) } {
        Value::Object(o) => Ok(o),
        _ => Err(agent.throw_type_error("not an object")),
    }
}

unsafe fn write(out: *mut KahviValue, v: KahviValue) {
    if !out.is_null() {
        unsafe { *out = v };
    }
}

/// Runs an operation that may throw, writing its value to `out`.
unsafe fn op(
    rt: *mut KahviRuntime,
    out: *mut KahviValue,
    f: impl FnOnce(&mut Agent) -> JsResult<Value>,
) -> bool {
    unsafe {
        with_agent(rt, |agent| match f(agent) {
            Ok(v) => {
                let c = to_c(agent, v);
                write(out, c);
                true
            }
            Err(e) => fail(rt, agent, e),
        })
    }
}

/// Calls into C from a host callback: the API's agent becomes the one the
/// engine passed in for the duration.
unsafe fn enter_callback<T>(
    rt: *mut KahviRuntime,
    agent: &mut Agent,
    f: impl FnOnce(*mut Agent) -> T,
) -> T {
    let ptr: *mut Agent = agent;
    let prev = unsafe { (*rt).agent.replace(ptr) };
    let r = f(ptr);
    unsafe { (*rt).agent.set(prev) };
    r
}

// ---- runtime ----------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn kahvi_runtime_new() -> *mut KahviRuntime {
    let mut runtime = Box::new(Runtime::new());
    // Web APIs Hermes provides and React Native code expects.
    kahvi::encoding::install(&mut runtime);
    let realm = runtime.realm();
    let agent: *mut Agent = runtime.agent();
    unsafe { (*agent).host_now = Some(now_ms) };
    Box::into_raw(Box::new(KahviRuntime {
        _runtime: runtime,
        realm,
        agent: Cell::new(agent),
        exception: RefCell::new(None),
        function_contexts: RefCell::new(Default::default()),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_runtime_free(rt: *mut KahviRuntime) {
    if !rt.is_null() {
        drop(unsafe { Box::from_raw(rt) });
    }
}

/// The clock for Date.now: milliseconds since the epoch.
fn now_ms() -> f64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_millis() as f64,
        Err(e) => -(e.duration().as_millis() as f64),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_eval(
    rt: *mut KahviRuntime,
    source: *const u8,
    len: usize,
    name: *const u8,
    name_len: usize,
    out: *mut KahviValue,
) -> bool {
    let source =
        String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(source, len) }).into_owned();
    let name = if name.is_null() {
        String::from("<eval>")
    } else {
        String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(name, name_len) }).into_owned()
    };
    let realm = unsafe { (*rt).realm };
    unsafe {
        op(rt, out, |agent| {
            let script = kahvi_core::vm::parse_script(agent, &source, &name, realm)?;
            kahvi_core::vm::evaluate_script(agent, &script)
        })
    }
}

/// Runs queued jobs (promise reactions, microtasks) until there are none.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_run_jobs(rt: *mut KahviRuntime) -> bool {
    unsafe {
        op(rt, std::ptr::null_mut(), |agent| {
            agent.run_jobs().map(|_| Value::Undefined)
        })
    }
}

/// Queues a call of `f` with no arguments as a job (`queueMicrotask`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_queue_microtask(rt: *mut KahviRuntime, f: KahviValue) {
    unsafe {
        with_agent(rt, |agent| {
            let callback = from_c(agent, &f);
            let realm = agent.current_realm();
            agent.enqueue_job(kahvi_core::agent::Job::Callback { realm, callback });
        })
    }
}

/// The pending exception (undefined if none), which is cleared.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_take_exception(rt: *mut KahviRuntime) -> KahviValue {
    let p = unsafe { (*rt).exception.borrow_mut().take() };
    match p {
        Some(p) => unsafe {
            with_agent(rt, |agent| {
                let v = agent.persisted(&p);
                to_c(agent, v)
            })
        },
        None => UNDEFINED,
    }
}

/// Collects garbage now if no host frame holds unrooted values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_collect_garbage(rt: *mut KahviRuntime) {
    unsafe { agent(rt).collect_garbage_if_safe() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_global(rt: *mut KahviRuntime) -> KahviValue {
    let realm = unsafe { (*rt).realm };
    unsafe {
        with_agent(rt, |agent| {
            let g = agent.realm(realm).global_object;
            to_c(agent, Value::Object(g))
        })
    }
}

// ---- values -----------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_value_release(v: KahviValue) {
    if !v.handle.is_null() {
        drop(unsafe { Box::from_raw(v.handle) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_value_clone(v: KahviValue) -> KahviValue {
    if v.handle.is_null() {
        return v;
    }
    let h = unsafe { (*v.handle).0.clone() };
    KahviValue {
        handle: Box::into_raw(Box::new(KahviHandle(h))),
        ..v
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_strict_equals(
    rt: *mut KahviRuntime,
    a: KahviValue,
    b: KahviValue,
) -> bool {
    unsafe {
        with_agent(rt, |agent| {
            let (a, b) = (from_c(agent, &a), from_c(agent, &b));
            ops::is_strictly_equal(agent, a, b)
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_string_new(
    rt: *mut KahviRuntime,
    utf8: *const u8,
    len: usize,
) -> KahviValue {
    let s = String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(utf8, len) }).into_owned();
    unsafe {
        with_agent(rt, |agent| {
            let js = agent.string(&s);
            to_c(agent, Value::String(js))
        })
    }
}

/// The UTF-8 contents of a string (lone surrogates become U+FFFD), to be
/// freed with `kahvi_utf8_free`; NULL if `v` is not a string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_string_utf8(
    rt: *mut KahviRuntime,
    v: KahviValue,
    len: *mut usize,
) -> *mut u8 {
    let s = unsafe {
        with_agent(rt, |agent| match from_c(agent, &v) {
            Value::String(s) => Some(agent.str_lossy(s)),
            _ => None,
        })
    };
    let Some(s) = s else {
        return std::ptr::null_mut();
    };
    let bytes = s.into_bytes().into_boxed_slice();
    unsafe { *len = bytes.len() };
    Box::into_raw(bytes) as *mut u8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_utf8_free(p: *mut u8, len: usize) {
    if !p.is_null() {
        drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(p, len)) });
    }
}

/// ToString(v) (may run JavaScript).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_to_string(
    rt: *mut KahviRuntime,
    v: KahviValue,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            let v = from_c(agent, &v);
            ops::to_string(agent, v).map(Value::String)
        })
    }
}

/// SymbolDescriptiveString: "Symbol(description)".
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_symbol_to_string(
    rt: *mut KahviRuntime,
    v: KahviValue,
) -> KahviValue {
    unsafe {
        with_agent(rt, |agent| {
            let Value::Symbol(s) = from_c(agent, &v) else {
                return UNDEFINED;
            };
            let d = agent
                .symbol_description(s)
                .map(|d| agent.str_lossy(d))
                .unwrap_or_default();
            let js = agent.string(&format!("Symbol({d})"));
            to_c(agent, Value::String(js))
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_bigint_from_i64(rt: *mut KahviRuntime, v: i64) -> KahviValue {
    unsafe {
        with_agent(rt, |agent| {
            let b = ops::bigint::from_i128(agent, v as i128);
            to_c(agent, Value::BigInt(b))
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_bigint_from_u64(rt: *mut KahviRuntime, v: u64) -> KahviValue {
    unsafe {
        with_agent(rt, |agent| {
            let b = ops::bigint::from_i128(agent, v as i128);
            to_c(agent, Value::BigInt(b))
        })
    }
}

/// The low 64 bits of a bigint (two's complement). `fits_i64`/`fits_u64`
/// say whether the value is exactly representable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_bigint_truncate(
    rt: *mut KahviRuntime,
    v: KahviValue,
    fits_i64: *mut bool,
    fits_u64: *mut bool,
) -> u64 {
    unsafe {
        with_agent(rt, |agent| {
            let Value::BigInt(b) = from_c(agent, &v) else {
                return 0;
            };
            let value: &BigIntValue = agent.bigint(b);
            let bits = value.bit_length();
            let low = value.low_u64();
            let neg = value.negative;
            if !fits_i64.is_null() {
                *fits_i64 = bits < 64 || (neg && bits == 64 && low == 1 << 63);
            }
            if !fits_u64.is_null() {
                *fits_u64 = !neg && bits <= 64;
            }
            low
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_bigint_to_string(
    rt: *mut KahviRuntime,
    v: KahviValue,
    radix: u32,
) -> KahviValue {
    unsafe {
        with_agent(rt, |agent| {
            let Value::BigInt(b) = from_c(agent, &v) else {
                return UNDEFINED;
            };
            let s = ops::bigint::to_js_string(agent, b, radix.clamp(2, 36));
            to_c(agent, Value::String(s))
        })
    }
}

// ---- objects ----------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_object_new(rt: *mut KahviRuntime) -> KahviValue {
    unsafe {
        with_agent(rt, |agent| {
            let proto = agent.intrinsic("%Object.prototype%");
            let o = agent.ordinary_object_create(Some(proto));
            to_c(agent, Value::Object(o))
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_array_new(
    rt: *mut KahviRuntime,
    len: usize,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            kahvi_core::object::array::array_create(agent, len as f64, None).map(Value::Object)
        })
    }
}

/// `obj[key]`; `key` is converted with ToPropertyKey.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_get(
    rt: *mut KahviRuntime,
    obj: KahviValue,
    key: KahviValue,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            let o = object_arg(agent, &obj)?;
            let k = from_c(agent, &key);
            let k = ops::to_property_key(agent, k)?;
            ops::get(agent, o, k)
        })
    }
}

/// `obj[key] = value` (throws on failure, as in strict code).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_set(
    rt: *mut KahviRuntime,
    obj: KahviValue,
    key: KahviValue,
    value: KahviValue,
) -> bool {
    unsafe {
        op(rt, std::ptr::null_mut(), |agent| {
            let o = object_arg(agent, &obj)?;
            let k = from_c(agent, &key);
            let k = ops::to_property_key(agent, k)?;
            let v = from_c(agent, &value);
            ops::set(agent, o, k, v, true).map(|_| Value::Undefined)
        })
    }
}

/// `key in obj`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_has(
    rt: *mut KahviRuntime,
    obj: KahviValue,
    key: KahviValue,
    out: *mut bool,
) -> bool {
    unsafe {
        with_agent(rt, |agent| {
            let r = (|| {
                let o = object_arg(agent, &obj)?;
                let k = from_c(agent, &key);
                let k = ops::to_property_key(agent, k)?;
                o.internal_has_property(agent, k)
            })();
            match r {
                Ok(b) => {
                    *out = b;
                    true
                }
                Err(e) => fail(rt, agent, e),
            }
        })
    }
}

/// An array of the enumerable string-keyed property names of `obj` and its
/// prototype chain (the names a for-in loop visits).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_property_names(
    rt: *mut KahviRuntime,
    obj: KahviValue,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            let mut o = Some(object_arg(agent, &obj)?);
            let mut names: Vec<PropertyKey> = Vec::new();
            let mut seen: Vec<PropertyKey> = Vec::new();
            while let Some(cur) = o {
                for k in cur.internal_own_property_keys(agent)? {
                    if matches!(k, PropertyKey::Symbol(_)) || seen.contains(&k) {
                        continue;
                    }
                    seen.push(k);
                    if let Some(d) = cur.internal_get_own_property(agent, k)?
                        && d.enumerable == Some(true)
                    {
                        names.push(k);
                    }
                }
                o = cur.internal_get_prototype_of(agent)?;
            }
            let values: Vec<Value> = names
                .into_iter()
                .map(|k| match k {
                    PropertyKey::String(s) => Value::String(s),
                    PropertyKey::Index(i) => Value::String(agent.string(&i.to_string())),
                    PropertyKey::Symbol(s) => Value::Symbol(s),
                })
                .collect();
            Ok(Value::Object(ops::create_array_from_list(agent, &values)))
        })
    }
}

/// Call(f, this, args).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_call(
    rt: *mut KahviRuntime,
    f: KahviValue,
    this: KahviValue,
    args: *const KahviValue,
    argc: usize,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            let f = from_c(agent, &f);
            let this = from_c(agent, &this);
            let args = collect_args(agent, args, argc);
            ops::call_rooted(agent, f, this, &args)
        })
    }
}

/// Construct(f, args).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_construct(
    rt: *mut KahviRuntime,
    f: KahviValue,
    args: *const KahviValue,
    argc: usize,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            let fv = from_c(agent, &f);
            if !ops::is_constructor(agent, fv) {
                return Err(agent.throw_type_error("not a constructor"));
            }
            let Value::Object(fo) = fv else {
                unreachable!()
            };
            let args = collect_args(agent, args, argc);
            ops::construct(agent, fo, &args, None).map(Value::Object)
        })
    }
}

/// The arguments as values. They stay rooted by their handles.
unsafe fn collect_args(agent: &Agent, args: *const KahviValue, argc: usize) -> Vec<Value> {
    if argc == 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(args, argc) }
        .iter()
        .map(|a| unsafe { from_c(agent, a) })
        .collect()
}

/// `v instanceof ctor`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_instance_of(
    rt: *mut KahviRuntime,
    v: KahviValue,
    ctor: KahviValue,
    out: *mut bool,
) -> bool {
    unsafe {
        with_agent(rt, |agent| {
            let (v, c) = (from_c(agent, &v), from_c(agent, &ctor));
            match ops::instance_of_operator(agent, v, c) {
                Ok(b) => {
                    *out = b;
                    true
                }
                Err(e) => fail(rt, agent, e),
            }
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_is_array(rt: *mut KahviRuntime, v: KahviValue) -> bool {
    unsafe {
        with_agent(rt, |agent| match from_c(agent, &v) {
            Value::Object(o) => matches!(agent.obj(o).kind, ObjectKind::Array),
            _ => false,
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_is_function(rt: *mut KahviRuntime, v: KahviValue) -> bool {
    unsafe {
        with_agent(rt, |agent| {
            let v = from_c(agent, &v);
            ops::is_callable(agent, v)
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_is_array_buffer(rt: *mut KahviRuntime, v: KahviValue) -> bool {
    unsafe {
        with_agent(rt, |agent| match from_c(agent, &v) {
            Value::Object(o) => matches!(agent.obj(o).kind, ObjectKind::ArrayBuffer(_)),
            _ => false,
        })
    }
}

/// ToLength(Get(v, "length")).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_length(
    rt: *mut KahviRuntime,
    v: KahviValue,
    out: *mut usize,
) -> bool {
    unsafe {
        with_agent(rt, |agent| {
            let r = (|| {
                let o = object_arg(agent, &v)?;
                ops::length_of_array_like(agent, o)
            })();
            match r {
                Ok(n) => {
                    *out = n as usize;
                    true
                }
                Err(e) => fail(rt, agent, e),
            }
        })
    }
}

/// A new ArrayBuffer of `len` zero bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_array_buffer_new(
    rt: *mut KahviRuntime,
    len: usize,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            let ctor = agent.intrinsic("%ArrayBuffer%");
            kahvi_core::object::typed_array::allocate_array_buffer(agent, ctor, len as f64, None)
                .map(Value::Object)
        })
    }
}

/// An ArrayBuffer's bytes (valid until the buffer is detached, resized or
/// collected); NULL if `v` is not an attached ArrayBuffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_array_buffer_data(
    rt: *mut KahviRuntime,
    v: KahviValue,
    len: *mut usize,
) -> *mut u8 {
    unsafe {
        with_agent(rt, |agent| {
            let Value::Object(o) = from_c(agent, &v) else {
                return std::ptr::null_mut();
            };
            match &mut agent.obj_mut(o).kind {
                ObjectKind::ArrayBuffer(d) => match d.data.as_mut() {
                    Some(bytes) => {
                        *len = bytes.len();
                        bytes.as_mut_ptr()
                    }
                    None => std::ptr::null_mut(),
                },
                _ => std::ptr::null_mut(),
            }
        })
    }
}

/// `new WeakRef(obj)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_weak_new(
    rt: *mut KahviRuntime,
    obj: KahviValue,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            let target = from_c(agent, &obj);
            let ctor = agent.intrinsic("%WeakRef%");
            ops::construct(agent, ctor, &[target], None).map(Value::Object)
        })
    }
}

/// The target of a WeakRef made by `kahvi_weak_new`, or undefined.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_weak_lock(
    rt: *mut KahviRuntime,
    weak: KahviValue,
    out: *mut KahviValue,
) -> bool {
    unsafe {
        op(rt, out, |agent| {
            let w = from_c(agent, &weak);
            let proto = agent.intrinsic("%WeakRef.prototype%");
            let k = agent.key("deref");
            let deref = ops::get(agent, proto, k)?;
            ops::call(agent, deref, w, &[])
        })
    }
}

// ---- host functions and host objects ----------------------------------------------------

/// Calls `finalize(context)` when dropped.
struct Finalize {
    finalize: KahviFinalizer,
    context: usize,
}

impl Drop for Finalize {
    fn drop(&mut self) {
        if let Some(f) = self.finalize {
            unsafe { f(self.context as *mut c_void) };
        }
    }
}

/// A function whose behaviour is the C callback `f`. `finalize(context)`
/// runs when the runtime is freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_function_new(
    rt: *mut KahviRuntime,
    name: *const u8,
    name_len: usize,
    length: u32,
    f: KahviHostFunction,
    context: *mut c_void,
    finalize: KahviFinalizer,
) -> KahviValue {
    let name = if name.is_null() {
        String::new()
    } else {
        String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(name, name_len) }).into_owned()
    };
    let rt_addr = rt as usize;
    let guard = Arc::new(Finalize {
        finalize,
        context: context as usize,
    });
    let ctx = context as usize;
    unsafe {
        with_agent(rt, move |agent| {
            let behaviour =
                move |agent: &mut Agent, this: Value, args: &[Value]| -> JsResult<Value> {
                    let _keep = &guard;
                    let rt = rt_addr as *mut KahviRuntime;
                    enter_callback(rt, agent, |agent| {
                        let agent = &mut *agent;
                        let this_c = to_c(agent, this);
                        let args_c: Vec<KahviValue> =
                            args.iter().map(|&a| to_c(agent, a)).collect();
                        let mut out = UNDEFINED;
                        let ok = f(
                            rt,
                            ctx as *mut c_void,
                            this_c,
                            args_c.as_ptr(),
                            args_c.len(),
                            &mut out,
                        );
                        let agent = &mut *(*rt).agent.get();
                        let result = from_c(agent, &out);
                        kahvi_value_release(out);
                        kahvi_value_release(this_c);
                        for a in args_c {
                            kahvi_value_release(a);
                        }
                        if ok {
                            Ok(result)
                        } else {
                            Err(JsError::Throw(result))
                        }
                    })
                };
            let fo = agent.host_function(&name, length, behaviour);
            let index = agent
                .host_function_index(Value::Object(fo))
                .expect("a host function");
            (*rt).function_contexts.borrow_mut().insert(index, ctx);
            to_c(agent, Value::Object(fo))
        })
    }
}

struct FfiHostObject {
    rt: usize,
    class: KahviHostObjectClass,
    context: usize,
}

impl Drop for FfiHostObject {
    fn drop(&mut self) {
        if let Some(f) = self.class.finalize {
            unsafe { f(self.context as *mut c_void) };
        }
    }
}

fn key_value(agent: &mut Agent, key: PropertyKey) -> Value {
    match key {
        PropertyKey::String(s) => Value::String(s),
        PropertyKey::Symbol(s) => Value::Symbol(s),
        PropertyKey::Index(i) => Value::String(agent.string(&i.to_string())),
    }
}

impl HostObject for FfiHostObject {
    fn get(&self, agent: &mut Agent, _this: Object, key: PropertyKey) -> JsResult<Value> {
        let rt = self.rt as *mut KahviRuntime;
        unsafe {
            enter_callback(rt, agent, |agent| {
                let agent = &mut *agent;
                let name = key_value(agent, key);
                let name_c = to_c(agent, name);
                let mut out = UNDEFINED;
                let ok = (self.class.get)(rt, self.context as *mut c_void, name_c, &mut out);
                let agent = &mut *(*rt).agent.get();
                let v = from_c(agent, &out);
                kahvi_value_release(out);
                kahvi_value_release(name_c);
                if ok { Ok(v) } else { Err(JsError::Throw(v)) }
            })
        }
    }

    fn set(
        &self,
        agent: &mut Agent,
        _this: Object,
        key: PropertyKey,
        value: Value,
    ) -> JsResult<()> {
        let rt = self.rt as *mut KahviRuntime;
        unsafe {
            enter_callback(rt, agent, |agent| {
                let agent = &mut *agent;
                let name = key_value(agent, key);
                let name_c = to_c(agent, name);
                let value_c = to_c(agent, value);
                let mut exception = UNDEFINED;
                let ok = (self.class.set)(
                    rt,
                    self.context as *mut c_void,
                    name_c,
                    value_c,
                    &mut exception,
                );
                let agent = &mut *(*rt).agent.get();
                let e = from_c(agent, &exception);
                kahvi_value_release(exception);
                kahvi_value_release(value_c);
                kahvi_value_release(name_c);
                if ok { Ok(()) } else { Err(JsError::Throw(e)) }
            })
        }
    }

    fn property_names(&self, agent: &mut Agent, _this: Object) -> JsResult<Vec<PropertyKey>> {
        let rt = self.rt as *mut KahviRuntime;
        unsafe {
            enter_callback(rt, agent, |agent| {
                let mut out = UNDEFINED;
                let ok = (self.class.property_names)(rt, self.context as *mut c_void, &mut out);
                let agent = &mut *agent;
                let v = from_c(agent, &out);
                kahvi_value_release(out);
                if !ok {
                    return Err(JsError::Throw(v));
                }
                let Value::Object(list) = v else {
                    return Err(agent.throw_type_error("property_names must return an array"));
                };
                let n = ops::length_of_array_like(agent, list)? as u32;
                let mut keys = Vec::with_capacity(n as usize);
                for i in 0..n {
                    let item = ops::get(agent, list, PropertyKey::Index(i))?;
                    keys.push(ops::to_property_key(agent, item)?);
                }
                Ok(keys)
            })
        }
    }
}

/// A host object whose properties `class` implements; `finalize(context)`
/// runs when the object is collected (it must not call into the runtime).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_host_object_new(
    rt: *mut KahviRuntime,
    class: *const KahviHostObjectClass,
    context: *mut c_void,
) -> KahviValue {
    let host = Arc::new(FfiHostObject {
        rt: rt as usize,
        class: unsafe { *class },
        context: context as usize,
    });
    unsafe {
        with_agent(rt, |agent| {
            let o = create_host_object(agent, host);
            to_c(agent, Value::Object(o))
        })
    }
}

/// The context of a host object made by `kahvi_host_object_new`, else NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_host_object_context(
    rt: *mut KahviRuntime,
    v: KahviValue,
) -> *mut c_void {
    unsafe {
        with_agent(rt, |agent| {
            let Value::Object(o) = from_c(agent, &v) else {
                return std::ptr::null_mut();
            };
            let Some(host) = host_of(agent, o) else {
                return std::ptr::null_mut();
            };
            let any: &dyn std::any::Any = &*host;
            match any.downcast_ref::<FfiHostObject>() {
                Some(h) => h.context as *mut c_void,
                None => std::ptr::null_mut(),
            }
        })
    }
}

/// The context of a function made by `kahvi_function_new`, else NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_host_function_context(
    rt: *mut KahviRuntime,
    v: KahviValue,
) -> *mut c_void {
    unsafe {
        with_agent(rt, |agent| {
            let f = from_c(agent, &v);
            agent
                .host_function_index(f)
                .and_then(|i| (*rt).function_contexts.borrow().get(&i).copied())
                .map_or(std::ptr::null_mut(), |c| c as *mut c_void)
        })
    }
}

/// Attaches `context` to `object` (JSI's NativeState); `finalize(context)`
/// runs when it is replaced or the object is collected (it must not call
/// into the runtime).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_set_native_state(
    rt: *mut KahviRuntime,
    object: KahviValue,
    context: *mut c_void,
    finalize: KahviFinalizer,
) -> bool {
    unsafe {
        with_agent(rt, |agent| {
            let Value::Object(o) = from_c(agent, &object) else {
                return false;
            };
            let state = Arc::new(Finalize {
                finalize,
                context: context as usize,
            });
            agent.set_native_state(o, state);
            true
        })
    }
}

/// The context attached with `kahvi_set_native_state`, else NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kahvi_get_native_state(
    rt: *mut KahviRuntime,
    object: KahviValue,
) -> *mut c_void {
    unsafe {
        with_agent(rt, |agent| {
            let Value::Object(o) = from_c(agent, &object) else {
                return std::ptr::null_mut();
            };
            agent
                .native_state(o)
                .and_then(|s| {
                    s.downcast_ref::<Finalize>()
                        .map(|f| f.context as *mut c_void)
                })
                .unwrap_or(std::ptr::null_mut())
        })
    }
}
