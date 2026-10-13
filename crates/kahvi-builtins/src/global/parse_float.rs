//! parseFloat ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-parsefloat-string

use kahvi_core::prelude::*;

/// parseFloat
pub(crate) fn parse_float(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function produces a Number value dictated by interpretation of the contents of the _string_ argument as a decimal literal.
    // It is the %parseFloat% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _inputString_ be ? ToString(_string_).
    // 2. Let _trimmedString_ be ! TrimString(_inputString_, ~start~).
    // 3. Let _trimmed_ be StringToCodePoints(_trimmedString_).
    // 4. Let _trimmedPrefix_ be the longest prefix of _trimmed_ that satisfies the syntax of a |StrDecimalLiteral|, which might be _trimmed_ itself. If there is no such prefix, return *NaN*.
    // 5. Let _parsedNumber_ be ParseText(_trimmedPrefix_, |StrDecimalLiteral|).
    // 6. Assert: _parsedNumber_ is a Parse Node.
    // 7. Return the StringNumericValue of _parsedNumber_.
    //
    // NOTE:
    // This function may interpret only a leading portion of _string_ as a Number value; it ignores any code units that cannot be interpreted as part of the notation of a decimal literal, and no indication is given that any such code units were ignored.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("parseFloat"))
}
