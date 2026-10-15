# Ticket B-iterator-prototype-includes: Iterator.prototype.includes

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

- `crates/kahvi-builtins/src/iterator/prototype_includes.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

Iterator.prototype.includes ( _searchElement_ [ , _skippedElements_ ] )
  id: sec-iterator.prototype.includes   type: built-in function

1. Let _obj_ be the *this* value.
2. If _obj_ is not an Object, throw a *TypeError* exception.
3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
4. If _skippedElements_ is *undefined*, then
   a. Let _toSkip_ be *+0*_𝔽.
5. Else,
   a. If _skippedElements_ is not one of *+∞_𝔽*, *-∞_𝔽*, or an integral Number, then
      i. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
      ii. Return ? IteratorClose(_iterated_, _error_).
   b. Let _toSkip_ be _skippedElements_.
6. If _toSkip_ < *-0*_𝔽, then
   a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
   b. Return ? IteratorClose(_iterated_, _error_).
7. If _toSkip_ is finite and _toSkip_ > 𝔽(2^53 - 1), then
   a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
   b. Return ? IteratorClose(_iterated_, _error_).
8. Let _skipped_ be *+0*_𝔽.
9. Set _iterated_ to ? GetIteratorDirect(_obj_).
10. Repeat,
   a. Let _value_ be ? IteratorStepValue(_iterated_).
   b. If _value_ is ~done~, return *false*.
   c. If _skipped_ < _toSkip_, then
      i. Set _skipped_ to _skipped_ + *1*_𝔽.
   d. Else if SameValueZero(_value_, _searchElement_) is *true*, then
      i. Return ? IteratorClose(_iterated_, NormalCompletion(*true*)).

--- referenced: ThrowCompletion ( _value_: an ECMAScript language value, ): a throw completion
  id: sec-throwcompletion

1. Return Completion Record { [[Type]]: ~throw~, [[Value]]: _value_, [[Target]]: ~empty~ }.

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

--- referenced: GetIteratorDirect ( _obj_: an Object, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratordirect

1. Let _nextMethod_ be ? Get(_obj_, *"next"*).
2. Let _iteratorRecord_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: _nextMethod_, [[Done]]: *false* }.
3. Return _iteratorRecord_.

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

--- referenced: SameValueZero ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, ): a Boolean
  id: sec-samevaluezero

description: It determines whether or not the two arguments are the same value (ignoring the difference between *+0*_𝔽 and *-0*_𝔽).

1. If SameType(_x_, _y_) is *false*, return *false*.
2. If _x_ is a Number, then
   a. Return Number::sameValueZero(_x_, _y_).
3. Return SameValueNonNumber(_x_, _y_).

NOTE:
SameValueZero differs from SameValue only in that it treats *+0*_𝔽 and *-0*_𝔽 as equivalent.

--- referenced: NormalCompletion ( _value_: any value except a Completion Record, ): a normal completion
  id: sec-normalcompletion

skip return checks: true

1. Return Completion Record { [[Type]]: ~normal~, [[Value]]: _value_, [[Target]]: ~empty~ }.


## Done when

- Every test262 run under built-ins/Iterator/prototype/includes/ passes (check with `./t262 built-ins/Iterator/prototype/includes/`). 8 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 529: worker reported BLOCKED but changed files: none
- attempt 529: accepted, but conflicts with newer work on the stack; continue from this diff on the new base

