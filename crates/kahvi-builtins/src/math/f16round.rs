//! Math.f16round ( _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.f16round

use kahvi_core::prelude::*;

/// Math.f16round
pub(crate) fn f16round(
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
    // 4. Let _n16_ be the result of converting _n_ to IEEE 754-2019 binary16 format using roundTiesToEven mode.
    // 5. Let _n64_ be the result of converting _n16_ to IEEE 754-2019 binary64 format.
    // 6. Return the ECMAScript Number value corresponding to _n64_.
    //
    // NOTE:
    // This operation is not the same as casting to binary32 and then to binary16 because of the possibility of double-rounding: consider the number _k_ = *1.00048828125000022204*_𝔽, for example, for which Math.f16round(_k_) is *1.0009765625*_𝔽, but Math.f16round(Math.fround(_k_)) is *1*_𝔽.
    // Not all platforms provide native support for casting from binary64 to binary16. There are various libraries which can provide this, including the MIT-licensed half library. Alternatively, it is possible to first cast from binary64 to binary32 under roundTiesToEven and then check whether the result could lead to incorrect double-rounding. Such cases can be handled explicitly by adjusting the mantissa of the binary32 value so that it is the value which would be produced by performing the initial cast under roundTiesToOdd. Casting the adjusted value to binary16 under roundTiesToEven then produces the correct value.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.f16round"))
}
