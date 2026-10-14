//! Array.prototype.map ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.map

use kahvi_core::prelude::*;

/// Array.prototype.map
pub(crate) fn prototype_map(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 2. Let len be ? LengthOfArrayLike(O).
    let len = ops::length_of_array_like(agent, o)?;
    // 3. If IsCallable(callback) is false, throw a TypeError exception.
    let callback = args.get_or_undefined(0);
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("Array.prototype.map callback is not a function"));
    }
    let this_arg = args.get_or_undefined(1);
    // 4. Let A be ? ArraySpeciesCreate(O, len).
    let a = ops::array_species_create(agent, o, len)?;
    // The callback may collect garbage: O, callback, thisArg and A stay rooted.
    agent.with_roots(
        &[Value::Object(o), callback, this_arg, Value::Object(a)],
        |agent| {
            // 5. Let k be 0.
            let mut k = 0.0;
            // 6. Repeat, while k < len,
            while k < len {
                // a. Let Pk be ! ToString(𝔽(k)).
                let pk = agent.key_from_index(k);
                // b. Let kPresent be ? HasProperty(O, Pk).
                // c. If kPresent is true, then
                if ops::has_property(agent, o, pk)? {
                    // i. Let kValue be ? Get(O, Pk).
                    let k_value = ops::get(agent, o, pk)?;
                    // ii. Let mappedValue be ? Call(callback, thisArg, « kValue, 𝔽(k), O »).
                    let mark = agent.push_root(k_value);
                    agent.push_root_key(pk);
                    let mapped = ops::call_rooted(
                        agent,
                        callback,
                        this_arg,
                        &[k_value, Value::Number(k), Value::Object(o)],
                    )?;
                    // iii. Perform ? CreateDataPropertyOrThrow(A, Pk, mappedValue).
                    ops::create_data_property_or_throw(agent, a, pk, mapped)?;
                    agent.pop_roots(mark);
                }
                // d. Set k to k + 1.
                k += 1.0;
            }
            // 7. Return A.
            Ok(Value::Object(a))
        },
    )
}
