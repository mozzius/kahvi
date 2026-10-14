//! Array.prototype.findLastIndex ( _predicate_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.findlastindex

use kahvi_core::prelude::*;

/// Array.prototype.findLastIndex
pub(crate) fn prototype_find_last_index(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method calls _predicate_ once for each element of the array, in descending index order, until it finds one where _predicate_ returns a value that coerces to *true*. If such an element is found, `findLastIndex` immediately returns the index of that element value. Otherwise, `findLastIndex` returns -1.
    // See FindViaPredicate for additional information.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _findRecord_ be ? FindViaPredicate(_obj_, _length_, ~descending~, _predicate_, _thisArg_).
    // 4. Return _findRecord_.[[Index]].
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array object. Therefore it can be transferred to other kinds of objects for use as a method.
    // [subclause sec-findviapredicate: FindViaPredicate (
    // _obj_: an Object,
    // _length_: a non-negative integer,
    // _direction_: ~ascending~ or ~descending~,
    // _predicate_: an ECMAScript language value,
    // _thisArg_: an ECMAScript language value,
    // ): either a normal completion containing a Record with fields [[Index]] (an integral Number) and [[Value]] (an ECMAScript language value) or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.findLastIndex"))
}
