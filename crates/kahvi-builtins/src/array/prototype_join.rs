//! Array.prototype.join ( _separator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.join

use kahvi_core::prelude::*;

/// Array.prototype.join
pub(crate) fn prototype_join(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 2. Let len be ? LengthOfArrayLike(O).
    let len = ops::length_of_array_like(agent, o)?;
    // 3. If separator is undefined, let sep be ",".
    // 4. Else, let sep be ? ToString(separator).
    let sep = match args.get_or_undefined(0) {
        Value::Undefined => vec![u16::from(b',')],
        s => {
            let s = ops::to_string(agent, s)?;
            agent.str(s).to_vec()
        }
    };
    // 5. Let R be the empty String.
    let mut r: Vec<u16> = Vec::new();
    // 6. Let k be 0.
    let mut k = 0.0;
    // 7. Repeat, while k < len,
    while k < len {
        // a. If k > 0, set R to the string-concatenation of R and sep.
        if k > 0.0 {
            r.extend_from_slice(&sep);
        }
        // b. Let element be ? Get(O, ! ToString(𝔽(k))).
        let key = agent.key_from_index(k);
        let element = ops::get(agent, o, key)?;
        // c. If element is either undefined or null, let next be the empty String; otherwise, let next be ? ToString(element).
        if !element.is_nullish() {
            let next = ops::to_string(agent, element)?;
            // d. Set R to the string-concatenation of R and next.
            r.extend_from_slice(agent.str(next));
        }
        // e. Set k to k + 1.
        k += 1.0;
    }
    // 8. Return R.
    Ok(Value::String(agent.intern(&r)))
}
