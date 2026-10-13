//! Iterator.prototype.reduce ( _reducer_ [ , _initialValue_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.reduce

use kahvi_core::prelude::*;

/// Iterator.prototype.reduce
pub(crate) fn prototype_reduce(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    // 3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
    // 4. If IsCallable(_reducer_) is *false*, then
    //    a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 5. Set _iterated_ to ? GetIteratorDirect(_obj_).
    // 6. If _initialValue_ is not present, then
    //    a. Let _accumulator_ be ? IteratorStepValue(_iterated_).
    //    b. If _accumulator_ is ~done~, throw a *TypeError* exception.
    //    c. Let _counter_ be *1*_𝔽.
    // 7. Else,
    //    a. Let _accumulator_ be _initialValue_.
    //    b. Let _counter_ be *+0*_𝔽.
    // 8. Repeat,
    //    a. Let _value_ be ? IteratorStepValue(_iterated_).
    //    b. If _value_ is ~done~, return _accumulator_.
    //    c. Let _result_ be Completion(Call(_reducer_, *undefined*, « _accumulator_, _value_, _counter_ »)).
    //    d. IfAbruptCloseIterator(_result_, _iterated_).
    //    e. Set _accumulator_ to _result_.
    //    f. NOTE: The following step will not change _counter_ once it reaches *2^53*_𝔽.
    //    g. Set _counter_ to _counter_ + *1*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.reduce"))
}
