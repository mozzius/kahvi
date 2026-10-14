//! Object.groupBy ( _items_, _callback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.groupby

use std::collections::HashMap;

use kahvi_core::prelude::*;

use crate::support::iterator_helper::if_abrupt_close_iterator;

/// Object.groupBy
pub(crate) fn group_by(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that accepts two arguments. `groupBy` calls _callback_ once for each element in _items_, in ascending order, and constructs a new object. Each value returned by _callback_ is coerced to a property key. For each such property key, the result object has a property whose key is that property key and whose value is an array containing all the elements for which the _callback_ return value coerced to that key.
    // _callback_ is called with two arguments: the value of the element and the index of the element.
    // The return value of `groupBy` is an object that does not inherit from %Object.prototype%.
    // This function performs the following steps when called:
    //
    // 1. Let _groups_ be ? GroupBy(_items_, _callback_, ~property~).
    // 2. Let _obj_ be OrdinaryObjectCreate(*null*).
    // 3. For each Record { [[Key]], [[Elements]] } _group_ of _groups_, do
    //    a. Let _elements_ be CreateArrayFromList(_group_.[[Elements]]).
    //    b. Perform ! CreateDataPropertyOrThrow(_obj_, _group_.[[Key]], _elements_).
    // 4. Return _obj_.
    let _ = (this, new_target);
    let items = args.get_or_undefined(0);
    let callback = args.get_or_undefined(1);
    // The groups, the new object and the arrays stay rooted while JavaScript runs.
    agent.with_roots(&[items, callback], |agent| {
        // 1. Let groups be ? GroupBy(items, callback, ~property~).
        let groups = group_by_property(agent, items, callback)?;
        // 2. Let obj be OrdinaryObjectCreate(null).
        let obj = agent.ordinary_object_create(None);
        agent.push_root(Value::Object(obj));
        // 3. For each Record { [[Key]], [[Elements]] } group of groups, do
        for (key, elements) in groups {
            // a. Let elements be CreateArrayFromList(group.[[Elements]]).
            let array = ops::create_array_from_list(agent, &elements);
            agent.push_root(Value::Object(array));
            // b. Perform ! CreateDataPropertyOrThrow(obj, group.[[Key]], elements).
            ops::create_data_property_or_throw(agent, obj, key, Value::Object(array))?;
        }
        // 4. Return obj.
        Ok(Value::Object(obj))
    })
}

/// GroupBy(_items_, _callback_, _keyCoercion_) with _keyCoercion_ = ~property~.
/// Returns the groups in the order their keys were first seen. Every key and
/// element it returns is pushed onto the agent's root stack, so the caller must
/// run it inside `with_roots`.
fn group_by_property(
    agent: &mut Agent,
    items: Value,
    callback: Value,
) -> JsResult<Vec<(PropertyKey, Vec<Value>)>> {
    // 1. Perform ? RequireObjectCoercible(items).
    ops::require_object_coercible(agent, items)?;
    // 2. If IsCallable(callback) is false, throw a TypeError exception.
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("Object.groupBy callback is not a function"));
    }
    // 3. Let groups be a new empty List.
    // `order` keeps the keys in first-seen order; `elements` holds each group's [[Elements]].
    let mut order: Vec<PropertyKey> = Vec::new();
    let mut elements: HashMap<PropertyKey, Vec<Value>> = HashMap::new();
    // 4. Let iteratorRecord be ? GetIterator(items, sync).
    let mut iterator_record = ops::get_iterator(agent, items, ops::IteratorKind::Sync)?;
    agent.push_root(Value::Object(iterator_record.iterator));
    agent.push_root(iterator_record.next_method);
    // 5. Let k be 0.
    let mut k = 0.0;
    // 6. Repeat,
    loop {
        // a. If k ≥ 2^53 - 1, then
        if k >= 9_007_199_254_740_991.0 {
            // i. Let error be ThrowCompletion(a newly created TypeError object).
            let error = agent.throw_type_error("Object.groupBy: too many elements");
            // ii. Return ? IteratorClose(iteratorRecord, error).
            return ops::iterator_close(agent, &iterator_record, Err(error));
        }
        // b. Let next be ? IteratorStepValue(iteratorRecord).
        let Some(value) = ops::iterator_step_value_rooted(agent, &mut iterator_record)? else {
            // c. If next is done, then
            //    i. Return groups.
            return Ok(order
                .into_iter()
                .filter_map(|key| elements.remove(&key).map(|list| (key, list)))
                .collect());
        };
        // d. Let value be next.
        agent.push_root(value);
        // e. Let key be Completion(Call(callback, undefined, « value, 𝔽(k) »)).
        let key = ops::call_rooted(
            agent,
            callback,
            Value::Undefined,
            &[value, Value::Number(k)],
        );
        // f. IfAbruptCloseIterator(key, iteratorRecord).
        let key = if_abrupt_close_iterator(agent, key, &iterator_record)?;
        // g. If keyCoercion is ~property~, then
        //    i. Set key to Completion(ToPropertyKey(key)).
        let key = ops::to_property_key(agent, key);
        //    ii. IfAbruptCloseIterator(key, iteratorRecord).
        let key = if_abrupt_close_iterator(agent, key, &iterator_record)?;
        // i. Perform AddValueToKeyedGroup(groups, key, value).
        if !elements.contains_key(&key) {
            agent.push_root_key(key);
            order.push(key);
        }
        elements.entry(key).or_default().push(value);
        // j. Set k to k + 1.
        k += 1.0;
    }
}
