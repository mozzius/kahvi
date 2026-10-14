//! Set.prototype.add ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.add

use crate::support::collections::{collection_key, set_table, this_set};
use kahvi_core::prelude::*;

/// Set.prototype.add
pub(crate) fn prototype_add(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let set be the this value.
    // 2. Perform ? RequireInternalSlot(set, [[SetData]]).
    let set = this_set(agent, this)?;
    // 3. Set value to CanonicalizeKeyedCollectionKey(value).
    let value = collection_key(agent, args.get_or_undefined(0));
    // 4. For each element entry of set.[[SetData]], do
    //    a. If entry is not ~empty~ and SameValue(entry, value) is true, then
    //       i. Return set.
    if set_table(agent, set).has(&value) {
        return Ok(Value::Object(set));
    }
    // 5. Append value to set.[[SetData]].
    set_table(agent, set).set(value, Value::Undefined);
    // 6. Return set.
    Ok(Value::Object(set))
}
