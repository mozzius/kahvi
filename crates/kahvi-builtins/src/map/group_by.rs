//! Map.groupBy ( _items_, _callback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.groupby

use crate::support::collections::{MapData, collection_key, map_table};
use crate::support::iterator_helper::if_abrupt_close_iterator;
use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// Map.groupBy
pub(crate) fn group_by(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that accepts two arguments. `groupBy` calls _callback_ once for each element in _items_, in ascending order, and constructs a new Map. Each value returned by _callback_ is used as a key in the Map. For each such key, the result Map has an entry whose key is that key and whose value is an array containing all the elements for which _callback_ returned that key.
    // _callback_ is called with two arguments: the value of the element and the index of the element.
    // The return value of `groupBy` is a Map.
    // This function performs the following steps when called:
    //
    // 1. Let _groups_ be ? GroupBy(_items_, _callback_, ~collection~).
    // 2. Let _map_ be ! Construct(%Map%).
    // 3. For each Record { [[Key]], [[Elements]] } _group_ of _groups_, do
    //    a. Let _elements_ be CreateArrayFromList(_group_.[[Elements]]).
    //    b. Let _entry_ be the Record { [[Key]]: _group_.[[Key]], [[Value]]: _elements_ }.
    //    c. Append _entry_ to _map_.[[MapData]].
    // 4. Return _map_.
    //
    // The groups are built as the entries of _map_ while GroupBy runs: each
    // group's elements array is filled by AddValueToKeyedGroup, and the map
    // is what keeps those keys and arrays alive during the callback calls.
    // Nothing observable happens between the two steps, so the map is made first.
    let items = args.get_or_undefined(0);
    let callback = args.get_or_undefined(1);
    let proto = agent.intrinsic("%Map.prototype%");
    let map = agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(MapData::default())),
    );

    // GroupBy ( _items_: an ECMAScript language value, _callback_: an ECMAScript language value, _keyCoercion_: ~property~ or ~collection~, ): either a normal completion containing a List of Records with fields [[Key]] and [[Elements]], or a throw completion
    //
    // 1. Perform ? RequireObjectCoercible(_items_).
    ops::require_object_coercible(agent, items)?;
    // 2. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("Map.groupBy callback is not a function"));
    }
    // 3. Let _groups_ be a new empty List.
    // 4. Let _iteratorRecord_ be ? GetIterator(_items_, ~sync~).
    let mut record = ops::get_iterator(agent, items, ops::IteratorKind::Sync)?;
    // The iterator, the callback and the map stay rooted while callbacks run.
    agent.with_roots(
        &[
            items,
            callback,
            Value::Object(map),
            Value::Object(record.iterator),
            record.next_method,
        ],
        |agent| {
            // 5. Let _k_ be 0.
            let mut k = 0.0;
            // 6. Repeat,
            loop {
                // a. If _k_ ≥ 2^53 - 1, then
                if k >= 9_007_199_254_740_991.0 {
                    // i. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
                    let error = agent.throw_type_error("too many elements to group");
                    // ii. Return ? IteratorClose(_iteratorRecord_, _error_).
                    return ops::iterator_close(agent, &record, Err(error));
                }
                // b. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
                let Some(value) = ops::iterator_step_value_rooted(agent, &mut record)? else {
                    // c. If _next_ is ~done~, then
                    //    i. Return _groups_.
                    return Ok(Value::Object(map));
                };
                // d. Let _value_ be _next_.
                let mark = agent.push_root(value);
                // e. Let _key_ be Completion(Call(_callback_, *undefined*, « _value_, 𝔽(_k_) »)).
                let key = ops::call_rooted(
                    agent,
                    callback,
                    Value::Undefined,
                    &[value, Value::Number(k)],
                );
                // f. IfAbruptCloseIterator(_key_, _iteratorRecord_).
                let key = if_abrupt_close_iterator(agent, key, &record)?;
                // g. If _keyCoercion_ is ~property~, then ... (not reached: ~collection~)
                // h. Else,
                //    i. Assert: _keyCoercion_ is ~collection~.
                //    ii. Set _key_ to CanonicalizeKeyedCollectionKey(_key_).
                let key = collection_key(agent, key);
                // i. Perform AddValueToKeyedGroup(_groups_, _key_, _value_).
                //    AddValueToKeyedGroup ( _groups_: a List of Records, _key_: an ECMAScript language value, _value_: an ECMAScript language value, ): unused
                //    1. For each Record { [[Key]], [[Elements]] } _g_ of _groups_, do
                //       a. If SameValue(_g_.[[Key]], _key_) is *true*, then
                //          i. Assert: exactly one element of _groups_ meets this criteria.
                //          ii. Append _value_ to _g_.[[Elements]].
                //          iii. Return unused.
                //    2. Let _group_ be the Record { [[Key]]: _key_, [[Elements]]: « _value_ » }.
                //    3. Append _group_ to _groups_.
                //    4. Return unused.
                let existing = map_table(agent, map).get(&key);
                match existing {
                    Some(Value::Object(elements)) => {
                        let len = ops::length_of_array_like(agent, elements)?;
                        let pk = agent.key_from_index(len);
                        ops::create_data_property_or_throw(agent, elements, pk, value)?;
                    }
                    _ => {
                        let elements = ops::create_array_from_list(agent, &[value]);
                        map_table(agent, map).set(key, Value::Object(elements));
                    }
                }
                // j. Set _k_ to _k_ + 1.
                k += 1.0;
                agent.pop_roots(mark);
            }
        },
    )
}
