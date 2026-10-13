//! Proxy ( _target_, _handler_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-target-handler

use kahvi_core::prelude::*;

/// Proxy
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Return ? ProxyCreate(_target_, _handler_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Proxy"))
}
