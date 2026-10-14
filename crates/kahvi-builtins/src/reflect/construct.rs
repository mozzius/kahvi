//! Reflect.construct ( _target_, _args_ [ , _newTarget_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.construct

use kahvi_core::prelude::*;

/// Reflect.construct
pub(crate) fn construct(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If IsConstructor(target) is false, throw a TypeError exception.
    let target = match args.get_or_undefined(0) {
        Value::Object(t) if t.is_constructor(agent) => t,
        _ => return Err(agent.throw_type_error("Reflect.construct target is not a constructor")),
    };
    // 2. If newTarget is not present, set newTarget to target.
    // 3. Else if IsConstructor(newTarget) is false, throw a TypeError exception.
    let new_target = match args.get(2) {
        None => target,
        Some(Value::Object(nt)) if nt.is_constructor(agent) => *nt,
        Some(_) => {
            return Err(agent.throw_type_error("Reflect.construct newTarget is not a constructor"));
        }
    };
    // 4. Let args be ? CreateListFromArrayLike(argumentsList).
    let list = ops::create_list_from_array_like(agent, args.get_or_undefined(1), false)?;
    // 5. Return ? Construct(target, args, newTarget). (A tail call: nothing
    //    here is used after it; docs/GC.md.)
    Ok(Value::Object(ops::construct_rooted(
        agent,
        target,
        &list,
        Some(new_target),
    )?))
}
