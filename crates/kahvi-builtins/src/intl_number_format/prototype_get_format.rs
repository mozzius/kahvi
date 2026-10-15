//! get Intl.NumberFormat.prototype.format
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.numberformat.prototype.format

use kahvi_core::prelude::*;

/// get Intl.NumberFormat.prototype.format
pub(crate) fn prototype_get_format(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // Intl.NumberFormat.prototype.format is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _nf_ be the *this* value.
    // 2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Set _nf_ to ? UnwrapNumberFormat(_nf_).
    // 3. Perform ? RequireInternalSlot(_nf_, [[InitializedNumberFormat]]).
    // 4. If _nf_.[[BoundFormat]] is *undefined*, then
    //    a. Let _closure_ be a new Abstract Closure with parameters (_value_) that captures _nf_ and performs the following steps when called:
    //       i. Let _x_ be ? ToIntlMathematicalValue(_value_).
    //       ii. Return FormatNumeric(_nf_, _x_).
    //    b. Set _nf_.[[BoundFormat]] to CreateBuiltinFunction(_closure_, 1, *""*, « »).
    // 5. Return _nf_.[[BoundFormat]].
    //
    // NOTE:
    // The returned function is bound to _nf_ so that it can be passed directly to `Array.prototype.map` or other functions.
    // This is considered a historical artefact, as part of a convention which is no longer followed for new features, but is preserved to maintain compatibility with existing programs.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Intl.NumberFormat.prototype.format"))
}
