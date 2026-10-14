//! Array.prototype.push ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.push

use kahvi_core::prelude::*;

/// Array.prototype.push
pub(crate) fn prototype_push(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 2. Let len be ? LengthOfArrayLike(O).
    let mut len = ops::length_of_array_like(agent, o)?;
    // 3. Let argCount be the number of elements in items.
    // 4. If len + argCount > 2^53 - 1, throw a TypeError exception.
    if len + args.len() as f64 > 9007199254740991.0 {
        return Err(agent.throw_type_error("Pushing too many elements"));
    }
    // 5. For each element E of items, do
    for e in args {
        // a. Perform ? Set(O, ! ToString(𝔽(len)), E, true).
        let key = agent.key_from_index(len);
        ops::set(agent, o, key, *e, true)?;
        // b. Set len to len + 1.
        len += 1.0;
    }
    // 6. Perform ? Set(O, "length", 𝔽(len), true).
    let length_key = PropertyKey::String(agent.names.length);
    ops::set(agent, o, length_key, Value::Number(len), true)?;
    // 7. Return 𝔽(len).
    Ok(Value::Number(len))
}
