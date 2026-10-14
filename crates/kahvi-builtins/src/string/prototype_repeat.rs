//! String.prototype.repeat ( _count_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.repeat

use kahvi_core::prelude::*;

/// The longest String this engine will build, in code units. The spec leaves
/// the limit to the implementation; a longer result throws a RangeError
/// instead of exhausting memory.
const MAX_STRING_LENGTH: f64 = ((1u64 << 29) - 24) as f64;

/// String.prototype.repeat
pub(crate) fn prototype_repeat(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // The callback-free conversions below may collect garbage: _string_ stays rooted.
    agent.with_roots(&[Value::String(string)], |agent| {
        // 4. Let _n_ be ? ToIntegerOrInfinity(_count_).
        let count = args.get_or_undefined(0);
        let n = ops::to_integer_or_infinity(agent, count)?;
        // 5. If _n_ < 0 or _n_ = +∞, throw a *RangeError* exception.
        if n < 0.0 || n == f64::INFINITY {
            return Err(agent.throw_range_error(
                "String.prototype.repeat count must be a finite non-negative number",
            ));
        }
        // 6. If _n_ = 0, return the empty String.
        if n == 0.0 {
            return Ok(Value::String(agent.intern(&[])));
        }
        // Engine limit: refuse results too long to represent, rather than abort.
        let string_len = agent.str_len(string) as f64;
        if string_len * n > MAX_STRING_LENGTH {
            return Err(agent.throw_range_error("Invalid string length"));
        }
        // Copies of the empty String concatenate to the empty String, whatever _n_ is.
        if string_len == 0.0 {
            return Ok(Value::String(agent.intern(&[])));
        }
        // 7. Return the String value that is made from _n_ copies of _string_ appended together.
        let units = agent.str(string).to_vec();
        let copies = n as usize;
        let mut result: Vec<u16> = Vec::with_capacity(units.len() * copies);
        for _ in 0..copies {
            result.extend_from_slice(&units);
        }
        Ok(Value::String(agent.intern(&result)))
    })
}
