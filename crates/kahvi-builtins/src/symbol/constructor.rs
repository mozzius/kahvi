//! Symbol ( [ _description_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol-description

use kahvi_core::prelude::*;

/// Symbol
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is not *undefined*, throw a *TypeError* exception.
    // 2. If _description_ is *undefined*, let _descString_ be *undefined*.
    // 3. Else, let _descString_ be ? ToString(_description_).
    // 4. Return a new Symbol whose [[Description]] is _descString_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Symbol"))
}
