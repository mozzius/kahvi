//! Array.prototype.join ( _separator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.join

use kahvi_core::prelude::*;

/// Array.prototype.join
pub(crate) fn prototype_join(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method converts the elements of the array to Strings, and then concatenates these Strings, separated by occurrences of the _separator_. If no separator is provided, a single comma is used as the separator.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If _separator_ is *undefined*, let _separatorString_ be *","*.
    // 4. Else, let _separatorString_ be ? ToString(_separator_).
    // 5. Let _result_ be the empty String.
    // 6. Let _k_ be 0.
    // 7. Repeat, while _k_ < _length_,
    //    a. If _k_ > 0, set _result_ to the string-concatenation of _result_ and _separatorString_.
    //    b. Let _element_ be ? Get(_obj_, ! ToString(𝔽(_k_))).
    //    c. If _element_ is neither *undefined* nor *null*, then
    //       i. Let _elementString_ be ? ToString(_element_).
    //       ii. Set _result_ to the string-concatenation of _result_ and _elementString_.
    //    d. Set _k_ to _k_ + 1.
    // 8. Return _result_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.join"))
}
