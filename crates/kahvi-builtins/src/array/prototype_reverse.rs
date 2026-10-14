//! Array.prototype.reverse ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.reverse

use kahvi_core::prelude::*;

/// Array.prototype.reverse
pub(crate) fn prototype_reverse(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method rearranges the elements of the array so as to reverse their order. It returns the reversed array.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // The loop may run user code and collect garbage: obj stays rooted.
    agent.with_roots(&[Value::Object(obj)], |agent| {
        // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
        let length = ops::length_of_array_like(agent, obj)?;
        // 3. Let _middle_ be floor(_length_ / 2).
        let middle = (length / 2.0).floor();
        // 4. Let _lower_ be 0.
        let mut lower = 0.0;
        // 5. Repeat, while _lower_ ≠ _middle_,
        while lower != middle {
            // a. Let _upper_ be _length_ - _lower_ - 1.
            let upper = length - lower - 1.0;
            // b. Let _upperP_ be ! ToString(𝔽(_upper_)).
            let upper_p = agent.key_from_index(upper);
            // c. Let _lowerP_ be ! ToString(𝔽(_lower_)).
            let lower_p = agent.key_from_index(lower);
            // The keys and the values read below stay rooted for this iteration.
            let mark = agent.push_root_key(upper_p);
            agent.push_root_key(lower_p);
            // d. Let _lowerExists_ be ? HasProperty(_obj_, _lowerP_).
            let lower_exists = ops::has_property(agent, obj, lower_p)?;
            // e. If _lowerExists_ is *true*, then
            let lower_value = if lower_exists {
                // i. Let _lowerValue_ be ? Get(_obj_, _lowerP_).
                ops::get(agent, obj, lower_p)?
            } else {
                Value::Undefined
            };
            agent.push_root(lower_value);
            // f. Let _upperExists_ be ? HasProperty(_obj_, _upperP_).
            let upper_exists = ops::has_property(agent, obj, upper_p)?;
            // g. If _upperExists_ is *true*, then
            let upper_value = if upper_exists {
                // i. Let _upperValue_ be ? Get(_obj_, _upperP_).
                ops::get(agent, obj, upper_p)?
            } else {
                Value::Undefined
            };
            agent.push_root(upper_value);
            // h. If _lowerExists_ is *true* and _upperExists_ is *true*, then
            if lower_exists && upper_exists {
                // i. Perform ? Set(_obj_, _lowerP_, _upperValue_, *true*).
                ops::set(agent, obj, lower_p, upper_value, true)?;
                // ii. Perform ? Set(_obj_, _upperP_, _lowerValue_, *true*).
                ops::set(agent, obj, upper_p, lower_value, true)?;
            }
            // i. Else if _lowerExists_ is *false* and _upperExists_ is *true*, then
            else if !lower_exists && upper_exists {
                // i. Perform ? Set(_obj_, _lowerP_, _upperValue_, *true*).
                ops::set(agent, obj, lower_p, upper_value, true)?;
                // ii. Perform ? DeletePropertyOrThrow(_obj_, _upperP_).
                ops::delete_property_or_throw(agent, obj, upper_p)?;
            }
            // j. Else if _lowerExists_ is *true* and _upperExists_ is *false*, then
            else if lower_exists && !upper_exists {
                // i. Perform ? DeletePropertyOrThrow(_obj_, _lowerP_).
                ops::delete_property_or_throw(agent, obj, lower_p)?;
                // ii. Perform ? Set(_obj_, _upperP_, _lowerValue_, *true*).
                ops::set(agent, obj, upper_p, lower_value, true)?;
            }
            // k. Else,
            else {
                // i. Assert: _lowerExists_ and _upperExists_ are both *false*.
                // ii. NOTE: No action is required.
            }
            agent.pop_roots(mark);
            // l. Set _lower_ to _lower_ + 1.
            lower += 1.0;
        }
        // 6. Return _obj_.
        Ok(Value::Object(obj))
    })
}
