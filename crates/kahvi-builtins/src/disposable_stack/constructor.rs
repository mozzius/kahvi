//! DisposableStack ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack

use kahvi_core::prelude::*;

use crate::support::disposable_stack::DisposableStackData;

/// DisposableStack
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor DisposableStack requires 'new'"));
    };
    // 2. Let _disposableStack_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%DisposableStack.prototype%"*, « [[DisposableState]], [[DisposableResourceStack]] »).
    // 3. Set _disposableStack_.[[DisposableState]] to ~pending~.
    // 4. Set _disposableStack_.[[DisposableResourceStack]] to a new empty List.
    let disposable_stack = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%DisposableStack.prototype%",
        ObjectKind::Native(Box::new(DisposableStackData {
            disposed: false,
            stack: Vec::new(),
        })),
    )?;
    // 5. Return _disposableStack_.
    Ok(Value::Object(disposable_stack))
}
