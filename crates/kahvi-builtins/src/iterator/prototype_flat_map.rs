//! Iterator.prototype.flatMap ( _mapper_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.flatmap

use kahvi_core::prelude::*;

/// Iterator.prototype.flatMap
pub(crate) fn prototype_flat_map(
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
    // 4. If IsCallable(_mapper_) is *false*, then
    //    a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 5. Set _iterated_ to ? GetIteratorDirect(_obj_).
    // 6. Let _closure_ be a new Abstract Closure with no parameters that captures _iterated_ and _mapper_ and performs the following steps when called:
    //    a. Let _counter_ be *+0*_𝔽.
    //    b. Repeat,
    //       i. Let _value_ be ? IteratorStepValue(_iterated_).
    //       ii. If _value_ is ~done~, return ReturnCompletion(*undefined*).
    //       iii. Let _mapped_ be Completion(Call(_mapper_, *undefined*, « _value_, _counter_ »)).
    //       iv. IfAbruptCloseIterator(_mapped_, _iterated_).
    //       v. Let _innerIterator_ be Completion(GetIteratorFlattenable(_mapped_, ~reject-primitives~)).
    //       vi. IfAbruptCloseIterator(_innerIterator_, _iterated_).
    //       vii. Let _innerAlive_ be *true*.
    //       viii. Repeat, while _innerAlive_ is *true*,
    //          1. Let _innerValue_ be Completion(IteratorStepValue(_innerIterator_)).
    //          2. IfAbruptCloseIterator(_innerValue_, _iterated_).
    //          3. If _innerValue_ is ~done~, then
    //             a. Set _innerAlive_ to *false*.
    //          4. Else,
    //             a. Let _completion_ be Completion(Yield(_innerValue_)).
    //             b. If _completion_ is an abrupt completion, then
    //                i. Let _backupCompletion_ be Completion(IteratorClose(_innerIterator_, _completion_)).
    //                ii. IfAbruptCloseIterator(_backupCompletion_, _iterated_).
    //                iii. Return ? IteratorClose(_iterated_, _completion_).
    //       ix. NOTE: The following step will not change _counter_ once it reaches *2^53*_𝔽.
    //       x. Set _counter_ to _counter_ + *1*_𝔽.
    // 7. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 8. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
    // 9. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.flatMap"))
}
