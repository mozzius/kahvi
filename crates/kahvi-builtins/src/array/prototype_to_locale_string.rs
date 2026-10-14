//! Array.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.tolocalestring

use kahvi_core::prelude::*;

/// Array.prototype.toLocaleString
pub(crate) fn prototype_to_locale_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used.
    // NOTE:
    // The first edition of ECMA-402 did not include a replacement specification for this method.
    // The meanings of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.
    // This method performs the following steps when called:
    //
    // 1. Let _array_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_array_).
    // 3. Let _separator_ be the implementation-defined list-separator String value appropriate for the host environment's current locale (such as *", "*).
    // 4. Let _result_ be the empty String.
    // 5. Let _k_ be 0.
    // 6. Repeat, while _k_ < _length_,
    //    a. If _k_ > 0, set _result_ to the string-concatenation of _result_ and _separator_.
    //    b. Let _element_ be ? Get(_array_, ! ToString(𝔽(_k_))).
    //    c. If _element_ is neither *undefined* nor *null*, then
    //       i. Let _elementString_ be ? ToString(? Invoke(_element_, *"toLocaleString"*)).
    //       ii. Set _result_ to the string-concatenation of _result_ and _elementString_.
    //    d. Set _k_ to _k_ + 1.
    // 7. Return _result_.
    //
    // NOTE:
    // This method converts the elements of the array to Strings using their `toLocaleString` methods, and then concatenates these Strings, separated by occurrences of an implementation-defined locale-sensitive separator String. This method is analogous to `toString` except that it is intended to yield a locale-sensitive result corresponding with conventions of the host environment's current locale.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.toLocaleString"))
}
