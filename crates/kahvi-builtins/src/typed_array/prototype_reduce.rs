//! %TypedArray%.prototype.reduce ( _callback_ [ , _initialValue_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.reduce

use kahvi_core::prelude::*;

use crate::support::typed_array::{
    length as typed_array_length, typed_array_get_element, validate_typed_array,
};

/// TypedArray.prototype.reduce
pub(crate) fn prototype_reduce(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.reduce` as defined in [sec-array.prototype.reduce].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, obj)?;
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    let len = typed_array_length(agent, ta_record) as f64;
    let o = ta_record.object;
    let callback = args.get_or_undefined(0);
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, callback) {
        return Err(
            agent.throw_type_error("TypedArray.prototype.reduce callback is not a function")
        );
    }
    let initial_value = args.get(1).copied();
    // 5. If _length_ = 0 and _initialValue_ is not present, throw a *TypeError* exception.
    if len == 0.0 && initial_value.is_none() {
        return Err(agent.throw_type_error("Reduce of empty array with no initial value"));
    }
    // The callback may collect garbage: O and callback stay rooted.
    agent.with_roots(&[Value::Object(o), callback], |agent| {
        // 6. Let _k_ be 0.
        let mut k = 0.0;
        // 7. Let _accumulator_ be *undefined*.
        // 8. If _initialValue_ is present, then
        let mut accumulator = if let Some(initial_value) = initial_value {
            // a. Set _accumulator_ to _initialValue_.
            initial_value
        } else {
            // 9. Else,
            // a. Set _accumulator_ to TypedArrayGetElement(_obj_, 𝔽(_k_)).
            let first = typed_array_get_element(agent, o, k);
            // b. Set _k_ to _k_ + 1.
            k += 1.0;
            first
        };
        // 10. Repeat, while _k_ < _length_,
        while k < len {
            // a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
            let k_value = typed_array_get_element(agent, o, k);
            // b. Set _accumulator_ to ? Call(_callback_, *undefined*, « _accumulator_, _kValue_, 𝔽(_k_), _obj_ »).
            let mark = agent.push_root(accumulator);
            agent.push_root(k_value);
            let result = ops::call_rooted(
                agent,
                callback,
                Value::Undefined,
                &[accumulator, k_value, Value::Number(k), Value::Object(o)],
            );
            agent.pop_roots(mark);
            accumulator = result?;
            // c. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // 11. Return _accumulator_.
        Ok(accumulator)
    })
}
