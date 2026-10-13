//! Number ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number-constructor-number-value

use kahvi_core::prelude::*;

/// Number
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _value_ is present, then
    //    a. Let _primitive_ be ? ToNumeric(_value_).
    //    b. If _primitive_ is a BigInt, let _n_ be 𝔽(ℝ(_primitive_)).
    //    c. Else, let _n_ be _primitive_.
    // 2. Else,
    //    a. Let _n_ be *+0*_𝔽.
    // 3. If NewTarget is *undefined*, return _n_.
    // 4. Let _obj_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Number.prototype%"*, « [[NumberData]] »).
    // 5. Set _obj_.[[NumberData]] to _n_.
    // 6. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number"))
}
