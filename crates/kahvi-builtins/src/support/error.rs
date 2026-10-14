//! The Error and NativeError constructors (§20.5.1.1, §20.5.6.1.1).

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// InstallErrorCause(O, options) (§20.5.8.1)
pub(crate) fn install_error_cause(agent: &mut Agent, o: Object, options: Value) -> JsResult<()> {
    // 1. If options is an Object and ? HasProperty(options, "cause") is true, then
    if let Value::Object(opts) = options {
        let cause_key = PropertyKey::String(agent.names.cause);
        if ops::has_property(agent, opts, cause_key)? {
            // a. Let cause be ? Get(options, "cause").
            let cause = ops::get(agent, opts, cause_key)?;
            // b. Perform CreateNonEnumerableDataPropertyOrThrow(O, "cause", cause).
            ops::create_non_enumerable_data_property_or_throw(agent, o, cause_key, cause)?;
        }
    }
    // 2. Return unused.
    Ok(())
}

/// The steps shared by Error ( message [ , options ] ) and every
/// NativeError ( message [ , options ] ); `prototype` is the intrinsic
/// default prototype, e.g. "%TypeError.prototype%".
pub(crate) fn construct_error(
    agent: &mut Agent,
    args: &[Value],
    new_target: Option<Object>,
    prototype: &str,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, let newTarget be the active function object; else let newTarget be NewTarget.
    let new_target = match new_target {
        Some(nt) => nt,
        None => agent
            .active_function()
            .expect("a built-in runs with an active function"),
    };
    // 2. Let O be ? OrdinaryCreateFromConstructor(newTarget, "%Error.prototype%", « [[ErrorData]] »).
    let o = ops::ordinary_create_from_constructor(agent, new_target, prototype, ObjectKind::Error)?;
    // 3. If message is not undefined, then
    let message = args.get_or_undefined(0);
    if !message.is_undefined() {
        // a. Let msg be ? ToString(message).
        let msg = ops::to_string(agent, message)?;
        // b. Perform CreateNonEnumerableDataPropertyOrThrow(O, "message", msg).
        let key = PropertyKey::String(agent.names.message);
        ops::create_non_enumerable_data_property_or_throw(agent, o, key, Value::String(msg))?;
    }
    // 4. Perform ? InstallErrorCause(O, options).
    install_error_cause(agent, o, args.get_or_undefined(1))?;
    // 5. Return O.
    Ok(Value::Object(o))
}
