//! Math.atan2 ( _y_, _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.atan2

use kahvi_core::prelude::*;

/// Math.atan2
pub(crate) fn atan2(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the inverse tangent of the quotient _y_ / _x_ of the arguments _y_ and _x_, where the signs of _y_ and _x_ are used to determine the quadrant of the result. Note that it is intentional and traditional for the two-argument inverse tangent function that the argument named _y_ be first and the argument named _x_ be second. The result is expressed in radians and is in the inclusive interval from -π to +π.
    // It performs the following steps when called:
    //
    // 1. Let _ny_ be ? ToNumber(_y_).
    // 2. Let _nx_ be ? ToNumber(_x_).
    // 3. If _ny_ is *NaN* or _nx_ is *NaN*, return *NaN*.
    // 4. If _ny_ is *+∞*_𝔽, then
    //    a. If _nx_ is *+∞*_𝔽, return an implementation-approximated Number value representing π / 4.
    //    b. If _nx_ is *-∞*_𝔽, return an implementation-approximated Number value representing 3π / 4.
    //    c. Return an implementation-approximated Number value representing π / 2.
    // 5. If _ny_ is *-∞*_𝔽, then
    //    a. If _nx_ is *+∞*_𝔽, return an implementation-approximated Number value representing -π / 4.
    //    b. If _nx_ is *-∞*_𝔽, return an implementation-approximated Number value representing -3π / 4.
    //    c. Return an implementation-approximated Number value representing -π / 2.
    // 6. If _ny_ is *+0*_𝔽, then
    //    a. If _nx_ > *+0*_𝔽 or _nx_ is *+0*_𝔽, return *+0*_𝔽.
    //    b. Return an implementation-approximated Number value representing π.
    // 7. If _ny_ is *-0*_𝔽, then
    //    a. If _nx_ > *+0*_𝔽 or _nx_ is *+0*_𝔽, return *-0*_𝔽.
    //    b. Return an implementation-approximated Number value representing -π.
    // 8. Assert: _ny_ is a finite Number and is neither *+0*_𝔽 nor *-0*_𝔽.
    // 9. If _ny_ > *+0*_𝔽, then
    //    a. If _nx_ is *+∞*_𝔽, return *+0*_𝔽.
    //    b. If _nx_ is *-∞*_𝔽, return an implementation-approximated Number value representing π.
    //    c. If _nx_ is either *+0*_𝔽 or *-0*_𝔽, return an implementation-approximated Number value representing π / 2.
    // 10. If _ny_ < *-0*_𝔽, then
    //    a. If _nx_ is *+∞*_𝔽, return *-0*_𝔽.
    //    b. If _nx_ is *-∞*_𝔽, return an implementation-approximated Number value representing -π.
    //    c. If _nx_ is either *+0*_𝔽 or *-0*_𝔽, return an implementation-approximated Number value representing -π / 2.
    // 11. Assert: _nx_ is a finite Number and is neither *+0*_𝔽 nor *-0*_𝔽.
    // 12. Let _result_ be the inverse tangent of abs(ℝ(_ny_) / ℝ(_nx_)).
    // 13. If _nx_ < *-0*_𝔽, then
    //    a. If _ny_ > *+0*_𝔽, set _result_ to π - _result_.
    //    b. Else, set _result_ to -π + _result_.
    // 14. Else,
    //    a. If _ny_ < *-0*_𝔽, set _result_ to -_result_.
    // 15. Return an implementation-approximated Number value representing _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.atan2"))
}
