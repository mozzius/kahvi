//! Array.prototype.reduce ( _callback_ [ , _initialValue_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.reduce

use kahvi_core::prelude::*;

/// Array.prototype.reduce
pub(crate) fn prototype_reduce(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that takes four arguments. `reduce` calls _callback_ once for each element present in the array, in ascending order, skipping the first element unless _initialValue_ is provided.
    // _callback_ is called with four arguments: the _previousValue_ (value from the previous call to _callback_), the _currentValue_ (value of the current element), the _currentIndex_, and the object being traversed. The first time that callback is called, the _previousValue_ and _currentValue_ can be one of two values. If an _initialValue_ was supplied in the call to `reduce`, then _previousValue_ will be _initialValue_ and _currentValue_ will be the first value in the array. If no _initialValue_ was supplied, then _previousValue_ will be the first value in the array and _currentValue_ will be the second. It is a *TypeError* if the array contains no elements and _initialValue_ is not provided.
    // `reduce` does not directly mutate the object on which it is called but the object may be mutated by the calls to _callback_.
    // The range of elements processed by `reduce` is set before the first call to _callback_. Elements that are appended to the array after the call to `reduce` begins will not be visited by _callback_. If existing elements of the array are changed, their value as passed to _callback_ will be the value at the time `reduce` visits them; elements that are deleted after the call to `reduce` begins and before being visited are not visited.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    let length = ops::length_of_array_like(agent, obj)?;
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    let callback = args.get_or_undefined(0);
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("Array.prototype.reduce callback is not a function"));
    }
    // 4. If _length_ = 0 and _initialValue_ is not present, throw a *TypeError* exception.
    let initial_value_present = args.len() >= 2;
    if length == 0.0 && !initial_value_present {
        return Err(
            agent.throw_type_error("Array.prototype.reduce of empty array with no initial value")
        );
    }
    // 5. Let _k_ be 0.
    let mut k = 0.0;
    // 6. Let _accumulator_ be *undefined*.
    let mut accumulator = Value::Undefined;
    // 7. If _initialValue_ is present, then
    if initial_value_present {
        // a. Set _accumulator_ to _initialValue_.
        accumulator = args.get_or_undefined(1).clone();
    } else {
        // 8. Else,
        // a. Let _kPresent_ be *false*.
        let mut k_present = false;
        // b. Repeat, while _kPresent_ is *false* and _k_ < _length_,
        while !k_present && k < length {
            // i. Let _propertyKey_ be ! ToString(𝔽(_k_)).
            let property_key = agent.key_from_index(k);
            // ii. Set _kPresent_ to ? HasProperty(_obj_, _propertyKey_).
            k_present = ops::has_property(agent, obj, property_key)?;
            // iii. If _kPresent_ is *true*, then
            if k_present {
                // 1. Set _accumulator_ to ? Get(_obj_, _propertyKey_).
                accumulator = ops::get(agent, obj, property_key)?;
            }
            // iv. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // c. If _kPresent_ is *false*, throw a *TypeError* exception.
        if !k_present {
            return Err(agent
                .throw_type_error("Array.prototype.reduce of empty array with no initial value"));
        }
    }
    // The callback may collect garbage: what is used after it stays rooted.
    agent.with_roots(&[Value::Object(obj), callback], |agent| {
        // 9. Repeat, while _k_ < _length_,
        while k < length {
            // a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
            let property_key = agent.key_from_index(k);
            // b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
            let k_present = ops::has_property(agent, obj, property_key)?;
            // c. If _kPresent_ is *true*, then
            if k_present {
                // i. Let _kValue_ be ? Get(_obj_, _propertyKey_).
                let k_value = ops::get(agent, obj, property_key)?;
                // ii. Set _accumulator_ to ? Call(_callback_, *undefined*, « _accumulator_, _kValue_, 𝔽(_k_), _obj_ »).
                let mark = agent.push_root(accumulator);
                agent.push_root(k_value);
                accumulator = ops::call_rooted(
                    agent,
                    callback,
                    Value::Undefined,
                    &[accumulator, k_value, Value::Number(k), Value::Object(obj)],
                )?;
                agent.pop_roots(mark);
            }
            // d. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // 10. Return _accumulator_.
        Ok(accumulator)
    })
}
