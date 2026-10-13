//! %IteratorHelperPrototype%.return ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%iteratorhelperprototype%.return

use kahvi_core::prelude::*;

/// IteratorHelperPrototype.return
pub(crate) fn r#return(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[UnderlyingIterators]]).
    // 3. Assert: _obj_ has a [[GeneratorState]] internal slot.
    // 4. If _obj_.[[GeneratorState]] is ~suspended-start~, then
    //    a. Set _obj_.[[GeneratorState]] to ~completed~.
    //    b. NOTE: Once a generator enters the completed state it never leaves it and its associated execution context is never resumed. Any execution state associated with _obj_ can be discarded at this point.
    //    c. Perform ? IteratorCloseAll(_obj_.[[UnderlyingIterators]], NormalCompletion(~unused~)).
    //    d. Return CreateIteratorResultObject(*undefined*, *true*).
    // 5. Let _completion_ be ReturnCompletion(*undefined*).
    // 6. Return ? GeneratorResumeAbrupt(_obj_, _completion_, *"Iterator Helper"*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("IteratorHelperPrototype.return"))
}
