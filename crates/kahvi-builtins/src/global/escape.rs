//! escape ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-escape-string

use kahvi_core::prelude::*;

/// escape
pub(crate) fn escape(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function is a property of the global object. It computes a new version of a String value in which certain code units have been replaced by a hexadecimal escape sequence.
    // When replacing a code unit of numeric value less than or equal to 0x00FF, a two-digit escape sequence of the form %xx is used. When replacing a code unit of numeric value strictly greater than 0x00FF, a four-digit escape sequence of the form %uxxxx is used.
    // It is the %escape% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Set _string_ to ? ToString(_string_).
    // 2. Let _length_ be the length of _string_.
    // 3. Let _result_ be the empty String.
    // 4. Let _unescapedSet_ be the string-concatenation of the ASCII word characters and *"@\*+-./"*.
    // 5. Let _k_ be 0.
    // 6. Repeat, while _k_ < _length_,
    //    a. Let _codeUnit_ be the code unit at index _k_ within _string_.
    //    b. If _unescapedSet_ contains _codeUnit_, then
    //       i. Let _nextPart_ be _codeUnit_.
    //    c. Else,
    //       i. Let _codeUnitNumber_ be the numeric value of _codeUnit_.
    //       ii. If _codeUnitNumber_ < 256, then
    //          1. Let _hex_ be the String representation of _codeUnitNumber_, formatted as an uppercase hexadecimal number.
    //          2. Let _nextPart_ be the string-concatenation of *"%"* and StringPad(_hex_, 2, *"0"*, ~start~).
    //       iii. Else,
    //          1. Let _hex_ be the String representation of _codeUnitNumber_, formatted as an uppercase hexadecimal number.
    //          2. Let _nextPart_ be the string-concatenation of *"%u"* and StringPad(_hex_, 4, *"0"*, ~start~).
    //    d. Set _result_ to the string-concatenation of _result_ and _nextPart_.
    //    e. Set _k_ to _k_ + 1.
    // 7. Return _result_.
    //
    // NOTE:
    // The encoding is partly based on the encoding described in RFC 1738, but the entire encoding specified in this standard is described above without regard to the contents of RFC 1738. This encoding does not reflect changes to RFC 1738 made by RFC 3986.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("escape"))
}
