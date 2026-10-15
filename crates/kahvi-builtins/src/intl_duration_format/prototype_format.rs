//! Intl.DurationFormat.prototype.format ( _duration_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DurationFormat.prototype.format

use kahvi_core::prelude::*;

/// Intl.DurationFormat.prototype.format
pub(crate) fn prototype_format(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `format` method is called with an argument _duration_, the following steps are taken:
    //
    // 1. Let _df_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_df_, [[InitializedDurationFormat]]).
    // 3. Let _record_ be ? ToDurationRecord(_duration_).
    // 4. Let _parts_ be PartitionDurationFormatPattern(_df_, _record_).
    // 5. Let _result_ be the empty String.
    // 6. For each Record { [[Type]], [[Value]], [[Unit]] } _part_ of _parts_, do
    //    a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
    // 7. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DurationFormat.prototype.format"))
}
