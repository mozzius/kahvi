# Ticket B-iterator-zip: Iterator.zip

Implement this built-in in its file, following the spec steps already in
the stub: keep each step as a comment directly above the code for it.

- Abstract operations are in `kahvi_core::ops` (see docs/API.md); use them
  rather than re-implementing. If a helper you need is missing and other
  built-ins would need it too, write it in your file and say so in your
  final report; do not edit other files.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing. Never return
  a made-up value.
- No new Rust tests: the test262 target is the measure.

## Files you may change

- `crates/kahvi-builtins/src/iterator/zip.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

Iterator.zip ( _iterables_ [ , _options_ ] )
  id: sec-iterator.zip   type: built-in function

This function performs the following steps when called:

1. If _iterables_ is not an Object, throw a *TypeError* exception.
2. Set _options_ to ? GetOptionsObject(_options_).
3. Let _mode_ be ? Get(_options_, *"mode"*).
4. If _mode_ is *undefined*, set _mode_ to *"shortest"*.
5. If _mode_ is not one of *"shortest"*, *"longest"*, or *"strict"*, throw a *TypeError* exception.
6. Let _paddingOption_ be *undefined*.
7. If _mode_ is *"longest"*, then
   a. Set _paddingOption_ to ? Get(_options_, *"padding"*).
   b. If _paddingOption_ is not *undefined* and _paddingOption_ is not an Object, throw a *TypeError* exception.
8. Let _iters_ be a new empty List.
9. Let _padding_ be a new empty List.
10. Let _inputIter_ be ? GetIterator(_iterables_, ~sync~).
11. Let _next_ be ~not-started~.
12. Repeat, while _next_ is not ~done~,
   a. Set _next_ to Completion(IteratorStepValue(_inputIter_)).
   b. IfAbruptCloseIterators(_next_, _iters_).
   c. If _next_ is not ~done~, then
      i. Let _iter_ be Completion(GetIteratorFlattenable(_next_, ~reject-primitives~)).
      ii. Let _needClosing_ be the list-concatenation of « _inputIter_ » and _iters_.
      iii. IfAbruptCloseIterators(_iter_, _needClosing_).
      iv. Append _iter_ to _iters_.
13. Let _iterCount_ be the number of elements in _iters_.
14. If _mode_ is *"longest"*, then
   a. If _paddingOption_ is *undefined*, then
      i. Repeat _iterCount_ times:
         1. Append *undefined* to _padding_.
   b. Else,
      i. Let _paddingIter_ be Completion(GetIterator(_paddingOption_, ~sync~)).
      ii. IfAbruptCloseIterators(_paddingIter_, _iters_).
      iii. Let _usingIterator_ be *true*.
      iv. Repeat _iterCount_ times:
         1. If _usingIterator_ is *true*, then
            a. Set _next_ to Completion(IteratorStepValue(_paddingIter_)).
            b. IfAbruptCloseIterators(_next_, _iters_).
            c. If _next_ is ~done~, then
               i. Set _usingIterator_ to *false*.
            d. Else,
               i. Append _next_ to _padding_.
         2. If _usingIterator_ is *false*, append *undefined* to _padding_.
      v. If _usingIterator_ is *true*, then
         1. Let _completion_ be Completion(IteratorClose(_paddingIter_, NormalCompletion(~unused~))).
         2. IfAbruptCloseIterators(_completion_, _iters_).
15. Let _finishResults_ be a new Abstract Closure with parameters (_results_) that captures nothing and performs the following steps when called:
   a. Return CreateArrayFromList(_results_).
16. Return IteratorZip(_iters_, _mode_, _padding_, _finishResults_).

--- referenced: GetOptionsObject ( _options_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-getoptionsobject

1. If _options_ is *undefined*, then
   a. Return OrdinaryObjectCreate(*null*).
2. If _options_ is an Object, then
   a. Return _options_.
3. Throw a *TypeError* exception.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: GetIterator ( _obj_: an ECMAScript language value, _kind_: ~sync~ or ~async~, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiterator

1. If _kind_ is ~async~, then
   a. Let _method_ be ? GetMethod(_obj_, %Symbol.asyncIterator%).
   b. If _method_ is *undefined*, then
      i. Let _syncMethod_ be ? GetMethod(_obj_, %Symbol.iterator%).
      ii. If _syncMethod_ is *undefined*, throw a *TypeError* exception.
      iii. Let _syncIteratorRecord_ be ? GetIteratorFromMethod(_obj_, _syncMethod_).
      iv. Return CreateAsyncFromSyncIterator(_syncIteratorRecord_).
2. Else,
   a. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
3. If _method_ is *undefined*, throw a *TypeError* exception.
4. Return ? GetIteratorFromMethod(_obj_, _method_).

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

--- referenced: IteratorStepValue ( _iteratorRecord_: an Iterator Record, ): either a normal completion containing either an ECMAScript language value or ~done~, or a throw completion
  id: sec-iteratorstepvalue

description: It requests the next value from _iteratorRecord_.[[Iterator]] by calling _iteratorRecord_.[[NextMethod]] and returns either ~done~ indicating that the iterator has reached its end or the value from the IteratorResult object if a next value is available.

1. Let _result_ be ? IteratorStep(_iteratorRecord_).
2. If _result_ is ~done~, then
   a. Return ~done~.
3. Let _value_ be Completion(IteratorValue(_result_)).
4. If _value_ is a throw completion, then
   a. Set _iteratorRecord_.[[Done]] to *true*.
5. Return ? _value_.

--- referenced: GetIteratorFlattenable ( _obj_: an ECMAScript language value, _primitiveHandling_: ~iterate-string-primitives~ or ~reject-primitives~, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratorflattenable

1. If _obj_ is not an Object, then
   a. If _primitiveHandling_ is ~reject-primitives~, throw a *TypeError* exception.
   b. Assert: _primitiveHandling_ is ~iterate-string-primitives~.
   c. If _obj_ is not a String, throw a *TypeError* exception.
2. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
3. If _method_ is *undefined*, then
   a. Let _iterator_ be _obj_.
4. Else,
   a. Let _iterator_ be ? Call(_method_, _obj_).
5. If _iterator_ is not an Object, throw a *TypeError* exception.
6. Return ? GetIteratorDirect(_iterator_).

--- referenced: IteratorClose ( _iteratorRecord_: an Iterator Record, _completion_: a Completion Record, ): a Completion Record
  id: sec-iteratorclose

description: It is used to notify an iterator that it should perform any actions it would normally perform when it has reached its completed state.

1. Assert: _iteratorRecord_.[[Iterator]] is an Object.
2. Let _iterator_ be _iteratorRecord_.[[Iterator]].
3. Let _innerResult_ be Completion(GetMethod(_iterator_, *"return"*)).
4. If _innerResult_ is a normal completion, then
   a. Let _return_ be _innerResult_.[[Value]].
   b. If _return_ is *undefined*, return ? _completion_.
   c. Set _innerResult_ to Completion(Call(_return_, _iterator_)).
5. If _completion_ is a throw completion, return ? _completion_.
6. If _innerResult_ is a throw completion, return ? _innerResult_.
7. If _innerResult_.[[Value]] is not an Object, throw a *TypeError* exception.
8. Return ? _completion_.

--- referenced: NormalCompletion ( _value_: any value except a Completion Record, ): a normal completion
  id: sec-normalcompletion

skip return checks: true

1. Return Completion Record { [[Type]]: ~normal~, [[Value]]: _value_, [[Target]]: ~empty~ }.

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.


## Done when

- Every test262 run under built-ins/Iterator/zip/ passes (check with `./t262 built-ins/Iterator/zip/`). 12 pass on the main branch right now.

