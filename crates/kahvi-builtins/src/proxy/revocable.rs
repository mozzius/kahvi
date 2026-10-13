//! Proxy.revocable ( _target_, _handler_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy.revocable

use kahvi_core::function::{builtin_slots, create_builtin_function, set_builtin_slot};
use kahvi_core::object::proxy::{self, proxy_create};
use kahvi_core::prelude::*;

/// Proxy.revocable
pub(crate) fn revocable(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let proxy be ? ProxyCreate(target, handler).
    let proxy = proxy_create(agent, args.get_or_undefined(0), args.get_or_undefined(1))?;
    // 2-4. Let revoker be CreateBuiltinFunction(revokerClosure, 0, "", « [[RevocableProxy]] »).
    let empty = PropertyKey::String(agent.names.empty);
    let revoker = create_builtin_function(
        agent,
        revoke_proxy,
        0,
        empty,
        vec![Value::Object(proxy)],
        None,
        None,
        None,
        false,
    );
    // 5. Set revoker.[[RevocableProxy]] to proxy.
    // 6. Let result be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let result = agent.ordinary_object_create(Some(proto));
    // 7. Perform ! CreateDataPropertyOrThrow(result, "proxy", proxy).
    let proxy_key = agent.key("proxy");
    ops::create_data_property_or_throw(agent, result, proxy_key, Value::Object(proxy))?;
    // 8. Perform ! CreateDataPropertyOrThrow(result, "revoke", revoker).
    let revoke_key = agent.key("revoke");
    ops::create_data_property_or_throw(agent, result, revoke_key, Value::Object(revoker))?;
    // 9. Return result.
    Ok(Value::Object(result))
}

/// The revokerClosure of Proxy.revocable (§28.2.2.1 step 2).
fn revoke_proxy(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // a. Let F be the active function object.
    let f = agent
        .active_function()
        .expect("a built-in runs with an active function");
    // b. Let p be F.[[RevocableProxy]].
    // c. If p is null, return undefined.
    let Some(Value::Object(p)) = builtin_slots(agent, f).first().copied() else {
        return Ok(Value::Undefined);
    };
    // d. Set F.[[RevocableProxy]] to null.
    set_builtin_slot(agent, f, 0, Value::Null);
    // e-g. Set p.[[ProxyTarget]] and p.[[ProxyHandler]] to null.
    proxy::revoke(agent, p);
    // h. Return undefined.
    Ok(Value::Undefined)
}
