//! AsyncDisposableStack.prototype.adopt ( _value_, _onDisposeAsync_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.adopt

use kahvi_core::prelude::*;

/// AsyncDisposableStack.prototype.adopt
pub(crate) fn prototype_adopt(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    // 4. If IsCallable(_onDisposeAsync_) is *false*, throw a *TypeError* exception.
    // 5. Let _closure_ be a new Abstract Closure with no parameters that captures _value_ and _onDisposeAsync_ and performs the following steps when called:
    //    a. Return ? Call(_onDisposeAsync_, *undefined*, « _value_ »).
    // 6. Let _func_ be CreateBuiltinFunction(_closure_, 0, *""*, « »).
    // 7. Perform ? AddDisposableResource(_asyncDisposableStack_.[[DisposableResourceStack]], *undefined*, ~async-dispose~, _func_).
    // 8. Return _value_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncDisposableStack.prototype.adopt"))
}
