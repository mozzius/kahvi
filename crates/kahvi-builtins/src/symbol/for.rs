//! Symbol.for ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.for

use kahvi_core::prelude::*;

/// Symbol.for
pub(crate) fn r#for(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _stringKey_ be ? ToString(_key_).
    // 2. Let _agentRecord_ be the Agent Record of the surrounding agent.
    // 3. Let _globalSymbolRegistry_ be _agentRecord_.[[GlobalSymbolRegistry]].
    // 4. For each element _element_ of _globalSymbolRegistry_, do
    //    a. If _element_.[[Key]] is _stringKey_, return _element_.[[Symbol]].
    // 5. Assert: _globalSymbolRegistry_ does not currently contain an entry for _stringKey_.
    // 6. Let _newSymbol_ be a new Symbol whose [[Description]] is _stringKey_.
    // 7. Append the GlobalSymbolRegistry Record { [[Key]]: _stringKey_, [[Symbol]]: _newSymbol_ } to _globalSymbolRegistry_.
    // 8. Return _newSymbol_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Symbol.for"))
}
