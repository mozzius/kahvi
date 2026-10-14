//! String.prototype.substr ( _start_, _length_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.substr

use kahvi_core::prelude::*;

/// String.prototype.substr
pub(crate) fn prototype_substr(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a substring of the result of converting the *this* value to a String, starting from index _start_ and running for _length_ code units (or through the end of the String if _length_ is *undefined*). If _start_ is negative, it is treated as _sourceLength_ + _start_ where _sourceLength_ is the length of the String. The result is a String value, not a String object.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_obj_).
    // 3. Let _string_ be ? ToString(_obj_).
    // 4. Let _size_ be the length of _string_.
    // 5. Let _intStart_ be ? ToClampedIndex(_start_, _size_).
    // 6. If _length_ is *undefined*, let _intLength_ be _size_; else let _intLength_ be the result of clamping ? ToIntegerOrInfinity(_length_) between 0 and _size_.
    // 7. Let _intEnd_ be min(_intStart_ + _intLength_, _size_).
    // 8. Return the substring of _string_ from _intStart_ to _intEnd_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.substr"))
}
