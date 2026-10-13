//! String.prototype [ %Symbol.iterator% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype-%symbol.iterator%

use kahvi_core::prelude::*;

/// String.prototype[Symbol.iterator]
pub(crate) fn prototype_symbol_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns an iterator object that iterates over the code points of a String value, returning each code point as a String value.
    // It performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_string_).
    // 3. Set _string_ to ? ToString(_string_).
    // 4. Let _closure_ be a new Abstract Closure with no parameters that captures _string_ and performs the following steps when called:
    //    a. Let _length_ be the length of _string_.
    //    b. Let _position_ be 0.
    //    c. Repeat, while _position_ < _length_,
    //       i. Let _codePoint_ be CodePointAt(_string_, _position_).
    //       ii. Let _nextIndex_ be _position_ + _codePoint_.[[CodeUnitCount]].
    //       iii. Let _resultString_ be the substring of _string_ from _position_ to _nextIndex_.
    //       iv. Set _position_ to _nextIndex_.
    //       v. Perform ? GeneratorYield(CreateIteratorResultObject(_resultString_, *false*)).
    //    d. Return NormalCompletion(~unused~).
    // 5. Return CreateIteratorFromClosure(_closure_, *"%StringIteratorPrototype%"*, %StringIteratorPrototype%).
    //
    // The value of the *"name"* property of this method is *"[Symbol.iterator]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype[Symbol.iterator]"))
}
