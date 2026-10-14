//! Function.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-function.prototype.tostring

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// Function.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let func be the this value.
    let Value::Object(func) = this else {
        return Err(agent
            .throw_type_error("Function.prototype.toString requires that 'this' be a Function"));
    };
    let text = match &agent.obj(func).kind {
        // 2. If func is an Object, func has a [[SourceText]] internal slot, ... return CodePointsToString(func.[[SourceText]]).
        ObjectKind::Function(e) => match &e.source_override {
            Some((source, start, end)) => source
                .get(*start as usize..*end as usize)
                .unwrap_or("")
                .to_string(),
            None => e.code.source_text().to_string(),
        },
        // 3. If func is a built-in function object, return an implementation-defined
        //    String source code representation of func matching NativeFunction.
        ObjectKind::Builtin(b) => {
            let name = b
                .initial_name
                .map(|n| agent.str_lossy(n))
                .unwrap_or_default();
            format!("function {name}() {{ [native code] }}")
        }
        // 4. If func is an Object and IsCallable(func) is true, return a NativeFunction representation.
        _ if func.is_callable(agent) => "function () { [native code] }".to_string(),
        // 5. Throw a TypeError exception.
        _ => {
            return Err(agent.throw_type_error(
                "Function.prototype.toString requires that 'this' be a Function",
            ));
        }
    };
    Ok(Value::String(agent.string(&text)))
}
