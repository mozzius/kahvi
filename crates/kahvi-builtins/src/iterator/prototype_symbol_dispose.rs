//! Iterator.prototype [ %Symbol.dispose% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype-%symbol.dispose%

use kahvi_core::prelude::*;

/// Iterator.prototype[Symbol.dispose]
pub(crate) fn prototype_symbol_dispose(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _return_ be ? GetMethod(_obj_, *"return"*).
    // 3. If _return_ is not *undefined*, then
    //    a. Perform ? Call(_return_, _obj_).
    // 4. Return *undefined*.
    //
    // The value of the *"name"* property of this method is *"[Symbol.dispose]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype[Symbol.dispose]"))
}
