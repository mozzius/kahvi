//! Array.prototype.flat ( [ _depth_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.flat

use kahvi_core::prelude::*;

/// Array.prototype.flat
pub(crate) fn prototype_flat(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _sourceLength_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _depthNumber_ be 1.
    // 4. If _depth_ is not *undefined*, then
    //    a. Set _depthNumber_ to ? ToIntegerOrInfinity(_depth_).
    //    b. If _depthNumber_ < 0, set _depthNumber_ to 0.
    // 5. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
    // 6. Perform ? FlattenIntoArray(_array_, _obj_, _sourceLength_, 0, _depthNumber_).
    // 7. Return _array_.
    //
    // [subclause sec-flattenintoarray: FlattenIntoArray (
    // _target_: an Object,
    // _source_: an Object,
    // _sourceLength_: a non-negative integer,
    // _start_: a non-negative integer,
    // _depth_: a non-negative integer or +∞,
    // optional _mapperFunc_: a function object,
    // optional _thisArg_: an ECMAScript language value,
    // ): either a normal completion containing a non-negative integer or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.flat"))
}
