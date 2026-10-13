//! Iterator.zipKeyed ( _iterables_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.zipkeyed

use kahvi_core::prelude::*;

/// Iterator.zipKeyed
pub(crate) fn zip_keyed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _iterables_ is not an Object, throw a *TypeError* exception.
    // 2. Set _options_ to ? GetOptionsObject(_options_).
    // 3. Let _mode_ be ? Get(_options_, *"mode"*).
    // 4. If _mode_ is *undefined*, set _mode_ to *"shortest"*.
    // 5. If _mode_ is not one of *"shortest"*, *"longest"*, or *"strict"*, throw a *TypeError* exception.
    // 6. Let _paddingOption_ be *undefined*.
    // 7. If _mode_ is *"longest"*, then
    //    a. Set _paddingOption_ to ? Get(_options_, *"padding"*).
    //    b. If _paddingOption_ is not *undefined* and _paddingOption_ is not an Object, throw a *TypeError* exception.
    // 8. Let _iters_ be a new empty List.
    // 9. Let _padding_ be a new empty List.
    // 10. Let _allKeys_ be ? _iterables_.[[OwnPropertyKeys]]().
    // 11. Let _keys_ be a new empty List.
    // 12. For each element _key_ of _allKeys_, do
    //    a. Let _propertyDesc_ be Completion(_iterables_.[[GetOwnProperty]](_key_)).
    //    b. IfAbruptCloseIterators(_propertyDesc_, _iters_).
    //    c. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Enumerable]] is *true*, then
    //       i. Let _value_ be Completion(Get(_iterables_, _key_)).
    //       ii. IfAbruptCloseIterators(_value_, _iters_).
    //       iii. If _value_ is not *undefined*, then
    //          1. Append _key_ to _keys_.
    //          2. Let _iter_ be Completion(GetIteratorFlattenable(_value_, ~reject-primitives~)).
    //          3. IfAbruptCloseIterators(_iter_, _iters_).
    //          4. Append _iter_ to _iters_.
    // 13. Let _iterCount_ be the number of elements in _iters_.
    // 14. If _mode_ is *"longest"*, then
    //    a. If _paddingOption_ is *undefined*, then
    //       i. Repeat _iterCount_ times:
    //          1. Append *undefined* to _padding_.
    //    b. Else,
    //       i. For each element _key_ of _keys_, do
    //          1. Let _value_ be Completion(Get(_paddingOption_, _key_)).
    //          2. IfAbruptCloseIterators(_value_, _iters_).
    //          3. Append _value_ to _padding_.
    // 15. Let _finishResults_ be a new Abstract Closure with parameters (_results_) that captures _keys_ and _iterCount_ and performs the following steps when called:
    //    a. Let _obj_ be OrdinaryObjectCreate(*null*).
    //    b. For each integer _i_ such that 0 ≤ _i_ < _iterCount_, in ascending order, do
    //       i. Perform ! CreateDataPropertyOrThrow(_obj_, _keys_[_i_], _results_[_i_]).
    //    c. Return _obj_.
    // 16. Return IteratorZip(_iters_, _mode_, _padding_, _finishResults_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.zipKeyed"))
}
