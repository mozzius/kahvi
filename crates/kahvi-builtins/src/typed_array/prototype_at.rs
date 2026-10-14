//! %TypedArray%.prototype.at ( _index_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.at

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

/// TypedArray.prototype.at
pub(crate) fn prototype_at(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, obj)?;
    let obj = ta_record.object;
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    let length = typed_array_length(agent, ta_record) as f64;
    // 4. Let _k_ be ? ToAbsoluteIndex(_index_, _length_).
    let k = to_absolute_index(agent, args.get_or_undefined(0), length)?;
    // 5. If _k_ < 0 or _k_ ≥ _length_, return *undefined*.
    if k < 0.0 || k >= length {
        return Ok(Value::Undefined);
    }
    // 6. Return TypedArrayGetElement(_obj_, 𝔽(_k_)).
    Ok(typed_array_get_element(agent, obj, k))
}
