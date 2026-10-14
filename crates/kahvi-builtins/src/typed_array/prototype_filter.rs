//! %TypedArray%.prototype.filter ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.filter

use kahvi_core::object::typed_array::typed_array_length;
use kahvi_core::prelude::*;

use crate::support::typed_array::{
    typed_array_get_element, typed_array_set_element, typed_array_species_create,
    validate_typed_array,
};

/// TypedArray.prototype.filter
pub(crate) fn prototype_filter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.filter` as defined in [sec-array.prototype.filter].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, obj)?;
    let Value::Object(obj) = obj else {
        unreachable!("ValidateTypedArray accepted a non-object")
    };
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    let length = typed_array_length(agent, ta_record);
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    let callback = args.get_or_undefined(0);
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("callback is not a function"));
    }
    let this_arg = args.get_or_undefined(1);
    // 5. Let _kept_ be a new empty List.
    let mut kept: Vec<Value> = Vec::new();
    // 6. Let _captured_ be 0.
    let mut captured: usize = 0;
    // 7. Let _k_ be 0.
    let mut k: usize = 0;
    // 8. Repeat, while _k_ < _length_,
    while k < length {
        // a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
        let k_value = typed_array_get_element(agent, obj, k as f64);
        // b. Let _selected_ be ToBoolean(? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »)).
        let call_result = ops::call(
            agent,
            callback,
            this_arg,
            &[k_value, Value::Number(k as f64), Value::Object(obj)],
        )?;
        let selected = ops::to_boolean(agent, call_result);
        // c. If _selected_ is *true*, then
        if selected {
            // i. Append _kValue_ to _kept_.
            kept.push(k_value);
            // ii. Set _captured_ to _captured_ + 1.
            captured += 1;
        }
        // d. Set _k_ to _k_ + 1.
        k += 1;
    }
    // 9. Let _result_ be ? TypedArraySpeciesCreate(_obj_, « 𝔽(_captured_) »).
    let result = typed_array_species_create(agent, obj, &[Value::Number(captured as f64)])?;
    // 10. Let _n_ be 0.
    let mut n: usize = 0;
    // 11. For each element _element_ of _kept_, do
    for element in kept {
        // a. Perform ! TypedArraySetElement(_result_, 𝔽(_n_), _element_).
        typed_array_set_element(agent, result, n as f64, element)?;
        // b. Set _n_ to _n_ + 1.
        n += 1;
    }
    // 12. Return _result_.
    Ok(Value::Object(result))
}
