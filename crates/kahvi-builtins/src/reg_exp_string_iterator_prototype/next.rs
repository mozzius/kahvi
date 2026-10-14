//! %RegExpStringIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%regexpstringiteratorprototype%.next

use kahvi_core::prelude::*;

use crate::support::regexp::{advance_string_index, regexp_exec};
use crate::support::regexp_string_iterator::RegExpStringIterator;

/// RegExpStringIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (args, new_target);
    // 1. Let _iteratorObj_ be the *this* value.
    let iterator_obj = this;
    // 2. If _iteratorObj_ is not an Object, throw a *TypeError* exception.
    let Value::Object(iterator_obj) = iterator_obj else {
        return Err(agent.throw_type_error("not a RegExp String Iterator"));
    };
    // 3. If _iteratorObj_ does not have all of the internal slots of a RegExp String Iterator Object Instance (see [sec-properties-of-regexp-string-iterator-instances]), throw a *TypeError* exception.
    let Some(slots) = agent.native::<RegExpStringIterator>(iterator_obj) else {
        return Err(agent.throw_type_error("not a RegExp String Iterator"));
    };
    // 4. If _iteratorObj_.[[Done]] is *true*, then
    if slots.done {
        //    a. Return CreateIteratorResultObject(*undefined*, *true*).
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    }
    // 5. Let _regexp_ be _iteratorObj_.[[IteratingRegExp]].
    let regexp = slots.iterating_regexp;
    // 6. Let _string_ be _iteratorObj_.[[IteratedString]].
    let string = slots.iterated_string;
    // 7. Let _global_ be _iteratorObj_.[[Global]].
    let global = slots.global;
    // 8. Let _fullUnicode_ be _iteratorObj_.[[Unicode]].
    let full_unicode = slots.unicode;
    // 9. Let _match_ be ? RegExpExec(_regexp_, _string_).
    let r#match = regexp_exec(agent, regexp, string)?;
    // 10. If _match_ is *null*, then
    let Value::Object(match_obj) = r#match else {
        //    a. Set _iteratorObj_.[[Done]] to *true*.
        set_done(agent, iterator_obj);
        //    b. Return CreateIteratorResultObject(*undefined*, *true*).
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    };
    // 11. If _global_ is *false*, then
    if !global {
        //    a. Set _iteratorObj_.[[Done]] to *true*.
        set_done(agent, iterator_obj);
        //    b. Return CreateIteratorResultObject(_match_, *false*).
        return Ok(Value::Object(ops::create_iter_result_object(
            agent, r#match, false,
        )));
    }
    // The match object stays rooted while the steps below may collect garbage.
    agent.with_roots(&[r#match], |agent| {
        // 12. Let _matchString_ be ? ToString(? Get(_match_, *"0"*)).
        let key = agent.key("0");
        let match_value = ops::get(agent, match_obj, key)?;
        let match_string = ops::to_string(agent, match_value)?;
        // 13. If _matchString_ is the empty String, then
        if agent.str_len(match_string) == 0 {
            //    a. Let _thisIndex_ be ℝ(? ToLength(? Get(_regexp_, *"lastIndex"*))).
            let key = agent.key("lastIndex");
            let last_index = ops::get(agent, regexp, key)?;
            let this_index = ops::to_length(agent, last_index)?;
            //    b. Let _nextIndex_ be AdvanceStringIndex(_string_, _thisIndex_, _fullUnicode_).
            let next_index = advance_string_index(agent.str(string), this_index, full_unicode);
            //    c. Perform ? Set(_regexp_, *"lastIndex"*, 𝔽(_nextIndex_), *true*).
            let key = agent.key("lastIndex");
            ops::set(agent, regexp, key, Value::Number(next_index), true)?;
        }
        // 14. Return CreateIteratorResultObject(_match_, *false*).
        Ok(Value::Object(ops::create_iter_result_object(
            agent, r#match, false,
        )))
    })
}

/// Sets _iteratorObj_.[[Done]] to *true*.
fn set_done(agent: &mut Agent, iterator_obj: Object) {
    agent
        .native_mut::<RegExpStringIterator>(iterator_obj)
        .expect("a RegExp String Iterator")
        .done = true;
}
