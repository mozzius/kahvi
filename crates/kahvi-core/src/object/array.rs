//! Array exotic objects (ECMA-262 §10.4.2).

use super::internal::{ordinary_define_own_property, ordinary_get_own_property};
use super::{ObjectData, ObjectKind, Property, PropertyDescriptor, PropertyKey};
use crate::agent::Agent;
use crate::error::JsResult;
use crate::ops;
use crate::value::{Object, Value};

/// ArrayCreate(length [, proto]) (§10.4.2.2)
pub fn array_create(agent: &mut Agent, length: f64, proto: Option<Object>) -> JsResult<Object> {
    // 1. If length > 2^32 - 1, throw a RangeError exception.
    if length > 4_294_967_295.0 {
        return Err(agent.throw_range_error("invalid array length"));
    }
    // 2. If proto is not present, set proto to %Array.prototype%.
    let proto = match proto {
        Some(p) => p,
        None => agent.intrinsic_c("%Array.prototype%"),
    };
    // 3. Let A be MakeBasicObject(« [[Prototype]], [[Extensible]] »).
    // 4. Set A.[[Prototype]] to proto.
    // 5. Set A.[[DefineOwnProperty]] as specified in 10.4.2.1.
    let a = agent.alloc(ObjectData::new(Some(proto), ObjectKind::Array));
    // 6. Perform ! OrdinaryDefineOwnProperty(A, "length", PropertyDescriptor { [[Value]]: 𝔽(length),
    //    [[Writable]]: true, [[Enumerable]]: false, [[Configurable]]: false }).
    let key = PropertyKey::String(agent.names.length);
    agent.obj_mut(a).properties.insert(
        key,
        Property::Data {
            value: Value::Number(length),
            writable: true,
            enumerable: false,
            configurable: false,
        },
    );
    // 7. Return A.
    Ok(a)
}

/// The current value of an array's "length" (always a valid uint32 number).
pub fn array_length(agent: &Agent, a: Object) -> u32 {
    let key = PropertyKey::String(agent.names.length);
    match agent.obj(a).properties.value_of(&key, None) {
        Some(Some(Value::Number(n))) => n as u32,
        _ => 0,
    }
}

fn length_writable(agent: &Agent, a: Object) -> bool {
    let key = PropertyKey::String(agent.names.length);
    matches!(
        agent.obj(a).properties.get(&key),
        Some(Property::Data { writable: true, .. })
    )
}

