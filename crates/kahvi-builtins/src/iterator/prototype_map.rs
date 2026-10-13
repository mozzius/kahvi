//! Iterator.prototype.map ( _mapper_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.map

use kahvi_core::prelude::*;

/// Iterator.prototype.map
pub(crate) fn prototype_map(
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
    //       v. Let _completion_ be Completion(Yield(_mapped_)).
    //       vi. IfAbruptCloseIterator(_completion_, _iterated_).
    //       vii. NOTE: The following step will not change _counter_ once it reaches *2^53*_𝔽.
    //       viii. Set _counter_ to _counter_ + *1*_𝔽.
    // 7. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 8. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
    // 9. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.map"))
}
