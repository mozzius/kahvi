//! Number ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number-constructor-number-value

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// Number
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If value is present, then
    let n = match args.first() {
        Some(&value) => match ops::to_numeric(agent, value)? {
            // b. If prim is a BigInt, let n be 𝔽(ℝ(prim)).
            ops::Numeric::BigInt(b) => agent.bigint(b).to_f64(),
            // c. Otherwise, let n be prim.
            ops::Numeric::Number(n) => n,
        },
        // 2. Else, let n be +0𝔽.
        None => 0.0,
    };
    // 3. If NewTarget is undefined, return n.
    let Some(new_target) = new_target else {
        return Ok(Value::Number(n));
    };
    // 4-5. Let O be ? OrdinaryCreateFromConstructor(NewTarget, "%Number.prototype%", « [[NumberData]] »); set O.[[NumberData]] to n.
    let o = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Number.prototype%",
        ObjectKind::NumberObject(n),
    )?;
    // 6. Return O.
    Ok(Value::Object(o))
}
