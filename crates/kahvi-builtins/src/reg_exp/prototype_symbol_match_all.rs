//! RegExp.prototype [ %Symbol.matchAll% ] ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp-prototype-%symbol.matchall%

use kahvi_core::prelude::*;

/// RegExp.prototype[Symbol.matchAll]
pub(crate) fn prototype_symbol_match_all(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    // 3. Set _string_ to ? ToString(_string_).
    // 4. Let _speciesCtor_ be ? SpeciesConstructor(_regexp_, %RegExp%).
    // 5. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
    // 6. Let _matcher_ be ? Construct(_speciesCtor_, « _regexp_, _flags_ »).
    // 7. Let _lastIndex_ be ? ToLength(? Get(_regexp_, *"lastIndex"*)).
    // 8. Perform ? Set(_matcher_, *"lastIndex"*, _lastIndex_, *true*).
    // 9. If _flags_ contains *"g"*, let _global_ be *true*.
    // 10. Else, let _global_ be *false*.
    // 11. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*.
    // 12. Else, let _fullUnicode_ be *false*.
    // 13. Return CreateRegExpStringIterator(_matcher_, _string_, _global_, _fullUnicode_).
    //
    // The value of the *"name"* property of this method is *"[Symbol.matchAll]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype[Symbol.matchAll]"))
}
