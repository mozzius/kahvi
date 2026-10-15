//! Intl.Segmenter.prototype.segment ( _string_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.segmenter.prototype.segment

use kahvi_core::prelude::*;

use crate::support::intl_segmenter::{SegmenterSlots, create_segments_object};

/// Intl.Segmenter.prototype.segment
pub(crate) fn prototype_segment(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The `Intl.Segmenter.prototype.segment` method is called on an Intl.Segmenter instance with argument _string_ to create a Segments instance for the string using the locale and options of the Intl.Segmenter instance. The following steps are taken:
    //
    let _ = new_target;
    // 1. Let _segmenter_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_segmenter_, [[InitializedSegmenter]]).
    let segmenter = match this {
        Value::Object(o) if agent.native::<SegmenterSlots>(o).is_some() => o,
        _ => {
            return Err(agent.throw_type_error(
                "Intl.Segmenter.prototype.segment called on an incompatible receiver",
            ));
        }
    };
    // 3. Set _string_ to ? ToString(_string_).
    let string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 4. Return CreateSegmentsObject(_segmenter_, _string_).
    Ok(Value::Object(create_segments_object(
        agent, segmenter, string,
    )))
}
