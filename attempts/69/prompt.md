# Ticket B-string-prototype-replace: String.prototype.replace

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

- `crates/kahvi-builtins/src/string/prototype_replace.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.replace ( _searchValue_, _replaceValue_ )
  id: sec-string.prototype.replace   type: built-in function

This method performs the following steps when called:

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. If _searchValue_ is an Object, then
   a. Let _replacer_ be ? GetMethod(_searchValue_, %Symbol.replace%).
   b. If _replacer_ is not *undefined*, then
      i. Return ? Call(_replacer_, _searchValue_, « _thisValue_, _replaceValue_ »).
4. Let _string_ be ? ToString(_thisValue_).
5. Let _searchString_ be ? ToString(_searchValue_).
6. Let _functionalReplace_ be IsCallable(_replaceValue_).
7. If _functionalReplace_ is *false*, then
   a. Set _replaceValue_ to ? ToString(_replaceValue_).
8. Let _searchLength_ be the length of _searchString_.
9. Let _position_ be StringIndexOf(_string_, _searchString_, 0).
10. If _position_ is ~not-found~, return _string_.
11. Let _preceding_ be the substring of _string_ from 0 to _position_.
12. Let _following_ be the substring of _string_ from _position_ + _searchLength_.
13. If _functionalReplace_ is *true*, then
   a. Let _replacement_ be ? ToString(? Call(_replaceValue_, *undefined*, « _searchString_, 𝔽(_position_), _string_ »)).
14. Else,
   a. Assert: _replaceValue_ is a String.
   b. Let _captures_ be a new empty List.
   c. Let _replacement_ be ! GetSubstitution(_searchString_, _string_, _position_, _captures_, *undefined*, _replaceValue_).
15. Return the string-concatenation of _preceding_, _replacement_, and _following_.

NOTE:
This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
[subclause sec-getsubstitution: GetSubstitution (
_matched_: a String,
_string_: a String,
_position_: a non-negative integer,
_captures_: a List of either Strings or *undefined*,
_namedCaptures_: an Object or *undefined*,
_replacementTemplate_: a String,
): either a normal completion containing a String or a throw completion]

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

--- referenced: GetMethod ( _value_: an ECMAScript language value, _propertyKey_: a property key, ): either a normal completion containing either a function object or *undefined*, or a throw completion
  id: sec-getmethod

description: It is used to get the value of a specific property of an ECMAScript language value when the value of the property is expected to be a function.

1. Let _func_ be ? GetV(_value_, _propertyKey_).
2. If _func_ is either *undefined* or *null*, return *undefined*.
3. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
4. Return _func_.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).

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

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

--- referenced: StringIndexOf ( _string_: a String, _searchValue_: a String, _fromIndex_: a non-negative integer, ): a non-negative integer or ~not-found~
  id: sec-stringindexof

1. Let _length_ be the length of _string_.
2. If _searchValue_ is the empty String and _fromIndex_ ≤ _length_, return _fromIndex_.
3. Let _searchLength_ be the length of _searchValue_.
4. For each integer _i_ such that _fromIndex_ ≤ _i_ ≤ _length_ - _searchLength_, in ascending order, do
   a. Let _candidate_ be the substring of _string_ from _i_ to _i_ + _searchLength_.
   b. If _candidate_ is _searchValue_, return _i_.
5. Return ~not-found~.

NOTE:
If _searchValue_ is the empty String and _fromIndex_ ≤ the length of _string_, this algorithm returns _fromIndex_. The empty String is effectively found at every position within a string, including after the last code unit.
NOTE:
This algorithm always returns ~not-found~ if _fromIndex_ + the length of _searchValue_ > the length of _string_.


## Done when

- Every test262 run under built-ins/String/prototype/replace/ passes (check with `./t262 built-ins/String/prototype/replace/`). 10 pass on the main branch right now.

