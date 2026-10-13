//! AggregateError ( _errors_, _message_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-aggregate-error

use kahvi_core::prelude::*;

/// AggregateError
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
    // 2. Let _obj_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%AggregateError.prototype%"*, « [[ErrorData]] »).
    // 3. If _message_ is not *undefined*, then
    //    a. Let _messageString_ be ? ToString(_message_).
    //    b. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"message"*, _messageString_).
    // 4. Perform ? InstallErrorCause(_obj_, _options_).
    // 5. Let _errorsList_ be ? IteratorToList(? GetIterator(_errors_, ~sync~)).
    // 6. Perform ! DefinePropertyOrThrow(_obj_, *"errors"*, PropertyDescriptor { [[Configurable]]: *true*, [[Enumerable]]: *false*, [[Writable]]: *true*, [[Value]]: CreateArrayFromList(_errorsList_) }).
    // 7. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AggregateError"))
}
