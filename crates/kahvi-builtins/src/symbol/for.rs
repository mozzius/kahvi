//! Symbol.for ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.for

use kahvi_core::prelude::*;

/// Symbol.for
pub(crate) fn r#for(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let key = args.get_or_undefined(0);
    // 1. Let _stringKey_ be ? ToString(_key_).
    let string_key = ops::to_string(agent, key)?;
    // 2. Let _agentRecord_ be the Agent Record of the surrounding agent.
    // 3. Let _globalSymbolRegistry_ be _agentRecord_.[[GlobalSymbolRegistry]].
    // 4. For each element _element_ of _globalSymbolRegistry_, do
    //    a. If _element_.[[Key]] is _stringKey_, return _element_.[[Symbol]].
    let wanted = agent.str(string_key);
    let existing = agent
        .symbol_registry
        .iter()
        .find(|(k, _)| agent.str(*k) == wanted)
        .map(|(_, sym)| *sym);
    if let Some(sym) = existing {
        return Ok(Value::Symbol(sym));
    }
    // 5. Assert: _globalSymbolRegistry_ does not currently contain an entry for _stringKey_.
    // 6. Let _newSymbol_ be a new Symbol whose [[Description]] is _stringKey_.
    let new_symbol = agent.new_symbol(Some(string_key));
    // 7. Append the GlobalSymbolRegistry Record { [[Key]]: _stringKey_, [[Symbol]]: _newSymbol_ } to _globalSymbolRegistry_.
    agent.symbol_registry.push((string_key, new_symbol));
    // 8. Return _newSymbol_.
    Ok(Value::Symbol(new_symbol))
}
