//! SuppressedError ( _error_, _suppressed_, _message_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-suppressederror

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// SuppressedError
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
    // 2. Let _obj_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%SuppressedError.prototype%"*, « [[ErrorData]] »).
    // 3. If _message_ is not *undefined*, then
    //    a. Let _messageString_ be ? ToString(_message_).
    //    b. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"message"*, _messageString_).
    // 4. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"error"*, _error_).
    // 5. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"suppressed"*, _suppressed_).
    // 6. Return _obj_.
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
    let new_target = match new_target {
        Some(nt) => nt,
        None => agent
            .active_function()
            .expect("a built-in runs with an active function"),
    };
    // 2. Let _obj_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%SuppressedError.prototype%"*, « [[ErrorData]] »).
    let obj = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%SuppressedError.prototype%",
        ObjectKind::Error,
    )?;
    // 3. If _message_ is not *undefined*, then
    let message = args.get_or_undefined(2);
    if !message.is_undefined() {
        // a. Let _messageString_ be ? ToString(_message_).
        let message_string = ops::to_string(agent, message)?;
        // b. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"message"*, _messageString_).
        let key = PropertyKey::String(agent.names.message);
        ops::create_non_enumerable_data_property_or_throw(
            agent,
            obj,
            key,
            Value::String(message_string),
        )?;
    }
    // 4. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"error"*, _error_).
    let error = args.get_or_undefined(0);
    let key = agent.key("error");
    ops::create_non_enumerable_data_property_or_throw(agent, obj, key, error)?;
    // 5. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"suppressed"*, _suppressed_).
    let suppressed = args.get_or_undefined(1);
    let key = agent.key("suppressed");
    ops::create_non_enumerable_data_property_or_throw(agent, obj, key, suppressed)?;
    // 6. Return _obj_.
    Ok(Value::Object(obj))
}
