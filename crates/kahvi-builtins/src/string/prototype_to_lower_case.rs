//! String.prototype.toLowerCase ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.tolowercase

use kahvi_core::prelude::*;

/// String.prototype.toLowerCase
pub(crate) fn prototype_to_lower_case(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _sText_ be StringToCodePoints(_string_).
    // 5. Let _lowerText_ be toLowercase(_sText_), according to the Unicode Default Case Conversion algorithm.
    // 6. Let _lowercaseString_ be CodePointsToString(_lowerText_).
    // 7. Return _lowercaseString_.
    //
    // The result must be derived according to the locale-insensitive case mappings in the Unicode Character Database (this explicitly includes not only the file UnicodeData.txt, but also all locale-insensitive mappings in the file SpecialCasing.txt that accompanies it).
    // NOTE:
    // The case mapping of some code points may produce multiple code points. In this case the result String may not be the same length as the source String. Because both `toUpperCase` and `toLowerCase` have context-sensitive behaviour, the methods are not symmetrical. In other words, `s.toUpperCase().toLowerCase()` is not necessarily equal to `s.toLowerCase()`.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.toLowerCase"))
}
