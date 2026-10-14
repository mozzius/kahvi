//! get RegExp.prototype.flags
//!
//! Spec: https://tc39.es/ecma262/#sec-get-regexp.prototype.flags

use kahvi_core::prelude::*;

/// get RegExp.prototype.flags
pub(crate) fn prototype_get_flags(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // `RegExp.prototype.flags` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    let Value::Object(regexp) = this else {
        return Err(agent.throw_type_error("RegExp.prototype.flags getter called on a non-object"));
    };
    // 3. Let _codeUnits_ be a new empty List.
    let mut code_units: Vec<u16> = Vec::new();
    // 4. Let _hasIndices_ be ToBoolean(? Get(_regexp_, *"hasIndices"*)).
    let key = agent.key("hasIndices");
    let v = ops::get(agent, regexp, key)?;
    let has_indices = ops::to_boolean(agent, v);
    // 5. If _hasIndices_ is *true*, append the code unit 0x0064 (LATIN SMALL LETTER D) to _codeUnits_.
    if has_indices {
        code_units.push(0x0064);
    }
    // 6. Let _global_ be ToBoolean(? Get(_regexp_, *"global"*)).
    let key = agent.key("global");
    let v = ops::get(agent, regexp, key)?;
    let global = ops::to_boolean(agent, v);
    // 7. If _global_ is *true*, append the code unit 0x0067 (LATIN SMALL LETTER G) to _codeUnits_.
    if global {
        code_units.push(0x0067);
    }
    // 8. Let _ignoreCase_ be ToBoolean(? Get(_regexp_, *"ignoreCase"*)).
    let key = agent.key("ignoreCase");
    let v = ops::get(agent, regexp, key)?;
    let ignore_case = ops::to_boolean(agent, v);
    // 9. If _ignoreCase_ is *true*, append the code unit 0x0069 (LATIN SMALL LETTER I) to _codeUnits_.
    if ignore_case {
        code_units.push(0x0069);
    }
    // 10. Let _multiline_ be ToBoolean(? Get(_regexp_, *"multiline"*)).
    let key = agent.key("multiline");
    let v = ops::get(agent, regexp, key)?;
    let multiline = ops::to_boolean(agent, v);
    // 11. If _multiline_ is *true*, append the code unit 0x006D (LATIN SMALL LETTER M) to _codeUnits_.
    if multiline {
        code_units.push(0x006D);
    }
    // 12. Let _dotAll_ be ToBoolean(? Get(_regexp_, *"dotAll"*)).
    let key = agent.key("dotAll");
    let v = ops::get(agent, regexp, key)?;
    let dot_all = ops::to_boolean(agent, v);
    // 13. If _dotAll_ is *true*, append the code unit 0x0073 (LATIN SMALL LETTER S) to _codeUnits_.
    if dot_all {
        code_units.push(0x0073);
    }
    // 14. Let _unicode_ be ToBoolean(? Get(_regexp_, *"unicode"*)).
    let key = agent.key("unicode");
    let v = ops::get(agent, regexp, key)?;
    let unicode = ops::to_boolean(agent, v);
    // 15. If _unicode_ is *true*, append the code unit 0x0075 (LATIN SMALL LETTER U) to _codeUnits_.
    if unicode {
        code_units.push(0x0075);
    }
    // 16. Let _unicodeSets_ be ToBoolean(? Get(_regexp_, *"unicodeSets"*)).
    let key = agent.key("unicodeSets");
    let v = ops::get(agent, regexp, key)?;
    let unicode_sets = ops::to_boolean(agent, v);
    // 17. If _unicodeSets_ is *true*, append the code unit 0x0076 (LATIN SMALL LETTER V) to _codeUnits_.
    if unicode_sets {
        code_units.push(0x0076);
    }
    // 18. Let _sticky_ be ToBoolean(? Get(_regexp_, *"sticky"*)).
    let key = agent.key("sticky");
    let v = ops::get(agent, regexp, key)?;
    let sticky = ops::to_boolean(agent, v);
    // 19. If _sticky_ is *true*, append the code unit 0x0079 (LATIN SMALL LETTER Y) to _codeUnits_.
    if sticky {
        code_units.push(0x0079);
    }
    // 20. Return the String value whose code units are the elements of the List _codeUnits_. If _codeUnits_ has no elements, the empty String is returned.
    Ok(Value::String(agent.intern(&code_units)))
}
