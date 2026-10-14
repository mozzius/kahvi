//! Object.getOwnPropertySymbols ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getownpropertysymbols

use kahvi_core::prelude::*;

/// Object.getOwnPropertySymbols
pub(crate) fn get_own_property_symbols(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return CreateArrayFromList(? GetOwnPropertyKeys(_obj_, ~symbol~)).
    // GetOwnPropertyKeys ( _value_, _type_ ):
    // 1. Let _obj_ be ? ToObject(_value_).
    let obj = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Let _keys_ be ? _obj_.[[OwnPropertyKeys]]().
    let keys = obj.internal_own_property_keys(agent)?;
    // 3. Let _nameList_ be a new empty List.
    // 4. For each element _nextKey_ of _keys_, do
    //    a. If _nextKey_ is a Symbol and _type_ is ~symbol~, or if _nextKey_ is a String
    //       and _type_ is ~string~, then
    //       i. Append _nextKey_ to _nameList_.
    let symbols: Vec<Value> = keys
        .into_iter()
        .filter(|k| k.is_symbol())
        .map(|k| agent.key_to_value(k))
        .collect();
    // 5. Return _nameList_.
    Ok(Value::Object(ops::create_array_from_list(agent, &symbols)))
}
