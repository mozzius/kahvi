//! Map.prototype.forEach ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.foreach

use crate::support::collections::{map_table, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.forEach
pub(crate) fn prototype_for_each(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _map_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_map_, [[MapData]]).
    let map = this_map(agent, this)?;
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    let callback = args.get_or_undefined(0);
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("Map.prototype.forEach callback is not a function"));
    }
    let this_arg = args.get_or_undefined(1);
    // The callback may collect garbage: map, callback and thisArg stay rooted.
    agent.with_roots(&[Value::Object(map), callback, this_arg], |agent| {
        // 4. Let _entries_ be _map_.[[MapData]].
        // 5. Let _entriesCount_ be the number of elements in _entries_.
        let mut entries_count = map_table(agent, map).slots();
        // 6. Let _index_ be 0.
        let mut index = 0;
        // 7. Repeat, while _index_ < _entriesCount_,
        while index < entries_count {
            // a. Let _entry_ be _entries_[_index_].
            let entry = map_table(agent, map).entry(index);
            // b. Set _index_ to _index_ + 1.
            index += 1;
            // c. If _entry_.[[Key]] is not ~empty~, then
            if let Some((key, value)) = entry {
                // The entry's key and value stay rooted during the call.
                let mark = agent.push_root(value);
                agent.push_root(key);
                // i. Perform ? Call(_callback_, _thisArg_, « _entry_.[[Value]], _entry_.[[Key]], _map_ »).
                ops::call_rooted(agent, callback, this_arg, &[value, key, Value::Object(map)])?;
                agent.pop_roots(mark);
                // ii. NOTE: The number of elements in _entries_ may have increased during execution of _callback_.
                // iii. Set _entriesCount_ to the number of elements in _entries_.
                entries_count = map_table(agent, map).slots();
            }
        }
        // 8. Return *undefined*.
        Ok(Value::Undefined)
    })
}
