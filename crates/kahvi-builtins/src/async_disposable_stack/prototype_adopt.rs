//! AsyncDisposableStack.prototype.adopt ( _value_, _onDisposeAsync_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.adopt

use kahvi_core::function::{builtin_slots, create_builtin_function};
use kahvi_core::ops::dispose::DisposeKind;
use kahvi_core::prelude::*;

use crate::support::disposable_stack::{
    add_disposable_resource, async_disposable_stack, require_async_disposable_stack,
};

/// AsyncDisposableStack.prototype.adopt
pub(crate) fn prototype_adopt(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let value = args.get_or_undefined(0);
    let on_dispose_async = args.get_or_undefined(1);
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    let o = require_async_disposable_stack(agent, this)?;
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    if async_disposable_stack(agent, o).disposed {
        return Err(agent.throw_reference_error("the AsyncDisposableStack is already disposed"));
    }
    // 4. If IsCallable(_onDisposeAsync_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, on_dispose_async) {
        return Err(
            agent.throw_type_error("AsyncDisposableStack.prototype.adopt requires a callable")
        );
    }
    // 5. Let _closure_ be a new Abstract Closure with no parameters that captures _value_ and _onDisposeAsync_ and performs the following steps when called:
    //    a. Return ? Call(_onDisposeAsync_, *undefined*, « _value_ »).
    //
    // The captured values are the built-in's slots: [_value_, _onDisposeAsync_].
    // 6. Let _func_ be CreateBuiltinFunction(_closure_, 0, *""*, « »).
    let empty = PropertyKey::String(agent.string(""));
    let func = create_builtin_function(
        agent,
        adopt_closure,
        0,
        empty,
        vec![value, on_dispose_async],
        None,
        None,
        None,
        false,
    );
    // 7. Perform ? AddDisposableResource(_asyncDisposableStack_.[[DisposableResourceStack]], *undefined*, ~async-dispose~, _func_).
    add_disposable_resource(
        agent,
        o,
        Value::Undefined,
        DisposeKind::Async,
        Some(Value::Object(func)),
    )?;
    // 8. Return _value_.
    Ok(value)
}

/// The Abstract Closure of step 5, run as the function created in step 6.
fn adopt_closure(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let f = agent.active_function().expect("a running built-in");
    let (value, on_dispose_async) = match builtin_slots(agent, f) {
        [value, on_dispose_async] => (*value, *on_dispose_async),
        _ => unreachable!("an adopt closure without its captured values"),
    };
    // a. Return ? Call(_onDisposeAsync_, *undefined*, « _value_ »).
    ops::call(agent, on_dispose_async, Value::Undefined, &[value])
}
