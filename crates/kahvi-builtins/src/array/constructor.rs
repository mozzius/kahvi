//! Array ( ..._values_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array

use kahvi_core::object::array::array_create;
use kahvi_core::prelude::*;

/// Array
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, let newTarget be the active function object; else let newTarget be NewTarget.
    let new_target = match new_target {
        Some(nt) => nt,
        None => agent
            .active_function()
            .expect("a built-in runs with an active function"),
    };
    // 2. Let proto be ? GetPrototypeFromConstructor(newTarget, "%Array.prototype%").
    let proto = ops::get_prototype_from_constructor(agent, new_target, "%Array.prototype%")?;
    // 3. Let numberOfArgs be the number of elements in values.
    match args {
        // 4. If numberOfArgs = 0, then return ! ArrayCreate(0, proto).
        [] => Ok(Value::Object(array_create(agent, 0.0, Some(proto))?)),
        // 5. Else if numberOfArgs = 1, then
        [len] => {
            // b. Let array be ! ArrayCreate(0, proto).
            let array = array_create(agent, 0.0, Some(proto))?;
            let int_len = match *len {
                // d. Else, i. Let intLen be ! ToUint32(len).
                //    ii. If SameValueZero(intLen, len) is false, throw a RangeError exception.
                Value::Number(n) => {
                    let int_len = ops::f64_to_uint32(n);
                    if f64::from(int_len) != n {
                        return Err(agent.throw_range_error("Invalid array length"));
                    }
                    int_len
                }
                // c. If len is not a Number, then
                other => {
                    // i. Perform ! CreateDataPropertyOrThrow(array, "0", len).
                    ops::create_data_property_or_throw(agent, array, PropertyKey::Index(0), other)?;
                    // ii. Let intLen be 1𝔽.
                    1
                }
            };
            // e. Perform ! Set(array, "length", intLen, true).
            let length_key = PropertyKey::String(agent.names.length);
            ops::set(
                agent,
                array,
                length_key,
                Value::Number(f64::from(int_len)),
                true,
            )?;
            // f. Return array.
            Ok(Value::Object(array))
        }
        // 6. Else,
        values => {
            // b. Let array be ? ArrayCreate(numberOfArgs, proto).
            let array = array_create(agent, values.len() as f64, Some(proto))?;
            // c-d. For each k, perform ! CreateDataPropertyOrThrow(array, Pk, itemK).
            for (k, v) in values.iter().enumerate() {
                ops::create_data_property_or_throw(agent, array, PropertyKey::Index(k as u32), *v)?;
            }
            // f. Return array.
            Ok(Value::Object(array))
        }
    }
}
