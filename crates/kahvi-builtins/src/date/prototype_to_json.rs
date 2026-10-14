//! Date.prototype.toJSON ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.tojson

use kahvi_core::prelude::*;

/// Date.prototype.toJSON
pub(crate) fn prototype_to_json(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method provides a String representation of a Date for use by `JSON.stringify` ([sec-json.stringify]).
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _tv_ be ? ToPrimitive(_obj_, ~number~).
    // 3. If _tv_ is a Number and _tv_ is not a finite Number, return *null*.
    // 4. Return ? Invoke(_obj_, *"toISOString"*).
    //
    // NOTE:
    // The argument is ignored.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a Date. Therefore, it can be transferred to other kinds of objects for use as a method. However, it does require that any such object have a `toISOString` method.
    let _ = (args, new_target);
    // 1. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 2. Let tv be ? ToPrimitive(O, number).
    let tv = ops::to_primitive(agent, Value::Object(o), ops::PreferredType::Number)?;
    // 3. If tv is a Number and tv is not finite, return null.
    if let Value::Number(n) = tv {
        if !n.is_finite() {
            return Ok(Value::Null);
        }
    }
    // 4. Return ? Invoke(O, "toISOString").
    let key = agent.key("toISOString");
    ops::invoke(agent, Value::Object(o), key, &[])
}
