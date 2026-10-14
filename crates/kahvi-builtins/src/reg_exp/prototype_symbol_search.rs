//! RegExp.prototype [ %Symbol.search% ] ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype-%symbol.search%

use kahvi_core::prelude::*;

use crate::support::regexp::regexp_exec;

/// RegExp.prototype[Symbol.search]
pub(crate) fn prototype_symbol_search(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    let Value::Object(regexp) = this else {
        return Err(
            agent.throw_type_error("RegExp.prototype[Symbol.search] called on a non-object")
        );
    };
    // 3. Set _string_ to ? ToString(_string_).
    let string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 4. Let _previousLastIndex_ be ? Get(_regexp_, *"lastIndex"*).
    let key = agent.key("lastIndex");
    let previous_last_index = ops::get(agent, regexp, key)?;
    // 5. If _previousLastIndex_ is not *+0*_𝔽, then
    if !ops::same_value(agent, previous_last_index, Value::Number(0.0)) {
        // a. Perform ? Set(_regexp_, *"lastIndex"*, *+0*_𝔽, *true*).
        let key = agent.key("lastIndex");
        ops::set(agent, regexp, key, Value::Number(0.0), true)?;
    }
    // 6. Let _result_ be ? RegExpExec(_regexp_, _string_).
    let result = regexp_exec(agent, regexp, string)?;
    // 7. Let _currentLastIndex_ be ? Get(_regexp_, *"lastIndex"*).
    let key = agent.key("lastIndex");
    let current_last_index = ops::get(agent, regexp, key)?;
    // 8. If SameValue(_currentLastIndex_, _previousLastIndex_) is *false*, then
    if !ops::same_value(agent, current_last_index, previous_last_index) {
        // a. Perform ? Set(_regexp_, *"lastIndex"*, _previousLastIndex_, *true*).
        let key = agent.key("lastIndex");
        ops::set(agent, regexp, key, previous_last_index, true)?;
    }
    // 9. If _result_ is *null*, return *-1*_𝔽.
    let Value::Object(result) = result else {
        return Ok(Value::Number(-1.0));
    };
    // 10. Return ? Get(_result_, *"index"*).
    let key = agent.key("index");
    ops::get(agent, result, key)
}
