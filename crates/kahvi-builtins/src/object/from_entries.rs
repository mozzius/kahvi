//! Object.fromEntries ( _iterable_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.fromentries

use crate::support::collections::add_entries_from_iterable;
use kahvi_core::function::{builtin_slots, create_builtin_function};
use kahvi_core::prelude::*;

/// Object.fromEntries
pub(crate) fn from_entries(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Perform ? RequireObjectCoercible(_iterable_).
    let iterable = args.get_or_undefined(0);
    ops::require_object_coercible(agent, iterable)?;
    // 2. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let obj = agent.ordinary_object_create(Some(proto));
    // 3. Assert: _obj_ is an extensible ordinary object with no own properties.
    // 4. Let _closure_ be a new Abstract Closure with parameters (_key_, _value_) that captures _obj_ and performs the following steps when called:
    //    a. Let _propertyKey_ be ? ToPropertyKey(_key_).
    //    b. Perform ! CreateDataPropertyOrThrow(_obj_, _propertyKey_, _value_).
    //    c. Return NormalCompletion(*undefined*).
    // (The closure is the native function `from_entries_adder`, which reads
    // _obj_ from its [[Slots]].)
    // 5. Let _adder_ be CreateBuiltinFunction(_closure_, 2, *""*, « »).
    let empty = PropertyKey::String(agent.names.empty);
    let adder = create_builtin_function(
        agent,
        from_entries_adder,
        2,
        empty,
        vec![Value::Object(obj)],
        None,
        None,
        None,
        false,
    );
    // 6. Return ? AddEntriesFromIterable(_obj_, _iterable_, _adder_).
    add_entries_from_iterable(agent, obj, iterable, Value::Object(adder))
}

/// The closure created for _adder_ in Object.fromEntries (§20.1.2.7 step 4).
fn from_entries_adder(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The active function's [[Slots]] hold _obj_ as the captured value.
    let f = agent
        .active_function()
        .expect("a built-in runs with an active function");
    let Some(Value::Object(obj)) = builtin_slots(agent, f).first().copied() else {
        unreachable!("Object.fromEntries adder is created with _obj_ in its slots");
    };
    // a. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(0))?;
    // b. Perform ! CreateDataPropertyOrThrow(_obj_, _propertyKey_, _value_).
    ops::create_data_property_or_throw(agent, obj, property_key, args.get_or_undefined(1))?;
    // c. Return NormalCompletion(*undefined*).
    Ok(Value::Undefined)
}