/// A.[[DefineOwnProperty]](P, Desc) for arrays (§10.4.2.1)
pub fn array_define_own_property(
    agent: &mut Agent,
    a: Object,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> JsResult<bool> {
    // 1. If P is "length", then
    if key == PropertyKey::String(agent.names.length) {
        // a. Return ? ArraySetLength(A, Desc).
        return array_set_length(agent, a, desc);
    }
    // 2. Else if P is an array index, then
    if let PropertyKey::Index(index) = key {
        // a. Let lengthDesc be OrdinaryGetOwnProperty(A, "length").
        // c. Let length be lengthDesc.[[Value]].
        let length = array_length(agent, a);
        // e. If index ≥ length and lengthDesc.[[Writable]] is false, return false.
        if index >= length && !length_writable(agent, a) {
            return Ok(false);
        }
        // f. Let succeeded be ! OrdinaryDefineOwnProperty(A, P, Desc).
        let succeeded = ordinary_define_own_property(agent, a, key, desc)?;
        // g. If succeeded is false, return false.
        if !succeeded {
            return Ok(false);
        }
        // h. If index ≥ length, then
        if index >= length {
            // i. Set lengthDesc.[[Value]] to index + 1𝔽.
            // ii. Set succeeded to ! OrdinaryDefineOwnProperty(A, "length", lengthDesc).
            let len_key = PropertyKey::String(agent.names.length);
            if let Some((value, _)) = agent.obj_mut(a).properties.data_value_mut(&len_key) {
                *value = Value::Number(index as f64 + 1.0);
            }
        }
        // i. Return true.
        return Ok(true);
    }
    // 3. Return ? OrdinaryDefineOwnProperty(A, P, Desc).
    ordinary_define_own_property(agent, a, key, desc)
}

/// ArraySetLength(A, Desc) (§10.4.2.4)
pub fn array_set_length(agent: &mut Agent, a: Object, desc: PropertyDescriptor) -> JsResult<bool> {
    let len_key = PropertyKey::String(agent.names.length);
    // 1. If Desc does not have a [[Value]] field, then
    let Some(value) = desc.value else {
        // a. Return ! OrdinaryDefineOwnProperty(A, "length", Desc).
        return ordinary_define_own_property(agent, a, len_key, desc);
    };
    // 2. Let newLenDesc be a copy of Desc.
    let mut new_len_desc = desc;
    // 3. Let newLen be ? ToUint32(Desc.[[Value]]).
    let new_len = ops::to_uint32(agent, value)?;
    // 4. Let numberLen be ? ToNumber(Desc.[[Value]]).
    let number_len = ops::to_number(agent, value)?;
    // 5. If SameValueZero(newLen, numberLen) is false, throw a RangeError exception.
    if new_len as f64 != number_len {
        return Err(agent.throw_range_error("invalid array length"));
    }
    // 6. Set newLenDesc.[[Value]] to newLen.
    new_len_desc.value = Some(Value::Number(new_len as f64));
    // 7. Let oldLenDesc be OrdinaryGetOwnProperty(A, "length").
    let old_len_desc = ordinary_get_own_property(agent, a, len_key).unwrap_or_default();
    // 10. Let oldLen be oldLenDesc.[[Value]].
    let old_len = array_length(agent, a);
    // 11. If newLen ≥ oldLen, then
    if new_len >= old_len {
        // a. Return ! OrdinaryDefineOwnProperty(A, "length", newLenDesc).
        return ordinary_define_own_property(agent, a, len_key, new_len_desc);
    }
    // 12. If oldLenDesc.[[Writable]] is false, return false.
    if old_len_desc.writable == Some(false) {
        return Ok(false);
    }
    // 13. If newLenDesc does not have a [[Writable]] field or newLenDesc.[[Writable]] is true, then
    let new_writable = if new_len_desc.writable.is_none() || new_len_desc.writable == Some(true) {
        // a. Let newWritable be true.
        true
    } else {
        // 14. Else, a. NOTE: Setting [[Writable]] to false is deferred.
        //           b. Let newWritable be false.
        //           c. Set newLenDesc.[[Writable]] to true.
        new_len_desc.writable = Some(true);
        false
    };
    // 15. Let succeeded be ! OrdinaryDefineOwnProperty(A, "length", newLenDesc).
    // 16. If succeeded is false, return false.
    if !ordinary_define_own_property(agent, a, len_key, new_len_desc)? {
        return Ok(false);
    }
    // 17. For each own property key P of A such that P is an array index and
    //     ! ToUint32(P) ≥ newLen, in descending numeric index order, do
    for index in agent.obj(a).properties.index_keys_from(new_len) {
        // a. Let deleteSucceeded be ! A.[[Delete]](P).
        let delete_succeeded = a.internal_delete(agent, PropertyKey::Index(index))?;
        // b. If deleteSucceeded is false, then
        if !delete_succeeded {
            // i. Set newLenDesc.[[Value]] to ! ToUint32(P) + 1𝔽.
            new_len_desc.value = Some(Value::Number(index as f64 + 1.0));
            // ii. If newWritable is false, set newLenDesc.[[Writable]] to false.
            if !new_writable {
                new_len_desc.writable = Some(false);
            }
            // iii. Perform ! OrdinaryDefineOwnProperty(A, "length", newLenDesc).
            ordinary_define_own_property(agent, a, len_key, new_len_desc)?;
            // iv. Return false.
            return Ok(false);
        }
    }
    // 18. If newWritable is false, then
    if !new_writable {
        // a. Set succeeded to ! OrdinaryDefineOwnProperty(A, "length",
        //    PropertyDescriptor { [[Writable]]: false }).
        let d = PropertyDescriptor {
            writable: Some(false),
            ..Default::default()
        };
        ordinary_define_own_property(agent, a, len_key, d)?;
    }
    // 19. Return true.
    Ok(true)
}
