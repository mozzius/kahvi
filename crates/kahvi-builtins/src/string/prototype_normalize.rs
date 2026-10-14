//! String.prototype.normalize ( [ _form_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.normalize

use kahvi_core::prelude::*;
use kahvi_unicode::{NormalizationForm, normalize_code_points};

/// String.prototype.normalize
pub(crate) fn prototype_normalize(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this)?;
    let form = args.get_or_undefined(0);
    // 4. If _form_ is *undefined*, set _form_ to *"NFC"*.
    // 5. Else, set _form_ to ? ToString(_form_).
    let form = if form.is_undefined() {
        NormalizationForm::Nfc
    } else {
        let f = ops::to_string(agent, form)?;
        // 6. If _form_ is not one of *"NFC"*, *"NFD"*, *"NFKC"*, or *"NFKD"*, throw a *RangeError* exception.
        match String::from_utf16_lossy(agent.str(f)).as_str() {
            "NFC" if agent.str(f).len() == 3 => NormalizationForm::Nfc,
            "NFD" if agent.str(f).len() == 3 => NormalizationForm::Nfd,
            "NFKC" if agent.str(f).len() == 4 => NormalizationForm::Nfkc,
            "NFKD" if agent.str(f).len() == 4 => NormalizationForm::Nfkd,
            _ => {
                return Err(agent.throw_range_error(
                    "The normalization form should be one of NFC, NFD, NFKC, NFKD.",
                ));
            }
        }
    };
    // 7. Let _normal_ be the String value that is the result of normalizing _string_ into the normalization form named by _form_ as specified in the latest Unicode Standard, Normalization Forms.
    let code_points = to_code_points(agent.str(string));
    let normal = from_code_points(&normalize_code_points(&code_points, form));
    // 8. Return _normal_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore it can be transferred to other kinds of objects for use as a method.
    Ok(Value::String(agent.intern(&normal)))
}

/// The code points of a string (CodePointAt over each position; a lone
/// surrogate stands for itself).
fn to_code_points(units: &[u16]) -> Vec<u32> {
    char::decode_utf16(units.iter().copied())
        .map(|r| match r {
            Ok(c) => c as u32,
            Err(e) => u32::from(e.unpaired_surrogate()),
        })
        .collect()
}

/// UTF16EncodeCodePoint over each code point.
fn from_code_points(code_points: &[u32]) -> Vec<u16> {
    let mut out = Vec::with_capacity(code_points.len());
    for &cp in code_points {
        match char::from_u32(cp) {
            Some(c) => {
                let mut buf = [0u16; 2];
                out.extend_from_slice(c.encode_utf16(&mut buf));
            }
            // A lone surrogate.
            None => out.push(cp as u16),
        }
    }
    out
}
