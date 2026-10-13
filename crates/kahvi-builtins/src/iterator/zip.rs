//! Iterator.zip ( _iterables_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.zip

use kahvi_core::prelude::*;

/// Iterator.zip
pub(crate) fn zip(
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
    // 10. Let _inputIter_ be ? GetIterator(_iterables_, ~sync~).
    // 11. Let _next_ be ~not-started~.
    // 12. Repeat, while _next_ is not ~done~,
    //    a. Set _next_ to Completion(IteratorStepValue(_inputIter_)).
    //    b. IfAbruptCloseIterators(_next_, _iters_).
    //    c. If _next_ is not ~done~, then
    //       i. Let _iter_ be Completion(GetIteratorFlattenable(_next_, ~reject-primitives~)).
    //       ii. Let _needClosing_ be the list-concatenation of « _inputIter_ » and _iters_.
    //       iii. IfAbruptCloseIterators(_iter_, _needClosing_).
    //       iv. Append _iter_ to _iters_.
    // 13. Let _iterCount_ be the number of elements in _iters_.
    // 14. If _mode_ is *"longest"*, then
    //    a. If _paddingOption_ is *undefined*, then
    //       i. Repeat _iterCount_ times:
    //          1. Append *undefined* to _padding_.
    //    b. Else,
    //       i. Let _paddingIter_ be Completion(GetIterator(_paddingOption_, ~sync~)).
    //       ii. IfAbruptCloseIterators(_paddingIter_, _iters_).
    //       iii. Let _usingIterator_ be *true*.
    //       iv. Repeat _iterCount_ times:
    //          1. If _usingIterator_ is *true*, then
    //             a. Set _next_ to Completion(IteratorStepValue(_paddingIter_)).
    //             b. IfAbruptCloseIterators(_next_, _iters_).
    //             c. If _next_ is ~done~, then
    //                i. Set _usingIterator_ to *false*.
    //             d. Else,
    //                i. Append _next_ to _padding_.
    //          2. If _usingIterator_ is *false*, append *undefined* to _padding_.
    //       v. If _usingIterator_ is *true*, then
    //          1. Let _completion_ be Completion(IteratorClose(_paddingIter_, NormalCompletion(~unused~))).
    //          2. IfAbruptCloseIterators(_completion_, _iters_).
    // 15. Let _finishResults_ be a new Abstract Closure with parameters (_results_) that captures nothing and performs the following steps when called:
    //    a. Return CreateArrayFromList(_results_).
    // 16. Return IteratorZip(_iters_, _mode_, _padding_, _finishResults_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.zip"))
}
