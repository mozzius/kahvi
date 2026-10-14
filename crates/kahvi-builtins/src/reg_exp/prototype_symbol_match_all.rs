//! RegExp.prototype [ %Symbol.matchAll% ] ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp-prototype-%symbol.matchall%

use kahvi_core::prelude::*;

/// RegExp.prototype[Symbol.matchAll]
pub(crate) fn prototype_symbol_match_all(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _regexp_ be the *this* value.
    let regexp = this;
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    let Value::Object(regexp) = regexp else {
        return Err(
            agent.throw_type_error("RegExp.prototype[Symbol.matchAll] called on a non-object")
        );
    };
    // 3. Set _string_ to ? ToString(_string_).
    let string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 4. Let _speciesCtor_ be ? SpeciesConstructor(_regexp_, %RegExp%).
    let default_ctor = agent.intrinsic("%RegExp%");
    let species_ctor = ops::species_constructor(agent, regexp, default_ctor)?;
    // 5. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
    let key = agent.key("flags");
    let flags_value = ops::get(agent, regexp, key)?;
    let flags = ops::to_string(agent, flags_value)?;
    // 6. Let _matcher_ be ? Construct(_speciesCtor_, « _regexp_, _flags_ »).
    let matcher = ops::construct(
        agent,
        species_ctor,
        &[Value::Object(regexp), Value::String(flags)],
        None,
    )?;
    // 7. Let _lastIndex_ be ? ToLength(? Get(_regexp_, *"lastIndex"*)).
    let key = agent.key("lastIndex");
    let last_index_value = ops::get(agent, regexp, key)?;
    let last_index = ops::to_length(agent, last_index_value)?;
    // 8. Perform ? Set(_matcher_, *"lastIndex"*, _lastIndex_, *true*).
    let key = agent.key("lastIndex");
    ops::set(agent, matcher, key, Value::Number(last_index), true)?;
    let flags_text = agent.str_lossy(flags).to_string();
    // 9. If _flags_ contains *"g"*, let _global_ be *true*.
    // 10. Else, let _global_ be *false*.
    let global = flags_text.contains('g');
    // 11. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*.
    // 12. Else, let _fullUnicode_ be *false*.
    let full_unicode = flags_text.contains('u') || flags_text.contains('v');
    // 13. Return CreateRegExpStringIterator(_matcher_, _string_, _global_, _fullUnicode_).
    //     CreateRegExpStringIterator needs the [[IteratingRegExp]], [[IteratedString]],
    //     [[Global]], [[Unicode]] and [[Done]] slots on an object whose prototype is
    //     %RegExpStringIteratorPrototype%; no slot type for them exists yet.
    let _ = (matcher, string, global, full_unicode);
    Err(agent.not_implemented("CreateRegExpStringIterator (RegExp String Iterator slots)"))
}
