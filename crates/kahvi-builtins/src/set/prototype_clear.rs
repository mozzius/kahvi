//! Set.prototype.clear ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.clear

use kahvi_core::prelude::*;

/// Set.prototype.clear
pub(crate) fn prototype_clear(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    // 3. For each element _entry_ of _set_.[[SetData]], do
    //    a. Replace the element of _set_.[[SetData]] whose value is _entry_ with an element whose value is ~empty~.
    // 4. Return *undefined*.
    //
    // NOTE:
    // The existing [[SetData]] List is preserved because there may be existing Set Iterator objects that are suspended midway through iterating over that List.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.clear"))
}
