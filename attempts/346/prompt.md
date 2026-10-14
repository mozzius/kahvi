# Ticket B-math-sum-precise: Math.sumPrecise

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

- `crates/kahvi-builtins/src/math/sum_precise.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.sumPrecise ( _items_ )
  id: sec-math.sumprecise   type: built-in function

Given an iterable of Numbers, this function sums each value in the iterable and returns their sum. If any value is not a Number it throws a *TypeError* exception.
It performs the following steps when called:

1. Perform ? RequireObjectCoercible(_items_).
2. Let _iteratorRecord_ be ? GetIterator(_items_, ~sync~).
3. Let _state_ be ~minus-zero~.
4. Let _sum_ be 0.
5. Let _count_ be 0.
6. Let _next_ be ~not-started~.
7. Repeat, while _next_ is not ~done~,
   a. Set _next_ to ? IteratorStepValue(_iteratorRecord_).
   b. If _next_ is not ~done~, then
      i. If _count_ ≥ 2^53 - 1, then
         1. NOTE: This step is not expected to be reached in practice and is included only so that implementations may rely on inputs being “reasonably sized” without violating this specification.
         2. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
         3. Return ? IteratorClose(_iteratorRecord_, _error_).
      ii. If _next_ is not a Number, then
         1. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
         2. Return ? IteratorClose(_iteratorRecord_, _error_).
      iii. Let _n_ be _next_.
      iv. If _state_ is not ~not-a-number~, then
         1. If _n_ is *NaN*, then
            a. Set _state_ to ~not-a-number~.
         2. Else if _n_ is *+∞*_𝔽, then
            a. If _state_ is ~minus-infinity~, set _state_ to ~not-a-number~.
            b. Else, set _state_ to ~plus-infinity~.
         3. Else if _n_ is *-∞*_𝔽, then
            a. If _state_ is ~plus-infinity~, set _state_ to ~not-a-number~.
            b. Else, set _state_ to ~minus-infinity~.
         4. Else if _n_ is not *-0*_𝔽 and _state_ is either ~minus-zero~ or ~finite~, then
            a. Set _state_ to ~finite~.
            b. Set _sum_ to _sum_ + ℝ(_n_).
      v. Set _count_ to _count_ + 1.
8. If _state_ is ~not-a-number~, return *NaN*.
9. If _state_ is ~plus-infinity~, return *+∞*_𝔽.
10. If _state_ is ~minus-infinity~, return *-∞*_𝔽.
11. If _state_ is ~minus-zero~, return *-0*_𝔽.
12. Return 𝔽(_sum_).

NOTE:
The value of _sum_ can be computed without arbitrary-precision arithmetic by a variety of algorithms. One such is the “Grow-Expansion” algorithm given in Adaptive Precision Floating-Point Arithmetic and Fast Robust Geometric Predicates by Jonathan Richard Shewchuk. A more recent algorithm is given in “Fast exact summation using small and large superaccumulators”, code for which is available at https://gitlab.com/radfordneal/xsum.

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

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


## Done when

- Every test262 run under built-ins/Math/sumPrecise/ passes (check with `./t262 built-ins/Math/sumPrecise/`). 8 pass on the main branch right now.

