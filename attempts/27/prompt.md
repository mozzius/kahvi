# Ticket B-json-stringify: JSON.stringify

Implement this built-in in its file, following the spec steps already in
the stub: keep each step as a comment directly above the code for it.

- Abstract operations are in `kahvi_core::ops` (see docs/API.md); use them
  rather than re-implementing. If a helper you need is missing and other
  built-ins would need it too, write it in your file and say so in your
  final report; do not edit other files.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing. Never return
  a made-up value.
- No new Rust tests: the test262 target is the measure.

## Files you may change

- `crates/kahvi-builtins/src/json/stringify.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

…lly an object or array, although it can also be a String, Boolean, Number or *null*. The optional _replacer_ parameter is either a function that alters the way objects and arrays are stringified, or an array of Strings and Numbers that acts as an inclusion list for selecting the object properties that will be stringified. The optional _space_ parameter is a String or Number that allows the result to have white space injected into it to improve human readability.
It performs the following steps when called:

1. Let _stack_ be a new empty List.
2. Let _indent_ be the empty String.
3. Let _propertyList_ be *undefined*.
4. Let _replacerFunc_ be *undefined*.
5. If _replacer_ is an Object, then
   a. If IsCallable(_replacer_) is *true*, then
      i. Set _replacerFunc_ to _replacer_.
   b. Else,
      i. Let _isArray_ be ? IsArray(_replacer_).
      ii. If _isArray_ is *true*, then
         1. Set _propertyList_ to a new empty List.
         2. Let _length_ be ? LengthOfArrayLike(_replacer_).
         3. Let _k_ be 0.
         4. Repeat, while _k_ < _length_,
            a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
            b. Let _propertyValue_ be ? Get(_replacer_, _propertyKey_).
            c. Let _item_ be *undefined*.
            d. If _propertyValue_ is a String, then
               i. Set _item_ to _propertyValue_.
            e. Else if _propertyValue_ is a Number, then
               i. Set _item_ to ! ToString(_propertyValue_).
            f. Else if _propertyValue_ is an Object, then
               i. If _propertyValue_ has a [[StringData]] or [[NumberData]] internal slot, set _item_ to ? ToString(_propertyValue_).
            g. If _item_ is not *undefined* and _propertyList_ does not contain _item_, then
               i. Append _item_ to _propertyList_.
            h. Set _k_ to _k_ + 1.
6. If _space_ is an Object, then
   a. If _space_ has a [[NumberData]] internal slot, then
      i. Set _space_ to ? ToNumber(_space_).
   b. Else if _space_ has a [[StringData]] internal slot, then
      i. Set _space_ to ? ToString(_space_).
7. If _space_ is a Number, then
   a. Let _spaceMV_ be ! ToIntegerOrInfinity(_space_).
   b. Set _spaceMV_ to min(10, _spaceMV_).
   c. If _spaceMV_ < 1, let _gap_ be the empty String; else let _gap_ be the String value containing _spaceMV_ occurrences of the code unit 0x0020 (SPACE).
8. Else if _space_ is a String, then
   a. If the length of _space_ ≤ 10, let _gap_ be _space_; else let _gap_ be the substring of _space_ from 0 to 10.
9. Else,
   a. Let _gap_ be the empty String.
10. Let _wrapper_ be OrdinaryObjectCreate(%Object.prototype%).
11. Perform ! CreateDataPropertyOrThrow(_wrapper_, the empty String, _value_).
12. Let _state_ be the JSON Serialization Record { [[ReplacerFunction]]: _replacerFunc_, [[Stack]]: _stack_, [[Indent]]: _indent_, [[Gap]]: _gap_, [[PropertyList]]: _propertyList_ }.
13. Return ? SerializeJSONProperty(_state_, the empty String, _wrapper_).

The *"length"* property of this function is *3*_𝔽.
NOTE:
JSON structures are allowed to be nested to any depth, but they must be acyclic. If _value_ is or contains a cyclic structure, then this function must throw a *TypeError* exception. This is an example of a value that cannot be stringified:
a = [];
a[0] = a;
my_text = JSON.stringify(a); // This must throw a TypeError.
NOTE:
Symbolic primitive values are rendered as follows:
-
The *null* value is rendered in JSON text as the String value *"null"*.
-
The *undefined* value is not rendered.
-
The *true* value is rendered in JSON text as the String value *"true"*.
-
The *false* value is rendered in JSON text as the String value *"false"*.
NOTE:
String values are wrapped in QUOTATION MARK (`"`) code units. The code units `"` and `\\` are escaped with `\\` prefixes. Control characters code units are replaced with escape sequences `\\u`HHHH, or with the shorter forms, `\\b` (BACKSPACE), `\\f` (FORM FEED), `\\n` (LINE FEED), `\\r` (CARRIAGE RETURN), `\\t` (CHARACTER TABULATION).
NOTE:
Each finite Number is stringified as if by calling ToString(_number_). *NaN*, *+∞*_𝔽, and *-∞*_𝔽 are represented as the String value *"null"*.
NOTE:
Values that do not have a JSON representation (such as *undefined* and functions) do not produce a String. Instead they produce the *undefined* value. In arrays these values are represented as the String value *"null"*. In objects an unrepresentable value causes the property to be excluded from stringification.
NOTE:
An object is rendered as U+007B (LEFT CURLY BRACKET) followed by zero or more properties, separated with a U+002C (COMMA), closed with a U+007D (RIGHT CURLY BRACKET). A property is a quoted String representing the property name, a U+003A (COLON), and then the stringified property value. An array is rendered as an opening U+005B (LEFT SQUARE BRACKET) followed by zero or more values, separated with a U+002C (COMMA), closed with a U+005D (RIGHT SQUARE BRACKET).
[subclause sec-json-serialization-record: JSON Serialization Record]
[subclause sec-serializejsonproperty: SerializeJSONProperty (
_state_: a JSON Serialization Record,
_key_: a String,
_holder_: an Object,
): either a normal completion containing either a String or *undefined*, or a throw completion]
[subclause sec-quotejsonstring: QuoteJSONString (
_value_: a String,
): a String]
[subclause sec-unicodeescape: UnicodeEscape (
_codeUnit_: a code unit,
): a String]
[subclause sec-serializejsonobject: SerializeJSONObject (
_state_: a JSON Serialization Record,
_value_: an Object,
): either a normal completion containing a String or a throw completion]
[subclause sec-serializejsonarray: SerializeJSONArray (
_state_: a JSON Serialization Record,
_value_: an Object,
): either a normal completion containing a String or a throw completion]

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

--- referenced: IsArray ( _arg_: an ECMAScript language value, ): either a normal completion containing a Boolean or a throw completion
  id: sec-isarray

1. If _arg_ is not an Object, return *false*.
2. If _arg_ is an Array exotic object, return *true*.
3. If _arg_ is a Proxy exotic object, then
   a. Perform ? ValidateNonRevokedProxy(_arg_).
   b. Let _proxyTarget_ be _arg_.[[ProxyTarget]].
   c. Return ? IsArray(_proxyTarget_).
4. Return *false*.

--- referenced: LengthOfArrayLike ( _obj_: an Object, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-lengthofarraylike

description: It returns the value of the *"length"* property of an array-like object.

1. Return ℝ(? ToLength(? Get(_obj_, *"length"*))).

An array-like object is any object for which this operation returns a normal completion.
NOTE:
Typically, an array-like object would also have some properties with integer index names. However, that is not a requirement of this definition.
NOTE:
Arrays and String objects are examples of array-like objects.

--- referenced: ToString ( _arg_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-tostring

description: It converts _arg_ to a value of type String.

1. If _arg_ is a String, return _arg_.
2. If _arg_ is a Symbol, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *"undefined"*.
4. If _arg_ is *null*, return *"null"*.
5. If _arg_ is *true*, return *"true"*.
6. If _arg_ is *false*, return *"false"*.
7. If _arg_ is a Number, return Number::toString(_arg_, 10).
8. If _arg_ is a BigInt, return BigInt::toString(_arg_, 10).
9. Assert: _arg_ is an Object.
10. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~string~).
11. Assert: _primitiveValue_ is not an Object.
12. Return ? ToString(_primitiveValue_).

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: ToNumber ( _arg_: an ECMAScript language value, ): either a normal completion containing a Number or a throw completion
  id: sec-tonumber

description: It converts _arg_ to a value of type Number.

1. If _arg_ is a Number, return _arg_.
2. If _arg_ is either a Symbol or a BigInt, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *NaN*.
4. If _arg_ is either *null* or *false*, return *+0*_𝔽.
5. If _arg_ is *true*, return *1*_𝔽.
6. If _arg_ is a String, return StringToNumber(_arg_).
7. Assert: _arg_ is an Object.
8. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~number~).
9. Assert: _primitiveValue_ is not an Object.
10. Return ? ToNumber(_primitiveValue_).

[subclause sec-tonumber-applied-to-the-string-type: ToNumber Applied to the String Type]

--- referenced: ToIntegerOrInfinity ( _arg_: an ECMAScript language value, ): either a normal completion containing either an integer, +∞, or -∞, or a throw completion
  id: sec-tointegerorinfinity

description: It converts _arg_ to an integer representing its Number value with fractional part truncated, or to +∞ or -∞ when that Number value is infinite.

1. Let _number_ be ? ToNumber(_arg_).
2. If _number_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return 0.
3. If _number_ is *+∞*_𝔽, return +∞.
4. If _number_ is *-∞*_𝔽, return -∞.
5. Return truncate(ℝ(_number_)).

NOTE:
𝔽(ToIntegerOrInfinity(_x_)) never returns *-0*_𝔽 for any value of _x_. The truncation of the fractional part is performed after converting _x_ to a mathematical value.

--- referenced: OrdinaryObjectCreate ( _proto_: an Object or *null*, optional _additionalInternalSlotsList_: a List of names of internal slots, ): an Object
  id: sec-ordinaryobjectcreate

description: It is used to specify the runtime creation of new ordinary objects. _additionalInternalSlotsList_ contains the names of additional internal slots that must be defined as part of the object, beyond [[Prototype]] and [[Extensible]]. If _additionalInternalSlotsList_ is not provided, a new empty List is used.

1. Let _internalSlotsList_ be « [[Prototype]], [[Extensible]] ».
2. If _additionalInternalSlotsList_ is present, set _internalSlotsList_ to the list-concatenation of _internalSlotsList_ and _additionalInternalSlotsList_.
3. Let _obj_ be MakeBasicObject(_internalSlotsList_).
4. Set _obj_.[[Prototype]] to _proto_.
5. Return _obj_.

NOTE:
Although OrdinaryObjectCreate does little more than call MakeBasicObject, its use communicates the intention to create an ordinary object, and not an exotic one. Thus, within this specification, it is not called by any algorithm that subsequently modifies the internal methods of the object in ways that would make the result non-ordinary. Operations that create exotic objects invoke MakeBasicObject directly.

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.


## Done when

- Every test262 run under built-ins/JSON/stringify/ passes (check with `./t262 built-ins/JSON/stringify/`). 8 pass on the main branch right now.

