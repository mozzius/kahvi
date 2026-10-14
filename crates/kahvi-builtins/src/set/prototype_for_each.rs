//! Set.prototype.forEach ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.foreach

use crate::support::collections::{set_table, this_set};
use kahvi_core::prelude::*;

/// Set.prototype.forEach
pub(crate) fn prototype_for_each(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _set_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    let set = this_set(agent, this)?;
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    let callback = args.get_or_undefined(0);
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("Set.prototype.forEach callback is not a function"));
    }
    let this_arg = args.get_or_undefined(1);
    // 4. Let _entries_ be _set_.[[SetData]].
    // 5. Let _entriesCount_ be the number of elements in _entries_.
    // 6. Let _index_ be 0.
    let mut index = 0;
    // 7. Repeat, while _index_ < _entriesCount_,
    //    (_entriesCount_ is re-read from the table on each pass: callbacks may add entries.)
    while index < set_table(agent, set).slots() {
        // a. Let _entry_ be _entries_[_index_].
        let entry = set_table(agent, set).entry(index);
        // b. Set _index_ to _index_ + 1.
        index += 1;
        // c. If _entry_ is not ~empty~, then
        if let Some((value, _)) = entry {
            // i. Perform ? Call(_callback_, _thisArg_, « _entry_, _entry_, _set_ »).
            // ii. NOTE: The number of elements in _entries_ may have increased during execution of _callback_.
            // iii. Set _entriesCount_ to the number of elements in _entries_.
            ops::call(
                agent,
                callback,
                this_arg,
                &[value, value, Value::Object(set)],
            )?;
        }
    }
    // 8. Return *undefined*.
    Ok(Value::Undefined)
}
