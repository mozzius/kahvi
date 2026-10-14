//! RegExp.escape ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.escape

use kahvi_core::prelude::*;

/// RegExp.escape
pub(crate) fn escape(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns a copy of _string_ in which characters that are potentially special in a regular expression |Pattern| have been replaced by equivalent escape sequences.
    // It performs the following steps when called:
    //
    // 1. If _string_ is not a String, throw a *TypeError* exception.
    // 2. Let _escaped_ be the empty String.
    // 3. Let _codePointList_ be StringToCodePoints(_string_).
    // 4. For each code point _codePoint_ of _codePointList_, do
    //    a. If _escaped_ is the empty String and _codePoint_ is matched by either |DecimalDigit| or |AsciiLetter|, then
    //       i. NOTE: Escaping a leading digit ensures that output corresponds with pattern text which may be used after a `\0` character escape or a |DecimalEscape| such as `\1` and still match _string_ rather than be interpreted as an extension of the preceding escape sequence. Escaping a leading ASCII letter does the same for the context after `\c`.
    //       ii. Let _numericValue_ be the numeric value of _codePoint_.
    //       iii. Let _hex_ be Number::toString(𝔽(_numericValue_), 16).
    //       iv. Assert: The length of _hex_ is 2.
    //       v. Set _escaped_ to the string-concatenation of the code unit 0x005C (REVERSE SOLIDUS), *"x"*, and _hex_.
    //    b. Else,
    //       i. Set _escaped_ to the string-concatenation of _escaped_ and EncodeForRegExpEscape(_codePoint_).
    // 5. Return _escaped_.
    //
    // NOTE:
    // Despite having similar names, EscapeRegExpPattern and `RegExp.escape` do not perform similar actions. The former escapes a pattern for representation as a string, while this function escapes a string for representation inside a pattern.
    // [subclause sec-encodeforregexpescape: EncodeForRegExpEscape (
    // _codePoint_: a code point,
    // ): a String]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.escape"))
}
