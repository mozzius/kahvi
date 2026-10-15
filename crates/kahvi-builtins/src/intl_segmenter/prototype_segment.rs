//! Intl.Segmenter.prototype.segment ( _string_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.segmenter.prototype.segment

use kahvi_core::prelude::*;

/// Intl.Segmenter.prototype.segment
pub(crate) fn prototype_segment(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The `Intl.Segmenter.prototype.segment` method is called on an Intl.Segmenter instance with argument _string_ to create a Segments instance for the string using the locale and options of the Intl.Segmenter instance. The following steps are taken:
    //
    // 1. Let _segmenter_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_segmenter_, [[InitializedSegmenter]]).
    // 3. Set _string_ to ? ToString(_string_).
    // 4. Return CreateSegmentsObject(_segmenter_, _string_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Segmenter.prototype.segment"))
}
