//! String.prototype.at ( _index_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.at

use kahvi_core::prelude::*;

/// String.prototype.at
pub(crate) fn prototype_at(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _length_ be the length of _string_.
    // 5. Let _k_ be ? ToAbsoluteIndex(_index_, _length_).
    // 6. If _k_ < 0 or _k_ ≥ _length_, return *undefined*.
    // 7. Return the substring of _string_ from _k_ to _k_ + 1.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.at"))
}
