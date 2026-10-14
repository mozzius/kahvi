//! Iterator.prototype.some ( _predicate_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.some

use kahvi_core::prelude::*;

/// Iterator.prototype.some
pub(crate) fn prototype_some(
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
    // 4. If IsCallable(_predicate_) is *false*, then
    //    a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 5. Set _iterated_ to ? GetIteratorDirect(_obj_).
    // 6. Let _counter_ be *+0*_𝔽.
    // 7. Repeat,
    //    a. Let _value_ be ? IteratorStepValue(_iterated_).
    //    b. If _value_ is ~done~, return *false*.
    //    c. Let _result_ be Completion(Call(_predicate_, *undefined*, « _value_, _counter_ »)).
    //    d. IfAbruptCloseIterator(_result_, _iterated_).
    //    e. If ToBoolean(_result_) is *true*, return ? IteratorClose(_iterated_, NormalCompletion(*true*)).
    //    f. NOTE: The following step will not change _counter_ once it reaches *2^53*_𝔽.
    //    g. Set _counter_ to _counter_ + *1*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.some"))
}
