//! %TypedArray%.prototype.lastIndexOf ( _searchElement_ [ , _fromIndex_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.lastindexof

use kahvi_core::object::typed_array::typed_array_length;
use kahvi_core::prelude::*;

use crate::support::typed_array::{typed_array_get_element, validate_typed_array};

/// ToAbsoluteIndex(_value_, _length_), which is not yet in `ops`.
///
/// Spec: https://tc39.es/ecma262/#sec-toabsoluteindex
fn to_absolute_index(agent: &mut Agent, value: Value, length: f64) -> JsResult<f64> {
    // 1. Let _int_ be ? ToIntegerOrInfinity(_value_).
    let mut int = ops::to_integer_or_infinity(agent, value)?;
    // 2. If _int_ is finite and _int_ < 0, set _int_ to _length_ + _int_.
    if int.is_finite() && int < 0.0 {
        int += length;
    }
    // 3. Return _int_.
    Ok(int)
}

/// TypedArray.prototype.lastIndexOf
pub(crate) fn prototype_last_index_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.lastIndexOf` as defined in [sec-array.prototype.lastindexof].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, obj)?;
    let obj = ta_record.object;
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    let length = typed_array_length(agent, ta_record) as f64;
    // 4. If _length_ = 0, return *-1*_𝔽.
    if length == 0.0 {
        return Ok(Value::Number(-1.0));
    }
    // 5. If _fromIndex_ is not present, let _k_ be _length_ - 1; else let _k_ be min(? ToAbsoluteIndex(_fromIndex_, _length_), _length_ - 1).
    let mut k = if args.len() < 2 {
        length - 1.0
    } else {
        to_absolute_index(agent, args.get_or_undefined(1), length)?.min(length - 1.0)
    };
    // 6. Repeat, while _k_ ≥ 0,
    while k >= 0.0 {
        // a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
        let property_key = agent.key_from_index(k);
        // b. Let _kPresent_ be ! HasProperty(_obj_, _propertyKey_).
        let k_present = ops::has_property(agent, obj, property_key)?;
        // c. If _kPresent_ is *true*, then
        if k_present {
            // i. Let _elementK_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
            let element_k = typed_array_get_element(agent, obj, k);
            // ii. If IsStrictlyEqual(_searchElement_, _elementK_) is *true*, return 𝔽(_k_).
            if ops::is_strictly_equal(agent, args.get_or_undefined(0), element_k) {
                return Ok(Value::Number(k));
            }
        }
        // d. Set _k_ to _k_ - 1.
        k -= 1.0;
    }
    // 7. Return *-1*_𝔽.
    Ok(Value::Number(-1.0))
}
