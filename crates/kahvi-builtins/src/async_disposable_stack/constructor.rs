//! AsyncDisposableStack ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack

use kahvi_core::prelude::*;

use crate::support::disposable_stack::AsyncDisposableStackData;

/// AsyncDisposableStack
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor AsyncDisposableStack requires 'new'"));
    };
    // 2. Let _asyncDisposableStack_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%AsyncDisposableStack.prototype%"*, « [[AsyncDisposableState]], [[DisposableResourceStack]] »).
    // 3. Set _asyncDisposableStack_.[[AsyncDisposableState]] to ~pending~.
    // 4. Set _asyncDisposableStack_.[[DisposableResourceStack]] to a new empty List.
    let async_disposable_stack = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%AsyncDisposableStack.prototype%",
        ObjectKind::Native(Box::new(AsyncDisposableStackData {
            disposed: false,
            stack: Vec::new(),
        })),
    )?;
    // 5. Return _asyncDisposableStack_.
    Ok(Value::Object(async_disposable_stack))
}
