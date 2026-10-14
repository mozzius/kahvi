//! RegExp.prototype [ %Symbol.search% ] ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype-%symbol.search%

use kahvi_core::prelude::*;

/// RegExp.prototype[Symbol.search]
pub(crate) fn prototype_symbol_search(
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
    // 4. Let _previousLastIndex_ be ? Get(_regexp_, *"lastIndex"*).
    // 5. If _previousLastIndex_ is not *+0*_𝔽, then
    //    a. Perform ? Set(_regexp_, *"lastIndex"*, *+0*_𝔽, *true*).
    // 6. Let _result_ be ? RegExpExec(_regexp_, _string_).
    // 7. Let _currentLastIndex_ be ? Get(_regexp_, *"lastIndex"*).
    // 8. If SameValue(_currentLastIndex_, _previousLastIndex_) is *false*, then
    //    a. Perform ? Set(_regexp_, *"lastIndex"*, _previousLastIndex_, *true*).
    // 9. If _result_ is *null*, return *-1*_𝔽.
    // 10. Return ? Get(_result_, *"index"*).
    //
    // The value of the *"name"* property of this method is *"[Symbol.search]"*.
    // NOTE:
    // The *"lastIndex"* and *"global"* properties of this RegExp object are ignored when performing the search. The *"lastIndex"* property is left unchanged.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype[Symbol.search]"))
}
