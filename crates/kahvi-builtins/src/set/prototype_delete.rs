//! Set.prototype.delete ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.delete

use kahvi_core::prelude::*;

/// Set.prototype.delete
pub(crate) fn prototype_delete(
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
    //    a. If _entry_ is not ~empty~ and SameValue(_entry_, _value_) is *true*, then
    //       i. Replace the element of _set_.[[SetData]] whose value is _entry_ with an element whose value is ~empty~.
    //       ii. Return *true*.
    // 5. Return *false*.
    //
    // NOTE:
    // The value ~empty~ is used as a specification device to indicate that an entry has been deleted. Actual implementations may take other actions such as physically removing the entry from internal data structures.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.delete"))
}
