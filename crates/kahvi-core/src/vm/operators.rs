//! Operators: ApplyStringOrNumericBinaryOperator (§13.15.3), relational and
//! equality operators, `typeof`, and the unary numeric operators.

use crate::agent::Agent;
use crate::compiler::BinOp;
use crate::error::JsResult;
use crate::ops::{self, Numeric, PreferredType};
use crate::value::{JsString, Value};

/// `typeof` (§13.5.3.1 table)
pub fn type_of(agent: &mut Agent, v: Value) -> JsString {
    let s = match v {
        Value::Undefined => "undefined",
        Value::Null => "object",
        Value::Boolean(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Symbol(_) => "symbol",
        Value::BigInt(_) => "bigint",
        Value::Object(o) => {
            if o.is_callable(agent) {
                "function"
            } else {
                "object"
            }
        }
    };
    agent.string(s)
}

/// Number::exponentiate(base, exponent) (§6.1.6.1.3)
pub fn number_exponentiate(base: f64, exponent: f64) -> f64 {
    // 1. If exponent is NaN, return NaN.
    if exponent.is_nan() {
        return f64::NAN;
    }
    // 2. If exponent is either +0 or -0, return 1.
    if exponent == 0.0 {
        return 1.0;
    }
    // 3. If base is NaN, return NaN.
    if base.is_nan() {
        return f64::NAN;
    }
    // 9-10. abs(base) = 1 with an infinite exponent is NaN.
    if base.abs() == 1.0 && exponent.is_infinite() {
        return f64::NAN;
    }
    base.powf(exponent)
}

fn number_shift(op: BinOp, l: f64, r: f64) -> f64 {
    let shift = ops::f64_to_uint32(r) & 31;
    match op {
        BinOp::Shl => ops::f64_to_int32(l).wrapping_shl(shift) as f64,
        BinOp::Shr => (ops::f64_to_int32(l) >> shift) as f64,
        BinOp::UShr => (ops::f64_to_uint32(l) >> shift) as f64,
        _ => unreachable!(),
    }
}

/// The Number operation for a numeric binary operator.
fn number_op(op: BinOp, l: f64, r: f64) -> f64 {
    match op {
        BinOp::Add => l + r,
        BinOp::Sub => l - r,
        BinOp::Mul => l * r,
        BinOp::Div => l / r,
        // Number::remainder: truncating, with the dividend's sign (as Rust's %).
        BinOp::Mod => l % r,
        BinOp::Exp => number_exponentiate(l, r),
        BinOp::Shl | BinOp::Shr | BinOp::UShr => number_shift(op, l, r),
        BinOp::BitAnd => (ops::f64_to_int32(l) & ops::f64_to_int32(r)) as f64,
        BinOp::BitOr => (ops::f64_to_int32(l) | ops::f64_to_int32(r)) as f64,
        BinOp::BitXor => (ops::f64_to_int32(l) ^ ops::f64_to_int32(r)) as f64,
        _ => unreachable!("not a numeric operator"),
    }
}

/// ApplyStringOrNumericBinaryOperator(lval, opText, rval) (§13.15.3)
fn apply_string_or_numeric(
    agent: &mut Agent,
    op: BinOp,
    lval: Value,
    rval: Value,
) -> JsResult<Value> {
    let (lval, rval) = if op == BinOp::Add {
        // 1.a-b Let lprim be ? ToPrimitive(lval); let rprim be ? ToPrimitive(rval).
        let lprim = ops::to_primitive(agent, lval, PreferredType::Default)?;
        let rprim = ops::to_primitive(agent, rval, PreferredType::Default)?;
        // c. If lprim is a String or rprim is a String, then concatenate.
        if matches!(lprim, Value::String(_)) || matches!(rprim, Value::String(_)) {
            let ls = ops::to_string(agent, lprim)?;
            let rs = ops::to_string(agent, rprim)?;
            return Ok(Value::String(agent.concat(ls, rs)));
        }
        (lprim, rprim)
    } else {
        (lval, rval)
    };
    // 3. Let lnum be ? ToNumeric(lval). 4. Let rnum be ? ToNumeric(rval).
    let lnum = ops::to_numeric(agent, lval)?;
    let rnum = ops::to_numeric(agent, rval)?;
    match (lnum, rnum) {
        (Numeric::Number(l), Numeric::Number(r)) => Ok(Value::Number(number_op(op, l, r))),
        // 6. If lnum is a BigInt, then ... Let operation be the abstract
        //    operation for opText and Type(lnum) from Table 39.
        (Numeric::BigInt(l), Numeric::BigInt(r)) => {
            let name = match op {
                BinOp::Add => "add",
                BinOp::Sub => "subtract",
                BinOp::Mul => "multiply",
                BinOp::Div => "divide",
                BinOp::Mod => "remainder",
                BinOp::Exp => "exponentiate",
                BinOp::Shl => "leftShift",
                BinOp::Shr => "signedRightShift",
                BinOp::UShr => "unsignedRightShift",
                BinOp::BitAnd => "bitwiseAND",
                BinOp::BitOr => "bitwiseOR",
                BinOp::BitXor => "bitwiseXOR",
                _ => unreachable!("not a numeric operator"),
            };
            Ok(Value::BigInt(ops::bigint::binary(agent, name, l, r)?))
        }
        // 5. If Type(lnum) is not Type(rnum), throw a TypeError exception.
        _ => {
            Err(agent
                .throw_type_error("Cannot mix BigInt and other types, use explicit conversions"))
        }
    }
}

/// Evaluates a binary operator (§13.6-13.12).
pub fn apply_binary_operator(agent: &mut Agent, op: BinOp, l: Value, r: Value) -> JsResult<Value> {
    // Fast path for the common Number case.
    if let (Value::Number(a), Value::Number(b)) = (l, r)
        && let Some(v) = number_binary(op, a, b)
    {
        return Ok(v);
    }
    apply_slow(agent, op, l, r)
}

/// A binary operator on two Numbers, where that cannot run user code (all
/// but `in` and `instanceof`, which need an object).
#[inline(always)]
pub(crate) fn number_binary(op: BinOp, a: f64, b: f64) -> Option<Value> {
    Some(match op {
        BinOp::Lt => Value::Boolean(a < b),
        BinOp::Gt => Value::Boolean(a > b),
        BinOp::LtEq => Value::Boolean(a <= b),
        BinOp::GtEq => Value::Boolean(a >= b),
        BinOp::StrictEq | BinOp::Eq => Value::Boolean(a == b),
        BinOp::StrictNotEq | BinOp::NotEq => Value::Boolean(a != b),
        BinOp::In | BinOp::InstanceOf => return None,
        _ => Value::Number(number_op(op, a, b)),
    })
}

fn apply_slow(agent: &mut Agent, op: BinOp, l: Value, r: Value) -> JsResult<Value> {
    Ok(match op {
        BinOp::Eq => Value::Boolean(ops::is_loosely_equal(agent, l, r)?),
        BinOp::NotEq => Value::Boolean(!ops::is_loosely_equal(agent, l, r)?),
        BinOp::StrictEq => Value::Boolean(ops::is_strictly_equal(agent, l, r)),
        BinOp::StrictNotEq => Value::Boolean(!ops::is_strictly_equal(agent, l, r)),
        // RelationalExpression < : r = IsLessThan(lval, rval, true); undefined is false.
        BinOp::Lt => Value::Boolean(ops::is_less_than(agent, l, r, true)? == Some(true)),
        // > : r = IsLessThan(rval, lval, false).
        BinOp::Gt => Value::Boolean(ops::is_less_than(agent, r, l, false)? == Some(true)),
        // <= : r = IsLessThan(rval, lval, false); true or undefined gives false.
        BinOp::LtEq => Value::Boolean(ops::is_less_than(agent, r, l, false)? == Some(false)),
        // >= : r = IsLessThan(lval, rval, true); true or undefined gives false.
        BinOp::GtEq => Value::Boolean(ops::is_less_than(agent, l, r, true)? == Some(false)),
        BinOp::InstanceOf => Value::Boolean(ops::instance_of_operator(agent, l, r)?),
        BinOp::In => {
            // 5. If rval is not an Object, throw a TypeError exception.
            let Value::Object(o) = r else {
                return Err(agent.throw_type_error(
                    "Cannot use 'in' operator to search for a key in a non-object",
                ));
            };
            // 6. Return ? HasProperty(rval, ? ToPropertyKey(lval)).
            let key = ops::to_property_key(agent, l)?;
            Value::Boolean(ops::has_property(agent, o, key)?)
        }
        _ => return apply_string_or_numeric(agent, op, l, r),
    })
}

/// Unary `-` (§13.5.5.1)
pub fn unary_minus(agent: &mut Agent, v: Value) -> JsResult<Value> {
    match ops::to_numeric(agent, v)? {
        Numeric::Number(n) => Ok(Value::Number(-n)),
        Numeric::BigInt(b) => {
            // BigInt::unaryMinus: -0n is 0n.
            let mut value = agent.bigint(b).clone();
            if !value.magnitude.is_empty() {
                value.negative = !value.negative;
            }
            Ok(Value::BigInt(agent.new_bigint(value)))
        }
    }
}

/// Unary `~` (§13.5.6.1)
pub fn bitwise_not(agent: &mut Agent, v: Value) -> JsResult<Value> {
    match ops::to_numeric(agent, v)? {
        Numeric::Number(n) => Ok(Value::Number(!ops::f64_to_int32(n) as f64)),
        Numeric::BigInt(b) => {
            // BigInt::bitwiseNOT(x): -x - 1.
            let value = agent.bigint(b).not();
            Ok(Value::BigInt(agent.new_bigint(value)))
        }
    }
}
