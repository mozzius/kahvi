//! Object.assign ( _target_, ..._sources_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.assign

use kahvi_core::prelude::*;

/// Object.assign
pub(crate) fn assign(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function copies the values of all of the enumerable own properties from one or more source objects to a _target_ object.
    // It performs the following steps when called:
    //
    // 1. Let _targetObj_ be ? ToObject(_target_).
    let target_obj = ops::to_object(agent, args.get_or_undefined(0).clone())?;
    // 2. If only one argument was passed, return _targetObj_.
    if args.len() == 1 {
        return Ok(Value::Object(target_obj));
    }
    // 3. For each element _nextSource_ of _sources_, do
    for next_source in args.iter().skip(1) {
        // a. If _nextSource_ is neither *undefined* nor *null*, then
        if !matches!(next_source, Value::Undefined | Value::Null) {
            // i. Let _from_ be ! ToObject(_nextSource_).
            let from = ops::to_object(agent, next_source.clone())?;
            // ii. Let _keys_ be ? _from_.[[OwnPropertyKeys]]().
            let keys = from.internal_own_property_keys(agent)?;
            // iii. For each element _nextKey_ of _keys_, do
            for next_key in keys {
                // 1. Let _propertyDesc_ be ? _from_.[[GetOwnProperty]](_nextKey_).
                let property_desc = from.internal_get_own_property(agent, next_key)?;
                // 2. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Enumerable]] is *true*, then
                if let Some(property_desc) = property_desc {
                    if property_desc.enumerable == Some(true) {
                        // a. Let _propertyValue_ be ? Get(_from_, _nextKey_).
                        let property_value = ops::get(agent, from, next_key)?;
                        // b. Perform ? Set(_targetObj_, _nextKey_, _propertyValue_, *true*).
                        ops::set(agent, target_obj, next_key, property_value, true)?;
                    }
                }
            }
        }
    }
    // 4. Return _targetObj_.
    Ok(Value::Object(target_obj))
}
