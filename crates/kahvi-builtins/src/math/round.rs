//! Math.round ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.round

use kahvi_core::prelude::*;

/// Math.round
pub(crate) fn round(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the Number value that is closest to _x_ and is integral. If two integral Numbers are equally close to _x_, then the result is the Number value that is closer to +∞. If _x_ is already integral, the result is _x_.
    // It performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is not a finite Number or _n_ is an integral Number, return _n_.
    // 3. If _n_ < *0.5*_𝔽 and _n_ > *+0*_𝔽, return *+0*_𝔽.
    // 4. If _n_ < *-0*_𝔽 and _n_ ≥ *-0.5*_𝔽, return *-0*_𝔽.
    // 5. Return the integral Number closest to _n_, preferring the Number closer to +∞ in the case of a tie.
    //
    // NOTE:
    // `Math.round(3.5)` returns 4, but `Math.round(-3.5)` returns -3.
    // NOTE:
    // The value of `Math.round(x)` is not always the same as the value of `Math.floor(x + 0.5)`. When `x` is *-0*_𝔽 or `x` is less than *-0*_𝔽 but greater than or equal to *-0.5*_𝔽, `Math.round(x)` returns *-0*_𝔽, but `Math.floor(x + 0.5)` returns *+0*_𝔽. `Math.round(x)` may also differ from the value of `Math.floor(x + 0.5)`because of internal rounding when computing `x + 0.5`.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.round"))
}
