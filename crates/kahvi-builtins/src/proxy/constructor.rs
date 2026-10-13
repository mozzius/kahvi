//! Proxy ( _target_, _handler_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-target-handler

use kahvi_core::object::proxy::proxy_create;
use kahvi_core::prelude::*;

/// Proxy
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, throw a TypeError exception.
    if new_target.is_none() {
        return Err(agent.throw_type_error("Constructor Proxy requires 'new'"));
    }
    // 2. Return ? ProxyCreate(target, handler).
    Ok(Value::Object(proxy_create(
        agent,
        args.get_or_undefined(0),
        args.get_or_undefined(1),
    )?))
}
