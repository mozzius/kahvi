//! Object.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.tostring

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

use crate::support::date::DateValue;

/// Object.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If the this value is undefined, return "[object Undefined]".
    // 2. If the this value is null, return "[object Null]".
    let builtin_tag = match this {
        Value::Undefined => return Ok(Value::String(agent.string("[object Undefined]"))),
        Value::Null => return Ok(Value::String(agent.string("[object Null]"))),
        _ => "",
    };
    // 3. Let O be ! ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 4-14. builtinTag from the object's internal slots.
    let builtin_tag = if ops::is_array(agent, Value::Object(o))? {
        "Array"
    } else {
        match &agent.obj(o).kind {
            ObjectKind::Arguments(_) => "Arguments",
            ObjectKind::Error => "Error",
            ObjectKind::BooleanObject(_) => "Boolean",
            ObjectKind::NumberObject(_) => "Number",
            ObjectKind::StringObject(_) => "String",
            _ if o.is_callable(agent) => "Function",
            _ if ops::has_regexp_matcher(agent, o) => "RegExp",
            _ if agent.native::<DateValue>(o).is_some() => "Date",
            _ => {
                if builtin_tag.is_empty() {
                    "Object"
                } else {
                    builtin_tag
                }
            }
        }
    };
    // 15. Let tag be ? Get(O, %Symbol.toStringTag%).
    let tag_key = PropertyKey::Symbol(agent.symbols_wk.to_string_tag);
    let tag = ops::get(agent, o, tag_key)?;
    // 16. If tag is not a String, set tag to builtinTag.
    let tag = match tag {
        Value::String(s) => s,
        _ => agent.string(builtin_tag),
    };
    // 17. Return the string-concatenation of "[object ", tag, and "]".
    let mut units: Vec<u16> = "[object ".encode_utf16().collect();
    units.extend_from_slice(agent.str(tag));
    units.push(u16::from(b']'));
    Ok(Value::String(agent.intern(&units)))
}
