//! Math.sumPrecise ( _items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.sumprecise

use kahvi_core::prelude::*;

/// Math.sumPrecise
pub(crate) fn sum_precise(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // Given an iterable of Numbers, this function sums each value in the iterable and returns their sum. If any value is not a Number it throws a *TypeError* exception.
    // It performs the following steps when called:
    //
    // 1. Perform ? RequireObjectCoercible(_items_).
    // 2. Let _iteratorRecord_ be ? GetIterator(_items_, ~sync~).
    // 3. Let _state_ be ~minus-zero~.
    // 4. Let _sum_ be 0.
    // 5. Let _count_ be 0.
    // 6. Let _next_ be ~not-started~.
    // 7. Repeat, while _next_ is not ~done~,
    //    a. Set _next_ to ? IteratorStepValue(_iteratorRecord_).
    //    b. If _next_ is not ~done~, then
    //       i. If _count_ ≥ 2^53 - 1, then
    //          1. NOTE: This step is not expected to be reached in practice and is included only so that implementations may rely on inputs being “reasonably sized” without violating this specification.
    //          2. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
    //          3. Return ? IteratorClose(_iteratorRecord_, _error_).
    //       ii. If _next_ is not a Number, then
    //          1. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
    //          2. Return ? IteratorClose(_iteratorRecord_, _error_).
    //       iii. Let _n_ be _next_.
    //       iv. If _state_ is not ~not-a-number~, then
    //          1. If _n_ is *NaN*, then
    //             a. Set _state_ to ~not-a-number~.
    //          2. Else if _n_ is *+∞*_𝔽, then
    //             a. If _state_ is ~minus-infinity~, set _state_ to ~not-a-number~.
    //             b. Else, set _state_ to ~plus-infinity~.
    //          3. Else if _n_ is *-∞*_𝔽, then
    //             a. If _state_ is ~plus-infinity~, set _state_ to ~not-a-number~.
    //             b. Else, set _state_ to ~minus-infinity~.
    //          4. Else if _n_ is not *-0*_𝔽 and _state_ is either ~minus-zero~ or ~finite~, then
    //             a. Set _state_ to ~finite~.
    //             b. Set _sum_ to _sum_ + ℝ(_n_).
    //       v. Set _count_ to _count_ + 1.
    // 8. If _state_ is ~not-a-number~, return *NaN*.
    // 9. If _state_ is ~plus-infinity~, return *+∞*_𝔽.
    // 10. If _state_ is ~minus-infinity~, return *-∞*_𝔽.
    // 11. If _state_ is ~minus-zero~, return *-0*_𝔽.
    // 12. Return 𝔽(_sum_).
    //
    // NOTE:
    // The value of _sum_ can be computed without arbitrary-precision arithmetic by a variety of algorithms. One such is the “Grow-Expansion” algorithm given in Adaptive Precision Floating-Point Arithmetic and Fast Robust Geometric Predicates by Jonathan Richard Shewchuk. A more recent algorithm is given in “Fast exact summation using small and large superaccumulators”, code for which is available at https://gitlab.com/radfordneal/xsum.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.sumPrecise"))
}
