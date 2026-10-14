//! %TypedArray%.prototype.join ( _separator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.join

use kahvi_core::object::typed_array::{typed_array_get_element, typed_array_length};
use kahvi_core::prelude::*;

use crate::support::typed_array::validate_typed_array;

/// %TypedArray%.prototype.join
pub(crate) fn prototype_join(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.join` as defined in [sec-array.prototype.join].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, this)?;
    let obj = ta_record.object;
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    let length = typed_array_length(agent, ta_record) as f64;
    // 4. If _separator_ is *undefined*, let _separatorString_ be *","*.
    // 5. Else, let _separatorString_ be ? ToString(_separator_).
    let separator = args.get_or_undefined(0);
    let separator_string: Vec<u16> = match separator {
        Value::Undefined => vec![u16::from(b',')],
        s => {
            let s = ops::to_string(agent, s)?;
            agent.str(s).to_vec()
        }
    };
    // 6. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::new();
    // 7. Let _k_ be 0.
    let mut k = 0.0;
    // 8. Repeat, while _k_ < _length_,
    while k < length {
        // a. If _k_ > 0, set _result_ to the string-concatenation of _result_ and _separatorString_.
        if k > 0.0 {
            result.extend_from_slice(&separator_string);
        }
        // b. Let _element_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
        let element = typed_array_get_element(agent, obj, k);
        // c. If _element_ is not *undefined*, then
        if !element.is_undefined() {
            // i. Let _elementString_ be ! ToString(_element_).
            let element_string = ops::to_string(agent, element)?;
            // ii. Set _result_ to the string-concatenation of _result_ and _elementString_.
            result.extend_from_slice(agent.str(element_string));
        }
        // d. Set _k_ to _k_ + 1.
        k += 1.0;
    }
    // 9. Return _result_.
    Ok(Value::String(agent.intern(&result)))
}
