# Ticket B-array-prototype-to-locale-string: Array.prototype.toLocaleString

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

- `crates/kahvi-builtins/src/array/prototype_to_locale_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Array.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
  id: sec-array.prototype.tolocalestring   type: built-in function

An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used.
NOTE:
The first edition of ECMA-402 did not include a replacement specification for this method.
The meanings of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.
This method performs the following steps when called:

1. Let _array_ be ? ToObject(*this* value).
2. Let _length_ be ? LengthOfArrayLike(_array_).
3. Let _separator_ be the implementation-defined list-separator String value appropriate for the host environment's current locale (such as *", "*).
4. Let _result_ be the empty String.
5. Let _k_ be 0.
6. Repeat, while _k_ < _length_,
   a. If _k_ > 0, set _result_ to the string-concatenation of _result_ and _separator_.
   b. Let _element_ be ? Get(_array_, ! ToString(𝔽(_k_))).
   c. If _element_ is neither *undefined* nor *null*, then
      i. Let _elementString_ be ? ToString(? Invoke(_element_, *"toLocaleString"*)).
      ii. Set _result_ to the string-concatenation of _result_ and _elementString_.
   d. Set _k_ to _k_ + 1.
7. Return _result_.

NOTE:
This method converts the elements of the array to Strings using their `toLocaleString` methods, and then concatenates these Strings, separated by occurrences of an implementation-defined locale-sensitive separator String. This method is analogous to `toString` except that it is intended to yield a locale-sensitive result corresponding with conventions of the host environment's current locale.
NOTE:
This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.

--- referenced: ToObject ( _arg_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-toobject

description: It converts _arg_ to a value of type Object.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. If _arg_ is a Boolean, return a new Boolean object whose [[BooleanData]] internal slot is set to _arg_. See [sec-boolean-objects] for a description of Boolean objects.
3. If _arg_ is a Number, return a new Number object whose [[NumberData]] internal slot is set to _arg_. See [sec-number-objects] for a description of Number objects.
4. If _arg_ is a String, return a new String object whose [[StringData]] internal slot is set to _arg_. See [sec-string-objects] for a description of String objects.
5. If _arg_ is a Symbol, return a new Symbol object whose [[SymbolData]] internal slot is set to _arg_. See [sec-symbol-objects] for a description of Symbol objects.
6. If _arg_ is a BigInt, return a new BigInt object whose [[BigIntData]] internal slot is set to _arg_. See [sec-bigint-objects] for a description of BigInt objects.
7. Assert: _arg_ is an Object.
8. Return _arg_.

--- referenced: LengthOfArrayLike ( _obj_: an Object, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-lengthofarraylike

description: It returns the value of the *"length"* property of an array-like object.

1. Return ℝ(? ToLength(? Get(_obj_, *"length"*))).

An array-like object is any object for which this operation returns a normal completion.
NOTE:
Typically, an array-like object would also have some properties with integer index names. However, that is not a requirement of this definition.
NOTE:
Arrays and String objects are examples of array-like objects.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

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

--- referenced: Invoke ( _value_: an ECMAScript language value, _propertyKey_: a property key, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-invoke

description: It is used to call a method property of an ECMAScript language value. _value_ serves as both the lookup point for the property and the *this* value of the call. _argList_ is the list of arguments values passed to the method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. Let _func_ be ? GetV(_value_, _propertyKey_).
3. Return ? Call(_func_, _value_, _argList_).


## Done when

- Every test262 run under built-ins/Array/prototype/toLocaleString/ passes (check with `./t262 built-ins/Array/prototype/toLocaleString/`). 8 pass on the main branch right now.

