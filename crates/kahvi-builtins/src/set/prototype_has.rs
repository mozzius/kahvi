//! Set.prototype.has ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.has

use crate::support::collections::{collection_key, set_table, this_set};
use kahvi_core::prelude::*;

/// Set.prototype.has
pub(crate) fn prototype_has(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _set_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    let set = this_set(agent, this)?;
    // 3. Set _value_ to CanonicalizeKeyedCollectionKey(_value_).
    let value = collection_key(agent, args.get_or_undefined(0));
    // 4. For each element _entry_ of _set_.[[SetData]], do
    //    a. If _entry_ is not ~empty~ and SameValue(_entry_, _value_) is *true*, return *true*.
    // 5. Return *false*.
    // The table is keyed by SameValueZero, which agrees with SameValue once the
    // key is canonicalized (-0 became +0 above, and all NaNs are one key).
    Ok(Value::Boolean(set_table(agent, set).has(&value)))
}
