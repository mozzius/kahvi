//! Array.prototype.find ( _predicate_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.find

use kahvi_core::prelude::*;

/// Array.prototype.find
pub(crate) fn prototype_find(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method calls _predicate_ once for each element of the array, in ascending index order, until it finds one where _predicate_ returns a value that coerces to *true*. If such an element is found, `find` immediately returns that element value. Otherwise, `find` returns *undefined*.
    // See FindViaPredicate for additional information.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    let length = ops::length_of_array_like(agent, obj)?;
    // 3. Let _findRecord_ be ? FindViaPredicate(_obj_, _length_, ~ascending~, _predicate_, _thisArg_).
    let predicate = args.get_or_undefined(0);
    let this_arg = args.get_or_undefined(1);
    let (_, value) = find_via_predicate(agent, obj, length, predicate, this_arg)?;
    // 4. Return _findRecord_.[[Value]].
    Ok(value)
}

/// FindViaPredicate(_obj_, _length_, _direction_, _predicate_, _thisArg_) (§23.1.3.12.1).
/// Only the ~ascending~ direction is needed by this method. Returns the
/// ([[Index]], [[Value]]) pair of the record.
fn find_via_predicate(
    agent: &mut Agent,
    obj: Object,
    length: f64,
    predicate: Value,
    this_arg: Value,
) -> JsResult<(f64, Value)> {
    // 1. If IsCallable(_predicate_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, predicate) {
        return Err(agent.throw_type_error("Array.prototype.find predicate is not a function"));
    }
    // 2. If _direction_ is ~ascending~, then
    //    a. Let _indices_ be a List of the integers in the interval from 0 (inclusive) to _length_ (exclusive), in ascending order.
    // 3. (Descending order is not used by this method.)
    // The callback may collect garbage: obj, predicate and thisArg stay rooted.
    agent.with_roots(&[Value::Object(obj), predicate, this_arg], |agent| {
        // 4. For each integer _k_ of _indices_, do
        let mut k = 0.0;
        while k < length {
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
                return Ok((k, k_value));
            }
            // e. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // 5. Return the Record { [[Index]]: -1𝔽, [[Value]]: *undefined* }.
        Ok((-1.0, Value::Undefined))
    })
}
