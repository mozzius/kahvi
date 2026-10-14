//! %IteratorHelperPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%iteratorhelperprototype%.next

use kahvi_core::prelude::*;

use crate::support::iterator_helper::helper_resume;

/// IteratorHelperPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(this value, undefined, "Iterator Helper").
    helper_resume(agent, this)
}
