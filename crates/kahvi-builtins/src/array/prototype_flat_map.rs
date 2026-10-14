//! Array.prototype.flatMap ( _mapperFunc_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.flatmap

use kahvi_core::prelude::*;

/// Array.prototype.flatMap
pub(crate) fn prototype_flat_map(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _sourceLength_ be ? LengthOfArrayLike(_obj_).
    // 3. If IsCallable(_mapperFunc_) is *false*, throw a *TypeError* exception.
    // 4. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
    // 5. Perform ? FlattenIntoArray(_array_, _obj_, _sourceLength_, 0, 1, _mapperFunc_, _thisArg_).
    // 6. Return _array_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.flatMap"))
}
