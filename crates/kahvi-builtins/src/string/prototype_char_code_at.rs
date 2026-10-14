//! String.prototype.charCodeAt ( _position_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.charcodeat

use kahvi_core::prelude::*;

/// String.prototype.charCodeAt
pub(crate) fn prototype_char_code_at(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method returns a Number (a non-negative integral Number less than 2^16) that is the numeric value of the code unit at index _position_ within the String resulting from converting this object to a String. If there is no element at that index, the result is *NaN*.
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Set _position_ to ? ToIntegerOrInfinity(_position_).
    // 5. Let _size_ be the length of _string_.
    // 6. If _position_ < 0 or _position_ ≥ _size_, return *NaN*.
    // 7. Return the Number value for the numeric value of the code unit at index _position_ within the String _string_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore it can be transferred to other kinds of objects for use as a method.
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 4. Set _position_ to ? ToIntegerOrInfinity(_position_).
    let position = ops::to_integer_or_infinity(agent, args.get_or_undefined(0).clone())?;
    // 5. Let _size_ be the length of _string_.
    let size = agent.str_len(string) as f64;
    // 6. If _position_ < 0 or _position_ ≥ _size_, return *NaN*.
    if position < 0.0 || position >= size {
        return Ok(Value::Number(f64::NAN));
    }
    // 7. Return the Number value for the numeric value of the code unit at index _position_ within the String _string_.
    let code_unit = agent.str(string)[position as usize];
    Ok(Value::Number(f64::from(code_unit)))
}
