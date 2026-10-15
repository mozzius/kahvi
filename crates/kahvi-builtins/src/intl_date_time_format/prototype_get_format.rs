//! get Intl.DateTimeFormat.prototype.format
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.datetimeformat.prototype.format

use kahvi_core::prelude::*;

/// get Intl.DateTimeFormat.prototype.format
pub(crate) fn prototype_get_format(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // Intl.DateTimeFormat.prototype.format is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _dtf_ be the *this* value.
    // 2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Set _dtf_ to ? UnwrapDateTimeFormat(_dtf_).
    // 3. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
    // 4. If _dtf_.[[BoundFormat]] is *undefined*, then
    //    a. Let _closure_ be a new Abstract Closure with parameters (_date_) that captures _dtf_ and performs the following steps when called:
    //       i. If _date_ is *undefined*, let _x_ be ! Call(%Date.now%, *undefined*); else let _x_ be ? ToNumber(_date_).
    //       ii. Return ? FormatDateTime(_dtf_, _x_).
    //    b. Set _dtf_.[[BoundFormat]] to CreateBuiltinFunction(_closure_, 1, *""*, « »).
    // 5. Return _dtf_.[[BoundFormat]].
    //
    // NOTE:
    // The returned function is bound to _dtf_ so that it can be passed directly to `Array.prototype.map` or other functions.
    // This is considered a historical artefact, as part of a convention which is no longer followed for new features, but is preserved to maintain compatibility with existing programs.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Intl.DateTimeFormat.prototype.format"))
}
