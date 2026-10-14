//! RegExp.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype.tostring

use kahvi_core::prelude::*;

/// RegExp.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _regexp_ be the *this* value.
    let regexp = this;
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    let Value::Object(regexp) = regexp else {
        return Err(agent.throw_type_error("RegExp.prototype.toString called on a non-object"));
    };
    // The Gets below may run user code and collect garbage: _regexp_ and the
    // values read so far stay rooted until the result is built.
    agent.with_roots(&[Value::Object(regexp)], |agent| {
        // 3. Let _pattern_ be ? ToString(? Get(_regexp_, *"source"*)).
        let source_key = agent.key("source");
        let source = ops::get(agent, regexp, source_key)?;
        let mark = agent.push_root(source);
        let pattern = ops::to_string(agent, source)?;
        agent.push_root(Value::String(pattern));
        // 4. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
        let flags_key = agent.key("flags");
        let flags_value = ops::get(agent, regexp, flags_key)?;
        agent.push_root(flags_value);
        let flags = ops::to_string(agent, flags_value)?;
        agent.pop_roots(mark);
        // 5. Let _result_ be the string-concatenation of *"/"*, _pattern_, *"/"*, and _flags_.
        let slash = agent.string("/");
        let with_pattern = agent.concat(slash, pattern);
        let with_slash = agent.concat(with_pattern, slash);
        let result = agent.concat(with_slash, flags);
        // 6. Return _result_.
        Ok(Value::String(result))
    })
}
