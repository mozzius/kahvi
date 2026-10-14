//! DisposableStack.prototype.adopt ( _value_, _onDispose_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack.prototype.adopt

use kahvi_core::function::{builtin_slots, create_builtin_function};
use kahvi_core::ops::dispose::DisposeKind;
use kahvi_core::prelude::*;

use crate::support::disposable_stack::{
    add_disposable_resource, disposable_stack, require_disposable_stack,
};

/// DisposableStack.prototype.adopt
pub(crate) fn prototype_adopt(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let value = args.get_or_undefined(0);
    let on_dispose = args.get_or_undefined(1);
    // 1. Let _disposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
    let o = require_disposable_stack(agent, this)?;
    // 3. If _disposableStack_.[[DisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    if disposable_stack(agent, o).disposed {
        return Err(agent.throw_reference_error("the DisposableStack is already disposed"));
    }
    // 4. If IsCallable(_onDispose_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, on_dispose) {
        return Err(agent.throw_type_error("the onDispose argument is not callable"));
    }
    // 5. Let _closure_ be a new Abstract Closure with no parameters that captures _value_ and _onDispose_ and performs the following steps when called:
    //    a. Return ? Call(_onDispose_, *undefined*, « _value_ »).
    // 6. Let _func_ be CreateBuiltinFunction(_closure_, 0, *""*, « »).
    let empty = PropertyKey::String(agent.string(""));
    let func = create_builtin_function(
        agent,
        adopt_closure,
        0,
        empty,
        vec![value, on_dispose],
        None,
        None,
        None,
        false,
    );
    // 7. Perform ? AddDisposableResource(_disposableStack_.[[DisposableResourceStack]], *undefined*, ~sync-dispose~, _func_).
    add_disposable_resource(
        agent,
        o,
        Value::Undefined,
        DisposeKind::Sync,
        Some(Value::Object(func)),
    )?;
    // 8. Return _value_.
    Ok(value)
}

/// The closure of step 5: calls _onDispose_ with _value_ (both captured slots).
fn adopt_closure(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let f = agent.active_function().expect("a running built-in");
    let slots = builtin_slots(agent, f);
    let (value, on_dispose) = (slots[0], slots[1]);
    // a. Return ? Call(_onDispose_, *undefined*, « _value_ »).
    ops::call(agent, on_dispose, Value::Undefined, &[value])
}
