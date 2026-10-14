//! Math.fround ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.fround

use kahvi_core::prelude::*;

/// Math.fround
pub(crate) fn fround(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _n_ be ? ToNumber(_x_).
    // 2. If _n_ is *NaN*, return *NaN*.
    // 3. If _n_ is one of *+0*_𝔽, *-0*_𝔽, *+∞*_𝔽, or *-∞*_𝔽, return _n_.
    // 4. Let _n32_ be the result of converting _n_ to IEEE 754-2019 binary32 format using roundTiesToEven mode.
    // 5. Let _n64_ be the result of converting _n32_ to IEEE 754-2019 binary64 format.
    // 6. Return the ECMAScript Number value corresponding to _n64_.
    let _ = (this, args, new_target);
    // 1. Let _n_ be ? ToNumber(_x_).
    let n = ops::to_number(agent, args.get_or_undefined(0))?;
    // 2. If _n_ is *NaN*, return *NaN*.
    if n.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 3. If _n_ is one of *+0*_𝔽, *-0*_𝔽, *+∞*_𝔽, or *-∞*_𝔽, return _n_.
    if n == 0.0 || n.is_infinite() {
        return Ok(Value::Number(n));
    }
    // 4. Let _n32_ be the result of converting _n_ to IEEE 754-2019 binary32 format using roundTiesToEven mode.
    // Rust's `as` cast from f64 to f32 rounds to nearest, ties to even.
    let n32 = n as f32;
    // 5. Let _n64_ be the result of converting _n32_ to IEEE 754-2019 binary64 format.
    let n64 = f64::from(n32);
    // 6. Return the ECMAScript Number value corresponding to _n64_.
    Ok(Value::Number(n64))
}
