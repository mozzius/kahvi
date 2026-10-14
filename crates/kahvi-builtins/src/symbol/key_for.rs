//! Symbol.keyFor ( _symbol_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.keyfor

use kahvi_core::prelude::*;

/// Symbol.keyFor
pub(crate) fn key_for(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let symbol = args.get_or_undefined(0);
    // 1. If _symbol_ is not a Symbol, throw a *TypeError* exception.
    let Value::Symbol(sym) = symbol else {
        return Err(agent.throw_type_error("Symbol.keyFor: argument is not a symbol"));
    };
    // 2. Return KeyForSymbol(_symbol_).
    //
    // KeyForSymbol ( _symbol_ )
    //
    // 1. Let _agentRecord_ be the Agent Record of the surrounding agent.
    // 2. Let _globalSymbolRegistry_ be _agentRecord_.[[GlobalSymbolRegistry]].
    // 3. For each element _element_ of _globalSymbolRegistry_, do
    //    a. If SameValue(_element_.[[Symbol]], _symbol_) is *true*, return _element_.[[Key]].
    let key = agent
        .symbol_registry
        .iter()
        .find(|(_, registered)| *registered == sym)
        .map(|(key, _)| *key);
    // 4. Assert: _globalSymbolRegistry_ does not currently contain an entry for _symbol_.
    // 5. Return *undefined*.
    Ok(match key {
        Some(key) => Value::String(key),
        None => Value::Undefined,
    })
}
