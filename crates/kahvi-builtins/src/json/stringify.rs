//! JSON.stringify ( _value_ [ , _replacer_ [ , _space_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-json.stringify

use kahvi_core::prelude::*;

/// JSON.stringify
pub(crate) fn stringify(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns a String in UTF-16 encoded JSON format representing an ECMAScript language value, or *undefined*. It can take three parameters. The _value_ parameter is an ECMAScript language value, which is usually an object or array, although it can also be a String, Boolean, Number or *null*. The optional _replacer_ parameter is either a function that alters the way objects and arrays are stringified, or an array of Strings and Numbers that acts as an inclusion list for selecting the object properties that will be stringified. The optional _space_ parameter is a String or Number that allows the result to have white space injected into it to improve human readability.
    // It performs the following steps when called:
    //
    // 1. Let _stack_ be a new empty List.
    // 2. Let _indent_ be the empty String.
    // 3. Let _propertyList_ be *undefined*.
    // 4. Let _replacerFunc_ be *undefined*.
    // 5. If _replacer_ is an Object, then
    //    a. If IsCallable(_replacer_) is *true*, then
    //       i. Set _replacerFunc_ to _replacer_.
    //    b. Else,
    //       i. Let _isArray_ be ? IsArray(_replacer_).
    //       ii. If _isArray_ is *true*, then
    //          1. Set _propertyList_ to a new empty List.
    //          2. Let _length_ be ? LengthOfArrayLike(_replacer_).
    //          3. Let _k_ be 0.
    //          4. Repeat, while _k_ < _length_,
    //             a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //             b. Let _propertyValue_ be ? Get(_replacer_, _propertyKey_).
    //             c. Let _item_ be *undefined*.
    //             d. If _propertyValue_ is a String, then
    //                i. Set _item_ to _propertyValue_.
    //             e. Else if _propertyValue_ is a Number, then
    //                i. Set _item_ to ! ToString(_propertyValue_).
    //             f. Else if _propertyValue_ is an Object, then
    //                i. If _propertyValue_ has a [[StringData]] or [[NumberData]] internal slot, set _item_ to ? ToString(_propertyValue_).
    //             g. If _item_ is not *undefined* and _propertyList_ does not contain _item_, then
    //                i. Append _item_ to _propertyList_.
    //             h. Set _k_ to _k_ + 1.
    // 6. If _space_ is an Object, then
    //    a. If _space_ has a [[NumberData]] internal slot, then
    //       i. Set _space_ to ? ToNumber(_space_).
    //    b. Else if _space_ has a [[StringData]] internal slot, then
    //       i. Set _space_ to ? ToString(_space_).
    // 7. If _space_ is a Number, then
    //    a. Let _spaceMV_ be ! ToIntegerOrInfinity(_space_).
    //    b. Set _spaceMV_ to min(10, _spaceMV_).
    //    c. If _spaceMV_ < 1, let _gap_ be the empty String; else let _gap_ be the String value containing _spaceMV_ occurrences of the code unit 0x0020 (SPACE).
    // 8. Else if _space_ is a String, then
    //    a. If the length of _space_ ≤ 10, let _gap_ be _space_; else let _gap_ be the substring of _space_ from 0 to 10.
    // 9. Else,
    //    a. Let _gap_ be the empty String.
    // 10. Let _wrapper_ be OrdinaryObjectCreate(%Object.prototype%).
    // 11. Perform ! CreateDataPropertyOrThrow(_wrapper_, the empty String, _value_).
    // 12. Let _state_ be the JSON Serialization Record { [[ReplacerFunction]]: _replacerFunc_, [[Stack]]: _stack_, [[Indent]]: _indent_, [[Gap]]: _gap_, [[PropertyList]]: _propertyList_ }.
    // 13. Return ? SerializeJSONProperty(_state_, the empty String, _wrapper_).
    //
    // The *"length"* property of this function is *3*_𝔽.
    // NOTE:
    // JSON structures are allowed to be nested to any depth, but they must be acyclic. If _value_ is or contains a cyclic structure, then this function must throw a *TypeError* exception. This is an example of a value that cannot be stringified:
    // a = [];
    // a[0] = a;
    // my_text = JSON.stringify(a); // This must throw a TypeError.
    // NOTE:
    // Symbolic primitive values are rendered as follows:
    // -
    // The *null* value is rendered in JSON text as the String value *"null"*.
    // -
    // The *undefined* value is not rendered.
    // -
    // The *true* value is rendered in JSON text as the String value *"true"*.
    // -
    // The *false* value is rendered in JSON text as the String value *"false"*.
    // NOTE:
    // String values are wrapped in QUOTATION MARK (`"`) code units. The code units `"` and `\\` are escaped with `\\` prefixes. Control characters code units are replaced with escape sequences `\\u`HHHH, or with the shorter forms, `\\b` (BACKSPACE), `\\f` (FORM FEED), `\\n` (LINE FEED), `\\r` (CARRIAGE RETURN), `\\t` (CHARACTER TABULATION).
    // NOTE:
    // Each finite Number is stringified as if by calling ToString(_number_). *NaN*, *+∞*_𝔽, and *-∞*_𝔽 are represented as the String value *"null"*.
    // NOTE:
    // Values that do not have a JSON representation (such as *undefined* and functions) do not produce a String. Instead they produce the *undefined* value. In arrays these values are represented as the String value *"null"*. In objects an unrepresentable value causes the property to be excluded from stringification.
    // NOTE:
    // An object is rendered as U+007B (LEFT CURLY BRACKET) followed by zero or more properties, separated with a U+002C (COMMA), closed with a U+007D (RIGHT CURLY BRACKET). A property is a quoted String representing the property name, a U+003A (COLON), and then the stringified property value. An array is rendered as an opening U+005B (LEFT SQUARE BRACKET) followed by zero or more values, separated with a U+002C (COMMA), closed with a U+005D (RIGHT SQUARE BRACKET).
    // [subclause sec-json-serialization-record: JSON Serialization Record]
    // [subclause sec-serializejsonproperty: SerializeJSONProperty (
    // _state_: a JSON Serialization Record,
    // _key_: a String,
    // _holder_: an Object,
    // ): either a normal completion containing either a String or *undefined*, or a throw completion]
    // [subclause sec-quotejsonstring: QuoteJSONString (
    // _value_: a String,
    // ): a String]
    // [subclause sec-unicodeescape: UnicodeEscape (
    // _codeUnit_: a code unit,
    // ): a String]
    // [subclause sec-serializejsonobject: SerializeJSONObject (
    // _state_: a JSON Serialization Record,
    // _value_: an Object,
    // ): either a normal completion containing a String or a throw completion]
    // [subclause sec-serializejsonarray: SerializeJSONArray (
    // _state_: a JSON Serialization Record,
    // _value_: an Object,
    // ): either a normal completion containing a String or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("JSON.stringify"))
}
