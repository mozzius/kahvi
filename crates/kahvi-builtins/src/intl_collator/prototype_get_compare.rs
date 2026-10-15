//! get Intl.Collator.prototype.compare
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.collator.prototype.compare

use kahvi_core::prelude::*;

/// get Intl.Collator.prototype.compare
pub(crate) fn prototype_get_compare(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This named accessor property returns a function that compares two strings according to the sort order of this Collator object.
    // Intl.Collator.prototype.compare is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _collator_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_collator_, [[InitializedCollator]]).
    // 3. If _collator_.[[BoundCompare]] is *undefined*, then
    //    a. Let _closure_ be a new Abstract Closure with parameters (_x_, _y_) that captures _collator_ and performs the following steps when called:
    //       i. Let _xString_ be ? ToString(_x_).
    //       ii. Let _yString_ be ? ToString(_y_).
    //       iii. Return CompareStrings(_collator_, _xString_, _yString_).
    //    b. Set _collator_.[[BoundCompare]] to CreateBuiltinFunction(_closure_, 2, *""*, « »).
    // 4. Return _collator_.[[BoundCompare]].
    //
    // NOTE:
    // The returned function is bound to _collator_ so that it can be passed directly to `Array.prototype.sort` or other functions.
    // This is considered a historical artefact, as part of a convention which is no longer followed for new features, but is preserved to maintain compatibility with existing programs.
    // [subclause sec-collator-comparestrings: CompareStrings (
    // _collator_: an Intl.Collator,
    // _x_: a String,
    // _y_: a String,
    // ): a Number, but not *NaN*]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Intl.Collator.prototype.compare"))
}
