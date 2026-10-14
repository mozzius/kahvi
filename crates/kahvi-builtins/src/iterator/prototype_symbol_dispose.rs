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
    let _ = (args, new_target);
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _return_ be ? GetMethod(_obj_, *"return"*).
    let ret = ops::get_method(agent, obj, PropertyKey::String(agent.names.r#return))?;
    // 3. If _return_ is not *undefined*, then
    if !ret.is_undefined() {
        // a. Perform ? Call(_return_, _obj_).
        ops::call(agent, ret, obj, &[])?;
    }
    // 4. Return *undefined*.
    Ok(Value::Undefined)
}
