//! Set.prototype.clear ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.clear

use crate::support::collections::{set_table, this_set};
use kahvi_core::prelude::*;

/// Set.prototype.clear
pub(crate) fn prototype_clear(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    let set = this_set(agent, this)?;
    // 3. For each element _entry_ of _set_.[[SetData]], do
    //    a. Replace the element of _set_.[[SetData]] whose value is _entry_ with an element whose value is ~empty~.
    set_table(agent, set).clear();
    // 4. Return *undefined*.
    Ok(Value::Undefined)
}
