//! %WrapForValidIteratorPrototype%.return ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%wrapforvaliditeratorprototype%.return

use kahvi_core::prelude::*;

use crate::support::iterator_helper::wrapped_iterated;

/// WrapForValidIteratorPrototype.return
pub(crate) fn r#return(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be this value.
    // 2. Perform ? RequireInternalSlot(O, [[Iterated]]).
    // 3. Let iterator be O.[[Iterated]].[[Iterator]].
    let iterator = Value::Object(wrapped_iterated(agent, this)?.iterator);
    // 5. Let returnMethod be ? GetMethod(iterator, "return").
    let key = PropertyKey::String(agent.names.r#return);
    let return_method = ops::get_method(agent, iterator, key)?;
    // 6. If returnMethod is undefined, return CreateIteratorResultObject(undefined, true).
    if return_method.is_undefined() {
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    }
    // 7. Return ? Call(returnMethod, iterator).
    ops::call(agent, return_method, iterator, &[])
}
