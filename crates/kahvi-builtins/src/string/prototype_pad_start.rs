//! String.prototype.padStart ( _maxLength_ [ , _fillString_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.padstart

use kahvi_core::prelude::*;

/// String.prototype.padStart
pub(crate) fn prototype_pad_start(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Return ? StringPaddingBuiltinsImpl(_thisValue_, _maxLength_, _fillString_, ~start~).
    //
    // [subclause sec-stringpaddingbuiltinsimpl: StringPaddingBuiltinsImpl (
    // _thisValue_: an ECMAScript language value,
    // _maxLength_: an ECMAScript language value,
    // _fillString_: an ECMAScript language value,
    // _placement_: ~start~ or ~end~,
    // ): either a normal completion containing a String or a throw completion]
    // [subclause sec-stringpad: StringPad (
    // _string_: a String,
    // _maxLength_: a non-negative integer,
    // _fillString_: a String,
    // _placement_: ~start~ or ~end~,
    // ): a String]
    // [subclause sec-tozeropaddeddecimalstring: ToZeroPaddedDecimalString (
    // _n_: a non-negative integer,
    // _minLength_: a non-negative integer,
    // ): a String]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.padStart"))
}
