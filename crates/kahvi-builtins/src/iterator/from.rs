//! Iterator.from ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.from

use kahvi_core::ops::IteratorRecord;
use kahvi_core::prelude::*;

use crate::support::iterator_helper::create_wrap_for_valid_iterator;

/// Iterator.from
pub(crate) fn from(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let obj = args.get_or_undefined(0);
    // 1. Let _iteratorRecord_ be ? GetIteratorFlattenable(_obj_, ~iterate-string-primitives~).
    let iterator_record = get_iterator_flattenable_iterate_strings(agent, obj)?;
    // 2. Let _hasInstance_ be ? OrdinaryHasInstance(%Iterator%, _iteratorRecord_.[[Iterator]]).
    let iterator_ctor = agent.intrinsic("%Iterator%");
    let has_instance = ops::ordinary_has_instance(
        agent,
        Value::Object(iterator_ctor),
        Value::Object(iterator_record.iterator),
    )?;
    // 3. If _hasInstance_ is *true*, then
    if has_instance {
        // a. Return _iteratorRecord_.[[Iterator]].
        return Ok(Value::Object(iterator_record.iterator));
    }
    // 4. Let _wrapper_ be OrdinaryObjectCreate(%WrapForValidIteratorPrototype%, « [[Iterated]] »).
    // 5. Set _wrapper_.[[Iterated]] to _iteratorRecord_.
    let wrapper = create_wrap_for_valid_iterator(agent, iterator_record);
    // 6. Return _wrapper_.
    Ok(Value::Object(wrapper))
}

/// GetIteratorFlattenable ( _obj_, ~iterate-string-primitives~ ) (§7.4.17)
fn get_iterator_flattenable_iterate_strings(
    agent: &mut Agent,
    obj: Value,
) -> JsResult<IteratorRecord> {
    // 1. If _obj_ is not an Object, then
    if !matches!(obj, Value::Object(_)) {
        // a. If _primitiveHandling_ is ~reject-primitives~, throw a *TypeError* exception.
        // b. Assert: _primitiveHandling_ is ~iterate-string-primitives~.
        // c. If _obj_ is not a String, throw a *TypeError* exception.
        if !matches!(obj, Value::String(_)) {
            return Err(agent.throw_type_error("Iterator.from: value is not an object or string"));
        }
    }
    // 2. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
    let method = ops::get_method(agent, obj, PropertyKey::Symbol(agent.symbols_wk.iterator))?;
    // 3. If _method_ is *undefined*, then
    let iterator = if method.is_undefined() {
        // a. Let _iterator_ be _obj_.
        obj
    } else {
        // 4. Else,
        // a. Let _iterator_ be ? Call(_method_, _obj_).
        ops::call(agent, method, obj, &[])?
    };
    // 5. If _iterator_ is not an Object, throw a *TypeError* exception.
    let Value::Object(iterator) = iterator else {
        return Err(agent.throw_type_error("Iterator.from: iterator is not an object"));
    };
    // 6. Return ? GetIteratorDirect(_iterator_).
    ops::get_iterator_direct(agent, iterator)
}
