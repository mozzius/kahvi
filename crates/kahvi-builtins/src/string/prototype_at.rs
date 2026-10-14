//! String.prototype.at ( _index_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.at

use kahvi_core::prelude::*;

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

/// String.prototype.at
pub(crate) fn prototype_at(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let index = args.get_or_undefined(0);
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // The conversions below may collect garbage: string and index stay rooted.
    agent.with_roots(&[Value::String(string), index], |agent| {
        // 4. Let _length_ be the length of _string_.
        let length = agent.str_len(string) as f64;
        // 5. Let _k_ be ? ToAbsoluteIndex(_index_, _length_).
        let k = to_absolute_index(agent, index, length)?;
        // 6. If _k_ < 0 or _k_ ≥ _length_, return *undefined*.
        if k < 0.0 || k >= length {
            return Ok(Value::Undefined);
        }
        // 7. Return the substring of _string_ from _k_ to _k_ + 1.
        let k = k as usize;
        Ok(Value::String(agent.substring(string, k, k + 1)))
    })
}
