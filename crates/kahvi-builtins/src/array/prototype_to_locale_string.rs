//! Array.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.tolocalestring

use kahvi_core::prelude::*;

/// Array.prototype.toLocaleString
pub(crate) fn prototype_to_locale_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
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
    // 1. Let _array_ be ? ToObject(*this* value).
    let array = ops::to_object(agent, this)?;
    // 2. Let _length_ be ? LengthOfArrayLike(_array_).
    let length = ops::length_of_array_like(agent, array)?;
    // 3. Let _separator_ be the implementation-defined list-separator String value appropriate for the host environment's current locale (such as *", "*).
    let separator: Vec<u16> = vec![u16::from(b',')];
    // 4. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::new();
    // 5. Let _k_ be 0.
    let mut k = 0.0;
    // The calls to toLocaleString may collect garbage: _array_ stays rooted.
    agent.with_roots(&[Value::Object(array)], |agent| {
        // 6. Repeat, while _k_ < _length_,
        while k < length {
            // a. If _k_ > 0, set _result_ to the string-concatenation of _result_ and _separator_.
            if k > 0.0 {
                result.extend_from_slice(&separator);
            }
            // b. Let _element_ be ? Get(_array_, ! ToString(𝔽(_k_))).
            let key = agent.key_from_index(k);
            let element = ops::get(agent, array, key)?;
            // c. If _element_ is neither *undefined* nor *null*, then
            if !element.is_nullish() {
                // i. Let _elementString_ be ? ToString(? Invoke(_element_, *"toLocaleString"*)).
                let mark = agent.push_root(element);
                let to_locale_string = agent.key("toLocaleString");
                let invoked = ops::invoke(agent, element, to_locale_string, &[])?;
                let element_string = ops::to_string(agent, invoked)?;
                agent.pop_roots(mark);
                // ii. Set _result_ to the string-concatenation of _result_ and _elementString_.
                result.extend_from_slice(agent.str(element_string));
            }
            // d. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // 7. Return _result_.
        Ok(Value::String(agent.intern(&result)))
    })
}
