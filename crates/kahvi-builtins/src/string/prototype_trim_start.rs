//! String.prototype.trimStart ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.trimstart

use kahvi_core::prelude::*;

use super::prototype_trim::{TrimWhere, trim_string};

/// String.prototype.trimStart
pub(crate) fn prototype_trim_start(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
    // It performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    // 2. Return ? TrimString(_string_, ~start~).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (args, new_target);
    // 1. Let _string_ be the *this* value.
    let string = this;
    // 2. Return ? TrimString(_string_, ~start~).
    let trimmed = trim_string(agent, string, TrimWhere::Start)?;
    Ok(Value::String(trimmed))
}
