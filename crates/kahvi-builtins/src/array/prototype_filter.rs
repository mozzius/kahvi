//! Array.prototype.filter ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.filter

use kahvi_core::prelude::*;

/// Array.prototype.filter
pub(crate) fn prototype_filter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that accepts three arguments and returns a value that is coercible to a Boolean value. `filter` calls _callback_ once for each element in the array, in ascending order, and constructs a new array of all the values for which _callback_ returns *true*. _callback_ is called only for elements of the array which actually exist; it is not called for missing elements of the array.
    // If a _thisArg_ parameter is provided, it will be used as the *this* value for each invocation of _callback_. If it is not provided, *undefined* is used instead.
    // _callback_ is called with three arguments: the value of the element, the index of the element, and the object being traversed.
    // `filter` does not directly mutate the object on which it is called but the object may be mutated by the calls to _callback_.
    // The range of elements processed by `filter` is set before the first call to _callback_. Elements which are appended to the array after the call to `filter` begins will not be visited by _callback_. If existing elements of the array are changed their value as passed to _callback_ will be the value at the time `filter` visits them; elements that are deleted after the call to `filter` begins and before being visited are not visited.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 4. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
    // 5. Let _k_ be 0.
    // 6. Let _to_ be 0.
    // 7. Repeat, while _k_ < _length_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
    //    c. If _kPresent_ is *true*, then
    //       i. Let _kValue_ be ? Get(_obj_, _propertyKey_).
    //       ii. Let _selected_ be ToBoolean(? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »)).
    //       iii. If _selected_ is *true*, then
    //          1. Perform ? CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_to_)), _kValue_).
    //          2. Set _to_ to _to_ + 1.
    //    d. Set _k_ to _k_ + 1.
    // 8. Return _array_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.

    // 1. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 2. Let len be ? LengthOfArrayLike(O).
    let len = ops::length_of_array_like(agent, o)?;
    // 3. If IsCallable(callback) is false, throw a TypeError exception.
    let callback = args.get_or_undefined(0);
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("Array.prototype.filter callback is not a function"));
    }
    let this_arg = args.get_or_undefined(1);
    // 4. Let A be ? ArraySpeciesCreate(O, 0).
    let a = ops::array_species_create(agent, o, 0.0)?;
    // The callback may collect garbage: what is used after it stays rooted.
    agent.with_roots(
        &[Value::Object(o), callback, this_arg, Value::Object(a)],
        |agent| {
            // 5. Let k be 0.
            let mut k = 0.0;
            // 6. Let to be 0.
            let mut to = 0.0;
            // 7. Repeat, while k < len,
            while k < len {
                // a. Let Pk be ! ToString(𝔽(k)).
                let pk = agent.key_from_index(k);
                // b. Let kPresent be ? HasProperty(O, Pk).
                // c. If kPresent is true, then
                if ops::has_property(agent, o, pk)? {
                    // i. Let kValue be ? Get(O, Pk).
                    let k_value = ops::get(agent, o, pk)?;
                    // ii. Let selected be ToBoolean(? Call(callback, thisArg, « kValue, 𝔽(k), O »)).
                    let mark = agent.push_root(k_value);
                    let selected = ops::call_rooted(
                        agent,
                        callback,
                        this_arg,
                        &[k_value, Value::Number(k), Value::Object(o)],
                    )?;
                    // iii. If selected is true, then
                    if ops::to_boolean(agent, selected) {
                        // 1. Perform ? CreateDataPropertyOrThrow(A, ! ToString(𝔽(to)), kValue).
                        let to_key = agent.key_from_index(to);
                        ops::create_data_property_or_throw(agent, a, to_key, k_value)?;
                        // 2. Set to to to + 1.
                        to += 1.0;
                    }
                    agent.pop_roots(mark);
                }
                // d. Set k to k + 1.
                k += 1.0;
            }
            // 8. Return A.
            Ok(Value::Object(a))
        },
    )
}
