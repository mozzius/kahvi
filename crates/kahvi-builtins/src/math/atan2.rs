//! Math.atan2 ( _y_, _x_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.atan2

use kahvi_core::prelude::*;

/// Math.atan2
pub(crate) fn atan2(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    use std::f64::consts::PI;

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
    // 1. Let ny be ? ToNumber(y).
    let ny = ops::to_number(agent, args.get_or_undefined(0))?;
    // 2. Let nx be ? ToNumber(x).
    let nx = ops::to_number(agent, args.get_or_undefined(1))?;
    // 3. If ny is NaN or nx is NaN, return NaN.
    if ny.is_nan() || nx.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 4. If ny is +∞, then
    if ny == f64::INFINITY {
        // a. If nx is +∞, return an implementation-approximated Number value representing π / 4.
        if nx == f64::INFINITY {
            return Ok(Value::Number(PI / 4.0));
        }
        // b. If nx is -∞, return an implementation-approximated Number value representing 3π / 4.
        if nx == f64::NEG_INFINITY {
            return Ok(Value::Number(3.0 * PI / 4.0));
        }
        // c. Return an implementation-approximated Number value representing π / 2.
        return Ok(Value::Number(PI / 2.0));
    }
    // 5. If ny is -∞, then
    if ny == f64::NEG_INFINITY {
        // a. If nx is +∞, return an implementation-approximated Number value representing -π / 4.
        if nx == f64::INFINITY {
            return Ok(Value::Number(-PI / 4.0));
        }
        // b. If nx is -∞, return an implementation-approximated Number value representing -3π / 4.
        if nx == f64::NEG_INFINITY {
            return Ok(Value::Number(-3.0 * PI / 4.0));
        }
        // c. Return an implementation-approximated Number value representing -π / 2.
        return Ok(Value::Number(-PI / 2.0));
    }
    // 6. If ny is +0𝔽, then
    if ny == 0.0 && ny.is_sign_positive() {
        // a. If nx > +0𝔽 or nx is +0𝔽, return +0𝔽.
        if nx > 0.0 || (nx == 0.0 && nx.is_sign_positive()) {
            return Ok(Value::Number(0.0));
        }
        // b. Return an implementation-approximated Number value representing π.
        return Ok(Value::Number(PI));
    }
    // 7. If ny is -0𝔽, then
    if ny == 0.0 && ny.is_sign_negative() {
        // a. If nx > +0𝔽 or nx is +0𝔽, return -0𝔽.
        if nx > 0.0 || (nx == 0.0 && nx.is_sign_positive()) {
            return Ok(Value::Number(-0.0));
        }
        // b. Return an implementation-approximated Number value representing -π.
        return Ok(Value::Number(-PI));
    }
    // 8. Assert: ny is a finite Number and is neither +0𝔽 nor -0𝔽.
    // 9. If ny > +0𝔽, then
    if ny > 0.0 {
        // a. If nx is +∞, return +0𝔽.
        if nx == f64::INFINITY {
            return Ok(Value::Number(0.0));
        }
        // b. If nx is -∞, return an implementation-approximated Number value representing π.
        if nx == f64::NEG_INFINITY {
            return Ok(Value::Number(PI));
        }
        // c. If nx is either +0𝔽 or -0𝔽, return an implementation-approximated Number value representing π / 2.
        if nx == 0.0 {
            return Ok(Value::Number(PI / 2.0));
        }
    }
    // 10. If ny < -0𝔽, then
    if ny < 0.0 {
        // a. If nx is +∞, return -0𝔽.
        if nx == f64::INFINITY {
            return Ok(Value::Number(-0.0));
        }
        // b. If nx is -∞, return an implementation-approximated Number value representing -π.
        if nx == f64::NEG_INFINITY {
            return Ok(Value::Number(-PI));
        }
        // c. If nx is either +0𝔽 or -0𝔽, return an implementation-approximated Number value representing -π / 2.
        if nx == 0.0 {
            return Ok(Value::Number(-PI / 2.0));
        }
    }
    // 11. Assert: nx is a finite Number and is neither +0𝔽 nor -0𝔽.
    // 12. Let result be the inverse tangent of abs(ℝ(ny) / ℝ(nx)).
    let mut result = (ny / nx).abs().atan();
    // 13. If nx < -0𝔽, then
    if nx < 0.0 {
        // a. If ny > +0𝔽, set result to π - result.
        if ny > 0.0 {
            result = PI - result;
        } else {
            // b. Else, set result to -π + result.
            result = -PI + result;
        }
    } else {
        // 14. Else,
        // a. If ny < -0𝔽, set result to -result.
        if ny < 0.0 {
            result = -result;
        }
    }
    // 15. Return an implementation-approximated Number value representing result.
    Ok(Value::Number(result))
}
