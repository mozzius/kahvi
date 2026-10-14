//! Array.from ( _items_ [ , _mapper_ [ , _thisArg_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.from

use kahvi_core::object::array::array_create;
use kahvi_core::prelude::*;

/// Array.from
pub(crate) fn from(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _C_ be the *this* value.
    let c = this;
    let items = args.get_or_undefined(0);
    let mapper = args.get_or_undefined(1);
    let this_arg = args.get_or_undefined(2);
    // 2. If _mapper_ is *undefined*, then
    //    a. Let _mapping_ be *false*.
    // 3. Else,
    //    a. If IsCallable(_mapper_) is *false*, throw a *TypeError* exception.
    //    b. Let _mapping_ be *true*.
    let mapping = if mapper.is_undefined() {
        false
    } else {
        if !ops::is_callable(agent, mapper) {
            return Err(agent.throw_type_error("Array.from: mapper is not a function"));
        }
        true
    };
    // 4. Let _usingIterator_ be ? GetMethod(_items_, %Symbol.iterator%).
    let iterator_key = PropertyKey::Symbol(agent.symbols_wk.iterator);
    let using_iterator = ops::get_method(agent, items, iterator_key)?;
    // 5. If _usingIterator_ is not *undefined*, then
    if !using_iterator.is_undefined() {
        // a. If IsConstructor(_C_) is *true*, then
        //    i. Let _A_ be ? Construct(_C_).
        // b. Else,
        //    i. Let _A_ be ! ArrayCreate(0).
        let a = match c {
            Value::Object(ctor) if ops::is_constructor(agent, c) => {
                ops::construct(agent, ctor, &[], None)?
            }
            _ => array_create(agent, 0.0, None)?,
        };
        // c. Let _iteratorRecord_ be ? GetIteratorFromMethod(_items_, _usingIterator_).
        let mut iterator_record = ops::get_iterator_from_method(agent, items, using_iterator)?;
        // d. Let _k_ be 0.
        let mut k = 0.0;
        // e. Repeat,
        loop {
            // i. If _k_ ≥ 2^53 - 1, then
            if k >= 9_007_199_254_740_991.0 {
                // 1. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
                let error =
                    agent.throw_type_error("Array.from: array-like length exceeds 2^53 - 1");
                // 2. Return ? IteratorClose(_iteratorRecord_, _error_).
                return ops::iterator_close(agent, &iterator_record, Err(error));
            }
            // ii. Let _propertyKey_ be ! ToString(𝔽(_k_)).
            let property_key = agent.key_from_index(k);
            // iii. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
            let Some(next) = ops::iterator_step_value(agent, &mut iterator_record)? else {
                // iv. If _next_ is ~done~, then
                //    1. Perform ? Set(_A_, *"length"*, 𝔽(_k_), *true*).
                let length_key = PropertyKey::String(agent.names.length);
                ops::set(agent, a, length_key, Value::Number(k), true)?;
                //    2. Return _A_.
                return Ok(Value::Object(a));
            };
            // v. If _mapping_ is *true*, then
            let mapped_value = if mapping {
                // 1. Let _mappedValue_ be Completion(Call(_mapper_, _thisArg_, « _next_, 𝔽(_k_) »)).
                // 2. IfAbruptCloseIterator(_mappedValue_, _iteratorRecord_).
                match ops::call(agent, mapper, this_arg, &[next, Value::Number(k)]) {
                    Ok(v) => v,
                    Err(e) => return ops::iterator_close(agent, &iterator_record, Err(e)),
                }
            } else {
                // vi. Else,
                //    1. Let _mappedValue_ be _next_.
                next
            };
            // vii. Let _defineStatus_ be Completion(CreateDataPropertyOrThrow(_A_, _propertyKey_, _mappedValue_)).
            // viii. IfAbruptCloseIterator(_defineStatus_, _iteratorRecord_).
            if let Err(e) = ops::create_data_property_or_throw(agent, a, property_key, mapped_value)
            {
                return ops::iterator_close(agent, &iterator_record, Err(e));
            }
            // ix. Set _k_ to _k_ + 1.
            k += 1.0;
        }
    }
    // 6. NOTE: _items_ is not iterable so assume it is an array-like object.
    // 7. Let _arrayLike_ be ! ToObject(_items_).
    let array_like = ops::to_object(agent, items)?;
    // 8. Let _len_ be ? LengthOfArrayLike(_arrayLike_).
    let len = ops::length_of_array_like(agent, array_like)?;
    // 9. If IsConstructor(_C_) is *true*, then
    //    a. Let _A_ be ? Construct(_C_, « 𝔽(_len_) »).
    // 10. Else,
    //    a. Let _A_ be ? ArrayCreate(_len_).
    let a = match c {
        Value::Object(ctor) if ops::is_constructor(agent, c) => {
            ops::construct(agent, ctor, &[Value::Number(len)], None)?
        }
        _ => array_create(agent, len, None)?,
    };
    // 11. Let _k_ be 0.
    let mut k = 0.0;
    // 12. Repeat, while _k_ < _len_,
    while k < len {
        // a. Let _Pk_ be ! ToString(𝔽(_k_)).
        let pk = agent.key_from_index(k);
        // b. Let _kValue_ be ? Get(_arrayLike_, _Pk_).
        let k_value = ops::get(agent, array_like, pk)?;
        // c. If _mapping_ is *true*, then
        let mapped_value = if mapping {
            // i. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
            ops::call(agent, mapper, this_arg, &[k_value, Value::Number(k)])?
        } else {
            // d. Else,
            //    i. Let _mappedValue_ be _kValue_.
            k_value
        };
        // e. Perform ? CreateDataPropertyOrThrow(_A_, _Pk_, _mappedValue_).
        ops::create_data_property_or_throw(agent, a, pk, mapped_value)?;
        // f. Set _k_ to _k_ + 1.
        k += 1.0;
    }
    // 13. Perform ? Set(_A_, *"length"*, 𝔽(_len_), *true*).
    let length_key = PropertyKey::String(agent.names.length);
    ops::set(agent, a, length_key, Value::Number(len), true)?;
    // 14. Return _A_.
    Ok(Value::Object(a))
}
