//! Intl.DurationFormat.prototype.formatToParts ( _duration_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DurationFormat.prototype.formatToParts

use kahvi_core::prelude::*;

/// Intl.DurationFormat.prototype.formatToParts
pub(crate) fn prototype_format_to_parts(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `formatToParts` method is called with an argument _duration_, the following steps are taken:
    //
    // 1. Let _df_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_df_, [[InitializedDurationFormat]]).
    // 3. Let _record_ be ? ToDurationRecord(_duration_).
    // 4. Let _parts_ be PartitionDurationFormatPattern(_df_, _record_).
    // 5. Let _result_ be ! ArrayCreate(0).
    // 6. Let _n_ be 0.
    // 7. For each Record { [[Type]], [[Value]], [[Unit]] } _part_ of _parts_, do
    //    a. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
    //    b. Perform ! CreateDataPropertyOrThrow(_obj_, *"type"*, _part_.[[Type]]).
    //    c. Perform ! CreateDataPropertyOrThrow(_obj_, *"value"*, _part_.[[Value]]).
    //    d. If _part_.[[Unit]] is not ~empty~, perform ! CreateDataPropertyOrThrow(_obj_, *"unit"*, _part_.[[Unit]]).
    //    e. Perform ! CreateDataPropertyOrThrow(_result_, ! ToString(_n_), _obj_).
    //    f. Set _n_ to _n_ + 1.
    // 8. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DurationFormat.prototype.formatToParts"))
}
