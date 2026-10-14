//! Array.prototype.unshift ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.unshift

use kahvi_core::prelude::*;

/// Array.prototype.unshift
pub(crate) fn prototype_unshift(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method prepends the arguments to the start of the array, such that their order within the array is the same as the order in which they appear in the argument list.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // The object and the items stay rooted across calls that may collect garbage.
    let mut roots = vec![Value::Object(obj)];
    roots.extend_from_slice(args);
    agent.with_roots(&roots, |agent| {
        // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
        let length = ops::length_of_array_like(agent, obj)?;
        // 3. Let _argCount_ be the number of elements in _items_.
        let arg_count = args.len() as f64;
        // 4. If _argCount_ > 0, then
        if arg_count > 0.0 {
            // a. If _length_ + _argCount_ > 2^53 - 1, throw a *TypeError* exception.
            if length + arg_count > 9007199254740991.0 {
                return Err(agent
                    .throw_type_error("Array.prototype.unshift: array length exceeds 2^53 - 1"));
            }
            // b. Let _k_ be _length_.
            let mut k = length;
            // c. Repeat, while _k_ > 0,
            while k > 0.0 {
                // i. Let _from_ be ! ToString(𝔽(_k_ - 1)).
                let from = agent.key_from_index(k - 1.0);
                // ii. Let _to_ be ! ToString(𝔽(_k_ + _argCount_ - 1)).
                let to = agent.key_from_index(k + arg_count - 1.0);
                // iii. Let _fromPresent_ be ? HasProperty(_obj_, _from_).
                let from_present = ops::has_property(agent, obj, from)?;
                // iv. If _fromPresent_ is *true*, then
                if from_present {
                    // 1. Let _fromValue_ be ? Get(_obj_, _from_).
                    let from_value = ops::get(agent, obj, from)?;
                    // 2. Perform ? Set(_obj_, _to_, _fromValue_, *true*).
                    ops::set(agent, obj, to, from_value, true)?;
                }
                // v. Else,
                else {
                    // 1. Assert: _fromPresent_ is *false*.
                    // 2. Perform ? DeletePropertyOrThrow(_obj_, _to_).
                    ops::delete_property_or_throw(agent, obj, to)?;
                }
                // vi. Set _k_ to _k_ - 1.
                k -= 1.0;
            }
            // d. Let _j_ be 0.
            let mut j = 0.0;
            // e. For each element _item_ of _items_, do
            for item in args {
                // i. Perform ? Set(_obj_, ! ToString(𝔽(_j_)), _item_, *true*).
                let key = agent.key_from_index(j);
                ops::set(agent, obj, key, *item, true)?;
                // ii. Set _j_ to _j_ + 1.
                j += 1.0;
            }
        }
        // 5. Perform ? Set(_obj_, *"length"*, 𝔽(_length_ + _argCount_), *true*).
        let length_key = PropertyKey::String(agent.names.length);
        ops::set(
            agent,
            obj,
            length_key,
            Value::Number(length + arg_count),
            true,
        )?;
        // 6. Return 𝔽(_length_ + _argCount_).
        Ok(Value::Number(length + arg_count))
    })
}
