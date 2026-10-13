//! Set.prototype.has ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.has

use kahvi_core::prelude::*;

/// Set.prototype.has
pub(crate) fn prototype_has(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    // 3. Set _value_ to CanonicalizeKeyedCollectionKey(_value_).
    // 4. For each element _entry_ of _set_.[[SetData]], do
    //    a. If _entry_ is not ~empty~ and SameValue(_entry_, _value_) is *true*, return *true*.
    // 5. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.has"))
}
