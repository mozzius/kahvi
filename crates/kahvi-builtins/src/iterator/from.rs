//! Iterator.from ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.from

use kahvi_core::prelude::*;

/// Iterator.from
pub(crate) fn from(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _iteratorRecord_ be ? GetIteratorFlattenable(_obj_, ~iterate-string-primitives~).
    // 2. Let _hasInstance_ be ? OrdinaryHasInstance(%Iterator%, _iteratorRecord_.[[Iterator]]).
    // 3. If _hasInstance_ is *true*, then
    //    a. Return _iteratorRecord_.[[Iterator]].
    // 4. Let _wrapper_ be OrdinaryObjectCreate(%WrapForValidIteratorPrototype%, « [[Iterated]] »).
    // 5. Set _wrapper_.[[Iterated]] to _iteratorRecord_.
    // 6. Return _wrapper_.
    //
    // [subclause sec-%wrapforvaliditeratorprototype%-object: The %WrapForValidIteratorPrototype% Object]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.from"))
}
