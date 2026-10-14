//! %IteratorHelperPrototype%.return ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%iteratorhelperprototype%.return

use kahvi_core::prelude::*;

use crate::support::iterator_helper::helper_return;

/// IteratorHelperPrototype.return
pub(crate) fn r#return(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be this value.
    // 2-6. RequireInternalSlot(O, [[UnderlyingIterators]]); close it (suspended-start) or
    //      GeneratorResumeAbrupt(O, ReturnCompletion(undefined), "Iterator Helper").
    helper_return(agent, this)
}
