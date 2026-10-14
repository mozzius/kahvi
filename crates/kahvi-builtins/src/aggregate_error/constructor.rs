//! AggregateError ( _errors_, _message_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-aggregate-error

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

use crate::support::error::install_error_cause;

/// AggregateError
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
    let new_target = match new_target {
        Some(nt) => nt,
        None => agent
            .active_function()
            .expect("a built-in runs with an active function"),
    };
    // 2. Let _obj_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%AggregateError.prototype%"*, « [[ErrorData]] »).
    let obj = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%AggregateError.prototype%",
        ObjectKind::Error,
    )?;
    // 3. If _message_ is not *undefined*, then
    let message = args.get_or_undefined(1);
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
    // 4. Perform ? InstallErrorCause(_obj_, _options_).
    install_error_cause(agent, obj, args.get_or_undefined(2))?;
    // 5. Let _errorsList_ be ? IteratorToList(? GetIterator(_errors_, ~sync~)).
    let errors = args.get_or_undefined(0);
    let mut record = ops::get_iterator(agent, errors, ops::IteratorKind::Sync)?;
    let errors_list = ops::iterator_to_list(agent, &mut record)?;
    // 6. Perform ! DefinePropertyOrThrow(_obj_, *"errors"*, PropertyDescriptor { [[Configurable]]: *true*, [[Enumerable]]: *false*, [[Writable]]: *true*, [[Value]]: CreateArrayFromList(_errorsList_) }).
    let errors_array = ops::create_array_from_list(agent, &errors_list);
    let key = agent.key("errors");
    ops::define_property_or_throw(
        agent,
        obj,
        key,
        PropertyDescriptor::data(Value::Object(errors_array), true, false, true),
    )?;
    // 7. Return _obj_.
    Ok(Value::Object(obj))
}
