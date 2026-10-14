//! Object.getOwnPropertySymbols ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getownpropertysymbols

use kahvi_core::prelude::*;

/// Object.getOwnPropertySymbols
pub(crate) fn get_own_property_symbols(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Return CreateArrayFromList(? GetOwnPropertyKeys(_obj_, ~symbol~)).
    //
    // [subclause sec-getownpropertykeys: GetOwnPropertyKeys (
    // _value_: an ECMAScript language value,
    // _type_: ~string~ or ~symbol~,
    // ): either a normal completion containing a List of property keys or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.getOwnPropertySymbols"))
}
