//! Array.prototype.toReversed ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.toreversed

use kahvi_core::object::array::array_create;
use kahvi_core::prelude::*;

/// Array.prototype.toReversed
pub(crate) fn prototype_to_reversed(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    let length = ops::length_of_array_like(agent, obj)?;
    // 3. Let _array_ be ? ArrayCreate(_length_).
    let array = array_create(agent, length, None)?;
    // The getter may collect garbage: _obj_ and _array_ stay rooted.
    agent.with_roots(&[Value::Object(obj), Value::Object(array)], |agent| {
        // 4. Let _k_ be 0.
        let mut k = 0.0;
        // 5. Repeat, while _k_ < _length_,
        while k < length {
            // a. Let _from_ be ! ToString(𝔽(_length_ - _k_ - 1)).
            let from = agent.key_from_index(length - k - 1.0);
            // b. Let _propertyKey_ be ! ToString(𝔽(_k_)).
            let property_key = agent.key_from_index(k);
            // c. Let _fromValue_ be ? Get(_obj_, _from_).
            let from_value = ops::get(agent, obj, from)?;
            // d. Perform ! CreateDataPropertyOrThrow(_array_, _propertyKey_, _fromValue_).
            let mark = agent.push_root(from_value);
            agent.push_root_key(property_key);
            ops::create_data_property_or_throw(agent, array, property_key, from_value)?;
            agent.pop_roots(mark);
            // e. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // 6. Return _array_.
        Ok(Value::Object(array))
    })
}
