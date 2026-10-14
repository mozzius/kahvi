//! Math.min ( ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.min

use kahvi_core::prelude::*;

/// Math.min
pub(crate) fn min(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // Given zero or more arguments, this function calls ToNumber on each of the arguments and returns the smallest of the resulting values.
    // It performs the following steps when called:
    //
    // 1. Let _coerced_ be a new empty List.
    // 2. For each element _arg_ of _args_, do
    //    a. Let _n_ be ? ToNumber(_arg_).
    //    b. Append _n_ to _coerced_.
    // 3. Let _lowest_ be *+∞*_𝔽.
    // 4. For each element _number_ of _coerced_, do
    //    a. If _number_ is *NaN*, return *NaN*.
    //    b. If _number_ is *-0*_𝔽 and _lowest_ is *+0*_𝔽, set _lowest_ to *-0*_𝔽.
    //    c. If _number_ < _lowest_, set _lowest_ to _number_.
    // 5. Return _lowest_.
    //
    // NOTE:
    // The comparison of values to determine the smallest value is done using the IsLessThan algorithm except that *+0*_𝔽 is considered to be larger than *-0*_𝔽.
    // The *"length"* property of this function is *2*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.min"))
}
