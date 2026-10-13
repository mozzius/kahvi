//! %WrapForValidIteratorPrototype%.return ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%wrapforvaliditeratorprototype%.return

use kahvi_core::prelude::*;

/// WrapForValidIteratorPrototype.return
pub(crate) fn r#return(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[Iterated]]).
    // 3. Let _iterator_ be _obj_.[[Iterated]].[[Iterator]].
    // 4. Assert: _iterator_ is an Object.
    // 5. Let _returnMethod_ be ? GetMethod(_iterator_, *"return"*).
    // 6. If _returnMethod_ is *undefined*, then
    //    a. Return CreateIteratorResultObject(*undefined*, *true*).
    // 7. Return ? Call(_returnMethod_, _iterator_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("WrapForValidIteratorPrototype.return"))
}
