//! Proxy.revocable ( _target_, _handler_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy.revocable

use kahvi_core::prelude::*;

/// Proxy.revocable
pub(crate) fn revocable(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function creates a revocable Proxy object.
    // It performs the following steps when called:
    //
    // 1. Let _proxy_ be ? ProxyCreate(_target_, _handler_).
    // 2. Let _revokerClosure_ be a new Abstract Closure with no parameters that captures nothing and performs the following steps when called:
    //    a. Let _activeFunc_ be the active function object.
    //    b. Let _revocableProxy_ be _activeFunc_.[[RevocableProxy]].
    //    c. If _revocableProxy_ is *null*, return NormalCompletion(*undefined*).
    //    d. Set _activeFunc_.[[RevocableProxy]] to *null*.
    //    e. Assert: _revocableProxy_ is a Proxy exotic object.
    //    f. Set _revocableProxy_.[[ProxyTarget]] to *null*.
    //    g. Set _revocableProxy_.[[ProxyHandler]] to *null*.
    //    h. Return NormalCompletion(*undefined*).
    // 3. Let _revoker_ be CreateBuiltinFunction(_revokerClosure_, 0, *""*, « [[RevocableProxy]] »).
    // 4. Set _revoker_.[[RevocableProxy]] to _proxy_.
    // 5. Let _result_ be OrdinaryObjectCreate(%Object.prototype%).
    // 6. Perform ! CreateDataPropertyOrThrow(_result_, *"proxy"*, _proxy_).
    // 7. Perform ! CreateDataPropertyOrThrow(_result_, *"revoke"*, _revoker_).
    // 8. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Proxy.revocable"))
}
