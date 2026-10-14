//! Iterator.prototype.find ( _predicate_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.find

use kahvi_core::prelude::*;

use kahvi_core::ops::IteratorRecord;

/// Iterator.prototype.find
pub(crate) fn prototype_find(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let predicate = args.get_or_undefined(0);
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    let Value::Object(obj) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.find called on a non-object"));
    };
    // 3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
    let iterated = IteratorRecord {
        iterator: obj,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. If IsCallable(_predicate_) is *false*, then
    if !ops::is_callable(agent, predicate) {
        // a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
        let error = agent.throw_type_error("Iterator.prototype.find: predicate is not callable");
        // b. Return ? IteratorClose(_iterated_, _error_).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 5. Set _iterated_ to ? GetIteratorDirect(_obj_).
    let mut iterated = ops::get_iterator_direct(agent, obj)?;
    // 6. Let _counter_ be *+0*_𝔽.
    let mut counter: f64 = 0.0;
    // 7. Repeat,
    loop {
        // a. Let _value_ be ? IteratorStepValue(_iterated_).
        let Some(value) = ops::iterator_step_value(agent, &mut iterated)? else {
            // b. If _value_ is ~done~, return *undefined*.
            return Ok(Value::Undefined);
        };
        // c. Let _result_ be Completion(Call(_predicate_, *undefined*, « _value_, _counter_ »)).
        let result = ops::call(
            agent,
            predicate,
            Value::Undefined,
            &[value, Value::Number(counter)],
        );
        // d. IfAbruptCloseIterator(_result_, _iterated_).
        let result = match result {
            Ok(v) => v,
            Err(e) => return ops::iterator_close(agent, &iterated, Err(e)),
        };
        // e. If ToBoolean(_result_) is *true*, return ? IteratorClose(_iterated_, NormalCompletion(_value_)).
        if ops::to_boolean(agent, result) {
            return ops::iterator_close(agent, &iterated, Ok(value));
        }
        // f. NOTE: The following step will not change _counter_ once it reaches *2^53*_𝔽.
        // g. Set _counter_ to _counter_ + *1*_𝔽.
        if counter < 9007199254740992.0 {
            counter += 1.0;
        }
    }
}
