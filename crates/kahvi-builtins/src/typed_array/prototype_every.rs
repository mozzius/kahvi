//! %TypedArray%.prototype.every ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.every

use kahvi_core::object::typed_array::typed_array_length;
use kahvi_core::prelude::*;

use crate::support::typed_array::{typed_array_get_element, validate_typed_array};

/// TypedArray.prototype.every
pub(crate) fn prototype_every(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.every` as defined in [sec-array.prototype.every].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, this)?;
    let obj = ta_record.object;
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    let length = typed_array_length(agent, ta_record);
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    let callback = args.get_or_undefined(0);
    if !ops::is_callable(agent, callback) {
        return Err(agent.throw_type_error("TypedArray.prototype.every callback is not a function"));
    }
    let this_arg = args.get_or_undefined(1);
    // The callback may collect garbage: obj, callback and thisArg stay rooted.
    agent.with_roots(&[Value::Object(obj), callback, this_arg], |agent| {
        // 5. Let _k_ be 0.
        let mut k: usize = 0;
        // 6. Repeat, while _k_ < _length_,
        while k < length {
            // a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
            let k_value = typed_array_get_element(agent, obj, k as f64);
            // b. Let _testResult_ be ToBoolean(? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »)).
            let mark = agent.push_root(k_value);
            let call_result = ops::call_rooted(
                agent,
                callback,
                this_arg,
                &[k_value, Value::Number(k as f64), Value::Object(obj)],
            )?;
            agent.pop_roots(mark);
            let test_result = ops::to_boolean(agent, call_result);
            // c. If _testResult_ is *false*, return *false*.
            if !test_result {
                return Ok(Value::Boolean(false));
            }
            // d. Set _k_ to _k_ + 1.
            k += 1;
        }
        // 7. Return *true*.
        Ok(Value::Boolean(true))
    })
}
