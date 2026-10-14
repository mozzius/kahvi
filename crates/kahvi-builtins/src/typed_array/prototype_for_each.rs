//! %TypedArray%.prototype.forEach ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.foreach

use kahvi_core::prelude::*;

use crate::support::typed_array::{
    length as typed_array_length, typed_array_get_element, validate_typed_array,
};

/// TypedArray.prototype.forEach
pub(crate) fn prototype_for_each(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.forEach` as defined in [sec-array.prototype.foreach].
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
    let this_arg = args.get_or_undefined(1);
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, callback) {
        return Err(
            agent.throw_type_error("TypedArray.prototype.forEach callback is not a function")
        );
    }
    // The callback may collect garbage: O, callback and thisArg stay rooted.
    agent.with_roots(&[Value::Object(o), callback, this_arg], |agent| {
        // 5. Let _k_ be 0.
        let mut k = 0.0;
        // 6. Repeat, while _k_ < _length_,
        while k < len {
            // a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
            let k_value = typed_array_get_element(agent, o, k);
            // b. Perform ? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »).
            ops::call_rooted(
                agent,
                callback,
                this_arg,
                &[k_value, Value::Number(k), Value::Object(o)],
            )?;
            // c. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // 7. Return *undefined*.
        Ok(Value::Undefined)
    })
}
