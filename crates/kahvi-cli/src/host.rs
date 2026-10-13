//! The host: realm setup with `print` and `$262` (test262 INTERPRETING.md).

use kahvi_core::function::{builtin, builtin_slots, create_builtin_function};
use kahvi_core::prelude::*;
use kahvi_core::vm;

fn define(agent: &mut Agent, o: Object, name: &str, value: Value) {
    let k = agent.key(name);
    agent.obj_mut(o).properties.insert(
        k,
        Property::Data {
            value,
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
}

/// `print(value)`: ToString(value) and a newline on stdout.
fn print(agent: &mut Agent, _: Value, args: &[Value], _: Option<Object>) -> JsResult<Value> {
    let s = ops::to_string(agent, args.get_or_undefined(0))?;
    println!("{}", agent.str_lossy(s));
    Ok(Value::Undefined)
}

/// The realm a `$262` function belongs to (its first slot holds the realm's global object).
fn realm_of(agent: &mut Agent) -> RealmId {
    let f = agent.active_function().expect("active function");
    let slot = builtin_slots(agent, f)
        .first()
        .copied()
        .and_then(|v| v.as_number())
        .unwrap_or(0.0);
    RealmId(slot as u32)
}

/// `$262.createRealm()`
fn create_realm(agent: &mut Agent, _: Value, _: &[Value], _: Option<Object>) -> JsResult<Value> {
    let realm = new_realm(agent)?;
    let global = agent.realm(realm).global_object;
    let k = agent.key("$262");
    ops::get(agent, global, k)
}

/// `$262.evalScript(source)`
fn eval_script(agent: &mut Agent, _: Value, args: &[Value], _: Option<Object>) -> JsResult<Value> {
    let realm = realm_of(agent);
    let s = ops::to_string(agent, args.get_or_undefined(0))?;
    let source = agent.str_lossy(s);
    let script = vm::parse_script(agent, &source, "evalScript", realm)?;
    vm::evaluate_script(agent, &script)
}

fn gc(_: &mut Agent, _: Value, _: &[Value], _: Option<Object>) -> JsResult<Value> {
    Ok(Value::Undefined)
}

fn detach_array_buffer(
    agent: &mut Agent,
    _: Value,
    _: &[Value],
    _: Option<Object>,
) -> JsResult<Value> {
    Err(agent.not_implemented("$262.detachArrayBuffer"))
}

/// The host clock for Date.now and new Date().
pub fn now_ms() -> f64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_millis() as f64,
        Err(e) => -(e.duration().as_millis() as f64),
    }
}

/// A new realm with the built-ins and host bindings.
pub fn new_realm(agent: &mut Agent) -> JsResult<RealmId> {
    let realm = agent.create_realm()?;
    let global = agent.realm(realm).global_object;
    agent.contexts.push(kahvi_core::agent::ExecutionContext {
        function: None,
        realm,
        script_or_module: None,
    });
    let p = builtin(agent, print, 1, "print");
    define(agent, global, "print", Value::Object(p));
    let proto = agent.intrinsic("%Object.prototype%");
    let d = agent.ordinary_object_create(Some(proto));
    let slots = vec![Value::Number(realm.0 as f64)];
    for (name, f, len) in [
        ("createRealm", create_realm as NativeFn, 0),
        ("evalScript", eval_script, 1),
        ("gc", gc, 0),
        ("detachArrayBuffer", detach_array_buffer, 1),
    ] {
        let k = agent.key(name);
        let func = create_builtin_function(
            agent,
            f,
            len,
            k,
            slots.clone(),
            Some(realm),
            None,
            None,
            false,
        );
        define(agent, d, name, Value::Object(func));
    }
    define(agent, d, "global", Value::Object(global));
    define(agent, global, "$262", Value::Object(d));
    agent.contexts.pop();
    Ok(realm)
}
