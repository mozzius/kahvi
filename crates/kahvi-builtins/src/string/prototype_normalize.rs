//! String.prototype.normalize ( [ _form_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.normalize

use kahvi_core::prelude::*;

/// String.prototype.normalize
pub(crate) fn prototype_normalize(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. If _form_ is *undefined*, set _form_ to *"NFC"*.
    // 5. Else, set _form_ to ? ToString(_form_).
    // 6. If _form_ is not one of *"NFC"*, *"NFD"*, *"NFKC"*, or *"NFKD"*, throw a *RangeError* exception.
    // 7. Let _normal_ be the String value that is the result of normalizing _string_ into the normalization form named by _form_ as specified in the latest Unicode Standard, Normalization Forms.
    // 8. Return _normal_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.normalize"))
}
