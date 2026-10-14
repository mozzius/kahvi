//! Iterator.prototype.join ( _separator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.join

use kahvi_core::prelude::*;

/// Iterator.prototype.join
pub(crate) fn prototype_join(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method converts the values of the iterator to Strings, and then concatenates these Strings, separated by occurrences of the _separator_. If no separator is provided, a single comma is used as the separator.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    // 3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
    // 4. If _separator_ is *undefined*, then
    //    a. Let _separatorString_ be *","*.
    // 5. Else,
    //    a. Let _separatorString_ be Completion(ToString(_separator_)).
    //    b. IfAbruptCloseIterator(_separatorString_, _iterated_).
    // 6. Set _iterated_ to ? GetIteratorDirect(_obj_).
    // 7. Let _result_ be the empty String.
    // 8. Let _first_ be *true*.
    // 9. Repeat,
    //    a. Let _value_ be ? IteratorStepValue(_iterated_).
    //    b. If _value_ is ~done~, return _result_.
    //    c. If _first_ is *true*, then
    //       i. Set _first_ to *false*.
    //    d. Else,
    //       i. Set _result_ to the string-concatenation of _result_ and _separatorString_.
    //    e. If _value_ is neither *undefined* nor *null*, then
    //       i. Let _valueString_ be Completion(ToString(_value_)).
    //       ii. IfAbruptCloseIterator(_valueString_, _iterated_).
    //       iii. Set _result_ to the string-concatenation of _result_ and _valueString_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.join"))
}
