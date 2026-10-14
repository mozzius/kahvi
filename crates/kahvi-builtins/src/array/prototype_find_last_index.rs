//! Array.prototype.findLastIndex ( _predicate_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.findlastindex

use kahvi_core::prelude::*;

/// Array.prototype.findLastIndex
pub(crate) fn prototype_find_last_index(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method calls _predicate_ once for each element of the array, in descending index order, until it finds one where _predicate_ returns a value that coerces to *true*. If such an element is found, `findLastIndex` immediately returns the index of that element value. Otherwise, `findLastIndex` returns -1.
    // See FindViaPredicate for additional information.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    let length = ops::length_of_array_like(agent, obj)?;
    // 3. Let _findRecord_ be ? FindViaPredicate(_obj_, _length_, ~descending~, _predicate_, _thisArg_).
    let predicate = args.get_or_undefined(0);
    let this_arg = args.get_or_undefined(1);
    let index = find_via_predicate(agent, obj, length, predicate, this_arg)?;
    // 4. Return _findRecord_.[[Index]].
    Ok(Value::Number(index))
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array object. Therefore it can be transferred to other kinds of objects for use as a method.
    // [subclause sec-findviapredicate: FindViaPredicate (
    // _obj_: an Object,
    // _length_: a non-negative integer,
    // _direction_: ~ascending~ or ~descending~,
    // _predicate_: an ECMAScript language value,
    // _thisArg_: an ECMAScript language value,
    // ): either a normal completion containing a Record with fields [[Index]] (an integral Number) and [[Value]] (an ECMAScript language value) or a throw completion]
}

/// FindViaPredicate(_obj_, _length_, _direction_, _predicate_, _thisArg_) (§23.1.3.12.1).
/// Only the ~descending~ direction is needed by this method. Returns the
/// [[Index]] of the record.
// NOTE: other built-ins (Array.prototype.find and friends) need this too; it lives here
// until the shared helper is added to `support`.
fn find_via_predicate(
    agent: &mut Agent,
    obj: Object,
    length: f64,
    predicate: Value,
    this_arg: Value,
) -> JsResult<f64> {
    // 1. If IsCallable(_predicate_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, predicate) {
        return Err(
            agent.throw_type_error("Array.prototype.findLastIndex predicate is not a function")
        );
    }
    // 2. If _direction_ is ~descending~, then
    //    a. Let _indices_ be a List of the integers in the interval from _length_ - 1 (inclusive) to 0 (inclusive), in descending order.
    // 3. (Ascending order is not used by this method.)
    // The callback may collect garbage: obj, predicate and thisArg stay rooted.
    agent.with_roots(&[Value::Object(obj), predicate, this_arg], |agent| {
        // 4. For each integer _k_ of _indices_, do
        let mut k = length;
        while k > 0.0 {
            k -= 1.0;
            // a. Let _Pk_ be ! ToString(𝔽(_k_)).
            let pk = agent.key_from_index(k);
            // b. Let _kValue_ be ? Get(_obj_, _Pk_).
            let k_value = ops::get(agent, obj, pk)?;
            // c. Let _testResult_ be ? Call(_predicate_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »).
            let mark = agent.push_root(k_value);
            let test_result = ops::call_rooted(
                agent,
                predicate,
                this_arg,
                &[k_value, Value::Number(k), Value::Object(obj)],
            );
            agent.pop_roots(mark);
            let test_result = test_result?;
            // d. If ToBoolean(_testResult_) is *true*, return the Record { [[Index]]: 𝔽(_k_), [[Value]]: _kValue_ }.
            if ops::to_boolean(agent, test_result) {
                return Ok(k);
            }
        }
        // 5. Return the Record { [[Index]]: -1𝔽, [[Value]]: *undefined* }.
        Ok(-1.0)
    })
}
