//! String.prototype.slice ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.slice

use kahvi_core::prelude::*;

/// String.prototype.slice
pub(crate) fn prototype_slice(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a substring of the result of converting this object to a String, starting from index _start_ and running to, but not including, index _end_ (or through the end of the String if _end_ is *undefined*). If _start_ is negative, it is treated as _sourceLength_ + _start_ where _sourceLength_ is the length of the String. If _end_ is negative, it is treated as _sourceLength_ + _end_ where _sourceLength_ is the length of the String. The result is a String value, not a String object.
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _length_ be the length of _string_.
    // 5. Let _from_ be ? ToClampedIndex(_start_, _length_).
    // 6. If _end_ is *undefined*, let _to_ be _length_; else let _to_ be ? ToClampedIndex(_end_, _length_).
    // 7. If _from_ ≥ _to_, return the empty String.
    // 8. Return the substring of _string_ from _from_ to _to_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.slice"))
}
