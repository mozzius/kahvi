//! Array.prototype.findIndex ( _predicate_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.findindex

use kahvi_core::prelude::*;

/// Array.prototype.findIndex
pub(crate) fn prototype_find_index(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method calls _predicate_ once for each element of the array, in ascending index order, until it finds one where _predicate_ returns a value that coerces to *true*. If such an element is found, `findIndex` immediately returns the index of that element value. Otherwise, `findIndex` returns -1.
    // See FindViaPredicate for additional information.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _findRecord_ be ? FindViaPredicate(_obj_, _length_, ~ascending~, _predicate_, _thisArg_).
    // 4. Return _findRecord_.[[Index]].
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.findIndex"))
}
