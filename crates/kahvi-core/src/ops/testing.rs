//! Testing and comparison operations (ECMA-262 §7.2).

use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{ObjectKind, PropertyKey};
use crate::ops::conversion::{PreferredType, string_to_number, to_number, to_primitive};
use crate::ops::{bigint, get, to_boolean};
use crate::value::{Object, Value};

/// RequireObjectCoercible(argument) (§7.2.1)
pub fn require_object_coercible(agent: &mut Agent, v: Value) -> JsResult<Value> {
    if v.is_nullish() {
        return Err(agent.throw_type_error("Cannot convert undefined or null to object"));
    }
    Ok(v)
}

/// IsArray(argument) (§7.2.2)
pub fn is_array(agent: &mut Agent, v: Value) -> JsResult<bool> {
    // 1. If argument is not an Object, return false.
    let Value::Object(o) = v else {
        return Ok(false);
    };
    // 2. If argument is an Array exotic object, return true.
    if matches!(agent.obj(o).kind, ObjectKind::Array) {
        return Ok(true);
    }
    // 3. If argument is a Proxy exotic object, then
    if let Some(p) = crate::object::proxy::proxy_data(agent, o) {
        // a. Perform ? ValidateNonRevokedProxy(argument).
        let Some(target) = p.target else {
            return Err(agent.throw_type_error("proxy has been revoked"));
        };
        // b-c. Return ? IsArray(argument.[[ProxyTarget]]).
        return is_array(agent, Value::Object(target));
    }
    // 4. Return false.
    Ok(false)
}

/// IsCallable(argument) (§7.2.3)
pub fn is_callable(agent: &Agent, v: Value) -> bool {
    matches!(v, Value::Object(o) if o.is_callable(agent))
}

/// IsConstructor(argument) (§7.2.4)
pub fn is_constructor(agent: &Agent, v: Value) -> bool {
    matches!(v, Value::Object(o) if o.is_constructor(agent))
}

/// IsExtensible(O) (§7.2.5)
pub fn is_extensible(agent: &mut Agent, o: Object) -> JsResult<bool> {
    o.internal_is_extensible(agent)
}

/// IsIntegralNumber(argument) (§7.2.6)
pub fn is_integral_number(v: Value) -> bool {
    matches!(v, Value::Number(n) if n.is_finite() && n.trunc() == n)
}

