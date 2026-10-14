//! Iterator.prototype.take ( _limit_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.take

use kahvi_core::prelude::*;

/// Iterator.prototype.take
pub(crate) fn prototype_take(
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
    // 4. Let _numberLimit_ be Completion(ToNumber(_limit_)).
    // 5. IfAbruptCloseIterator(_numberLimit_, _iterated_).
    // 6. If _numberLimit_ is *NaN*, then
    //    a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 7. If _numberLimit_ is finite and _numberLimit_ > 𝔽(2^53 - 1), then
    //    a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 8. Let _intLimit_ be ! ToIntegerOrInfinity(_numberLimit_).
    // 9. If _intLimit_ < 0, then
    //    a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
    //    b. Return ? IteratorClose(_iterated_, _error_).
    // 10. Set _iterated_ to ? GetIteratorDirect(_obj_).
    // 11. Let _closure_ be a new Abstract Closure with no parameters that captures _iterated_ and _intLimit_ and performs the following steps when called:
    //    a. Let _remaining_ be 𝔽(_intLimit_).
    //    b. Repeat,
    //       i. If _remaining_ is *+0*_𝔽, then
    //          1. Return ? IteratorClose(_iterated_, ReturnCompletion(*undefined*)).
    //       ii. Set _remaining_ to _remaining_ - *1*_𝔽.
    //       iii. Let _value_ be ? IteratorStepValue(_iterated_).
    //       iv. If _value_ is ~done~, return ReturnCompletion(*undefined*).
    //       v. Let _completion_ be Completion(Yield(_value_)).
    //       vi. IfAbruptCloseIterator(_completion_, _iterated_).
    // 12. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 13. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
    // 14. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.take"))
}
