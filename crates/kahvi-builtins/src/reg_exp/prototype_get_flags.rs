//! get RegExp.prototype.flags
//!
//! Spec: https://tc39.es/ecma262/#sec-get-regexp.prototype.flags

use kahvi_core::prelude::*;

/// get RegExp.prototype.flags
pub(crate) fn prototype_get_flags(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `RegExp.prototype.flags` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    // 3. Let _codeUnits_ be a new empty List.
    // 4. Let _hasIndices_ be ToBoolean(? Get(_regexp_, *"hasIndices"*)).
    // 5. If _hasIndices_ is *true*, append the code unit 0x0064 (LATIN SMALL LETTER D) to _codeUnits_.
    // 6. Let _global_ be ToBoolean(? Get(_regexp_, *"global"*)).
    // 7. If _global_ is *true*, append the code unit 0x0067 (LATIN SMALL LETTER G) to _codeUnits_.
    // 8. Let _ignoreCase_ be ToBoolean(? Get(_regexp_, *"ignoreCase"*)).
    // 9. If _ignoreCase_ is *true*, append the code unit 0x0069 (LATIN SMALL LETTER I) to _codeUnits_.
    // 10. Let _multiline_ be ToBoolean(? Get(_regexp_, *"multiline"*)).
    // 11. If _multiline_ is *true*, append the code unit 0x006D (LATIN SMALL LETTER M) to _codeUnits_.
    // 12. Let _dotAll_ be ToBoolean(? Get(_regexp_, *"dotAll"*)).
    // 13. If _dotAll_ is *true*, append the code unit 0x0073 (LATIN SMALL LETTER S) to _codeUnits_.
    // 14. Let _unicode_ be ToBoolean(? Get(_regexp_, *"unicode"*)).
    // 15. If _unicode_ is *true*, append the code unit 0x0075 (LATIN SMALL LETTER U) to _codeUnits_.
    // 16. Let _unicodeSets_ be ToBoolean(? Get(_regexp_, *"unicodeSets"*)).
    // 17. If _unicodeSets_ is *true*, append the code unit 0x0076 (LATIN SMALL LETTER V) to _codeUnits_.
    // 18. Let _sticky_ be ToBoolean(? Get(_regexp_, *"sticky"*)).
    // 19. If _sticky_ is *true*, append the code unit 0x0079 (LATIN SMALL LETTER Y) to _codeUnits_.
    // 20. Return the String value whose code units are the elements of the List _codeUnits_. If _codeUnits_ has no elements, the empty String is returned.
    //
    // [subclause sec-regexphasflag: RegExpHasFlag (
    // _regexp_: an ECMAScript language value,
    // _codeUnit_: a code unit,
    // ): either a normal completion containing either a Boolean or *undefined*, or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get RegExp.prototype.flags"))
}