/// IsRegExp(argument) (§7.2.8)
pub fn is_regexp(agent: &mut Agent, v: Value) -> JsResult<bool> {
    // 1. If argument is not an Object, return false.
    let Value::Object(o) = v else {
        return Ok(false);
    };
    // 2. Let matcher be ? Get(argument, %Symbol.match%).
    let matcher = get(agent, o, PropertyKey::Symbol(agent.symbols_wk.r#match))?;
    // 3. If matcher is not undefined, return ToBoolean(matcher).
    if !matcher.is_undefined() {
        return Ok(to_boolean(agent, matcher));
    }
    // 4. If argument has a [[RegExpMatcher]] internal slot, return true.
    // 5. Return false.
    Ok(crate::ops::has_regexp_matcher(agent, o))
}

/// SameValue(x, y) (§7.2.9)
pub fn same_value(agent: &Agent, x: Value, y: Value) -> bool {
    match (x, y) {
        // 2. If x is a Number, return Number::sameValue(x, y).
        (Value::Number(a), Value::Number(b)) => {
            if a.is_nan() && b.is_nan() {
                return true;
            }
            a == b && a.is_sign_negative() == b.is_sign_negative()
        }
        // 3. Return SameValueNonNumber(x, y).
        _ => same_value_non_number(agent, x, y),
    }
}

/// SameValueZero(x, y) (§7.2.10)
pub fn same_value_zero(agent: &Agent, x: Value, y: Value) -> bool {
    match (x, y) {
        (Value::Number(a), Value::Number(b)) => (a.is_nan() && b.is_nan()) || a == b,
        _ => same_value_non_number(agent, x, y),
    }
}

/// SameValueNonNumber(x, y) (§7.2.11)
pub fn same_value_non_number(agent: &Agent, x: Value, y: Value) -> bool {
    match (x, y) {
        (Value::Undefined, Value::Undefined) | (Value::Null, Value::Null) => true,
        (Value::BigInt(a), Value::BigInt(b)) => bigint::equal(agent, a, b),
        // Strings are interned: equal contents means equal handles.
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Boolean(a), Value::Boolean(b)) => a == b,
        (Value::Symbol(a), Value::Symbol(b)) => a == b,
        (Value::Object(a), Value::Object(b)) => a == b,
        _ => false,
    }
}

/// IsLessThan(x, y, LeftFirst) (§7.2.13). `None` means undefined.
pub fn is_less_than(
    agent: &mut Agent,
    x: Value,
    y: Value,
    left_first: bool,
) -> JsResult<Option<bool>> {
    // 1-2. Convert to primitives in the order LeftFirst says.
    let (px, py) = if left_first {
        let px = to_primitive(agent, x, PreferredType::Number)?;
        let py = to_primitive(agent, y, PreferredType::Number)?;
        (px, py)
    } else {
        let py = to_primitive(agent, y, PreferredType::Number)?;
        let px = to_primitive(agent, x, PreferredType::Number)?;
        (px, py)
    };
    // 3. If px is a String and py is a String, then compare code units.
    if let (Value::String(a), Value::String(b)) = (px, py) {
        return Ok(Some(agent.str(a) < agent.str(b)));
    }
    // 4. Else,
    // a. If px is a BigInt and py is a String, then
    if let (Value::BigInt(a), Value::String(b)) = (px, py) {
        let Some(ny) = bigint::string_to_bigint(agent, b) else {
            return Ok(None);
        };
        return Ok(Some(bigint::less_than(agent, a, ny)));
    }
    // b. If px is a String and py is a BigInt, then
    if let (Value::String(a), Value::BigInt(b)) = (px, py) {
        let Some(nx) = bigint::string_to_bigint(agent, a) else {
            return Ok(None);
        };
        return Ok(Some(bigint::less_than(agent, nx, b)));
    }
    // c-d. Let nx be ? ToNumeric(px), ny be ? ToNumeric(py).
    let nx = crate::ops::to_numeric(agent, px)?;
    let ny = crate::ops::to_numeric(agent, py)?;
    use crate::ops::conversion::Numeric;
    Ok(match (nx, ny) {
        // e. If Type(nx) is Type(ny), then Number::lessThan / BigInt::lessThan.
        (Numeric::Number(a), Numeric::Number(b)) => {
            if a.is_nan() || b.is_nan() {
                None
            } else {
                Some(a < b)
            }
        }
        (Numeric::BigInt(a), Numeric::BigInt(b)) => Some(bigint::less_than(agent, a, b)),
        // g-l. Mixed BigInt and Number.
        (Numeric::BigInt(a), Numeric::Number(b)) => {
            bigint::compare_number(agent, a, b).map(|o| o == std::cmp::Ordering::Less)
        }
        (Numeric::Number(a), Numeric::BigInt(b)) => {
            bigint::compare_number(agent, b, a).map(|o| o == std::cmp::Ordering::Greater)
        }
    })
}

/// IsLooselyEqual(x, y) (§7.2.14)
pub fn is_loosely_equal(agent: &mut Agent, x: Value, y: Value) -> JsResult<bool> {
    // 1. If Type(x) is Type(y), return IsStrictlyEqual(x, y).
    if std::mem::discriminant(&x) == std::mem::discriminant(&y) {
        return Ok(is_strictly_equal(agent, x, y));
    }
    match (x, y) {
        // 2-3. null and undefined are loosely equal.
        (Value::Null, Value::Undefined) | (Value::Undefined, Value::Null) => Ok(true),
        // 5. Number and String: compare x with ! ToNumber(y).
        (Value::Number(_), Value::String(s)) => {
            let n = string_to_number(agent.str(s));
            Ok(is_strictly_equal(agent, x, Value::Number(n)))
        }
        // 6. String and Number.
        (Value::String(s), Value::Number(_)) => {
            let n = string_to_number(agent.str(s));
            Ok(is_strictly_equal(agent, Value::Number(n), y))
        }
        // 7. BigInt and String.
        (Value::BigInt(a), Value::String(s)) => match bigint::string_to_bigint(agent, s) {
            Some(b) => Ok(bigint::equal(agent, a, b)),
            None => Ok(false),
        },
        // 8. String and BigInt.
        (Value::String(_), Value::BigInt(_)) => is_loosely_equal(agent, y, x),
        // 9-10. Boolean: compare ! ToNumber(x) with y.
        (Value::Boolean(b), _) => {
            is_loosely_equal(agent, Value::Number(if b { 1.0 } else { 0.0 }), y)
        }
        (_, Value::Boolean(b)) => {
            is_loosely_equal(agent, x, Value::Number(if b { 1.0 } else { 0.0 }))
        }
        // 11. x is String, Number, BigInt or Symbol and y is an Object.
        (
            Value::String(_) | Value::Number(_) | Value::BigInt(_) | Value::Symbol(_),
            Value::Object(_),
        ) => {
            let py = to_primitive(agent, y, PreferredType::Default)?;
            is_loosely_equal(agent, x, py)
        }
        // 12. x is an Object and y is String, Number, BigInt or Symbol.
        (
            Value::Object(_),
            Value::String(_) | Value::Number(_) | Value::BigInt(_) | Value::Symbol(_),
        ) => {
            let px = to_primitive(agent, x, PreferredType::Default)?;
            is_loosely_equal(agent, px, y)
        }
        // 13. BigInt and Number.
        (Value::BigInt(a), Value::Number(n)) | (Value::Number(n), Value::BigInt(a)) => {
            if !n.is_finite() {
                return Ok(false);
            }
            Ok(bigint::compare_number(agent, a, n) == Some(std::cmp::Ordering::Equal))
        }
        // 14. Return false.
        _ => Ok(false),
    }
}

/// IsStrictlyEqual(x, y) (§7.2.15)
pub fn is_strictly_equal(agent: &Agent, x: Value, y: Value) -> bool {
    match (x, y) {
        (Value::Number(a), Value::Number(b)) => a == b,
        _ => same_value_non_number(agent, x, y),
    }
}

/// Number::equal for convenience.
pub fn number_equal(x: f64, y: f64) -> bool {
    x == y
}

/// ToNumber on a value already known not to need user code (for tests of primitives).
pub fn primitive_to_number(agent: &mut Agent, v: Value) -> JsResult<f64> {
    to_number(agent, v)
}
