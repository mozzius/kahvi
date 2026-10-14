//! String.prototype.isWellFormed ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.iswellformed

use kahvi_core::prelude::*;

/// String.prototype.isWellFormed
pub(crate) fn prototype_is_well_formed(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
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
    // 4. Return IsStringWellFormedUnicode(_string_).
    Ok(Value::Boolean(is_string_well_formed_unicode(
        agent.str(string),
    )))
}

/// IsStringWellFormedUnicode ( _string_ )
///
/// The abstract operation IsStringWellFormedUnicode takes argument _string_
/// (a String) and returns a Boolean. It interprets _string_ as a sequence of
/// UTF-16 code units and reports whether it contains any lone surrogates.
fn is_string_well_formed_unicode(units: &[u16]) -> bool {
    // 1. Let strLen be the length of _string_.
    let str_len = units.len();
    // 2. Let k be 0.
    let mut k = 0;
    // 3. Repeat, while k < strLen,
    while k < str_len {
        // a. Let cp be CodePointAt(_string_, k).
        // b. If cp.[[IsUnpairedSurrogate]] is true, return false.
        let unit = units[k];
        if (0xD800..=0xDBFF).contains(&unit) {
            // A high surrogate is only paired when a low surrogate follows it.
            match units.get(k + 1) {
                Some(&next) if (0xDC00..=0xDFFF).contains(&next) => {
                    // c. Set k to k + cp.[[CodeUnitCount]].
                    k += 2;
                    continue;
                }
                _ => return false,
            }
        }
        if (0xDC00..=0xDFFF).contains(&unit) {
            // A low surrogate with no preceding high surrogate is unpaired.
            return false;
        }
        // c. Set k to k + cp.[[CodeUnitCount]].
        k += 1;
    }
    // 4. Return true.
    true
}
