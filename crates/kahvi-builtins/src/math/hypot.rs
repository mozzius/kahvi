//! Math.hypot ( ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.hypot

use kahvi_core::prelude::*;

/// Math.hypot
pub(crate) fn hypot(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // Given zero or more arguments, this function returns the square root of the sum of squares of its arguments.
    // It performs the following steps when called:
    //
    // 1. Let _coerced_ be a new empty List.
    // 2. For each element _arg_ of _args_, do
    //    a. Let _n_ be ? ToNumber(_arg_).
    //    b. Append _n_ to _coerced_.
    // 3. For each element _number_ of _coerced_, do
    //    a. If _number_ is either *+∞*_𝔽 or *-∞*_𝔽, return *+∞*_𝔽.
    // 4. Let _onlyZero_ be *true*.
    // 5. For each element _number_ of _coerced_, do
    //    a. If _number_ is *NaN*, return *NaN*.
    //    b. If _number_ is neither *+0*_𝔽 nor *-0*_𝔽, set _onlyZero_ to *false*.
    // 6. If _onlyZero_ is *true*, return *+0*_𝔽.
    // 7. Return an implementation-approximated Number value representing the square root of the sum of squares of the mathematical values of the elements of _coerced_.
    //
    // The *"length"* property of this function is *2*_𝔽.
    // NOTE:
    // Implementations should take care to avoid the loss of precision from overflows and underflows that are prone to occur in naive implementations when this function is called with two or more arguments.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.hypot"))
}
