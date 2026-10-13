# Ticket B-string-prototype-split: String.prototype.split

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

- `crates/kahvi-builtins/src/string/prototype_split.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`

## Spec

String.prototype.split ( _separator_, _limit_ )
  id: sec-string.prototype.split   type: built-in function

This method returns an Array into which substrings of the result of converting this object to a String have been stored. The substrings are determined by searching from left to right for occurrences of _separator_; these occurrences are not part of any String in the returned array, but serve to divide up the String value. The value of _separator_ may be a String of any length or it may be an object, such as a RegExp, that has a %Symbol.split% method.
It performs the following steps when called:

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. If _separator_ is an Object, then
   a. Let _splitter_ be ? GetMethod(_separator_, %Symbol.split%).
   b. If _splitter_ is not *undefined*, then
      i. Return ? Call(_splitter_, _separator_, « _thisValue_, _limit_ »).
4. Let _string_ be ? ToString(_thisValue_).
5. If _limit_ is *undefined*, let _lim_ be 2^32 - 1; else let _lim_ be ℝ(? ToUint32(_limit_)).
6. Let _separatorString_ be ? ToString(_separator_).
7. If _lim_ = 0, then
   a. Return CreateArrayFromList(« »).
8. If _separator_ is *undefined*, then
   a. Return CreateArrayFromList(« _string_ »).
9. Let _separatorLength_ be the length of _separatorString_.
10. If _separatorLength_ = 0, then
   a. Let _stringLength_ be the length of _string_.
   b. Let _outLength_ be the result of clamping _lim_ between 0 and _stringLength_.
   c. Let _head_ be the substring of _string_ from 0 to _outLength_.
   d. Let _codeUnits_ be a List consisting of the sequence of code units that are the elements of _head_.
   e. Return CreateArrayFromList(_codeUnits_).
11. If _string_ is the empty String, return CreateArrayFromList(« _string_ »).
12. Let _substrings_ be a new empty List.
13. Let _searchStart_ be 0.
14. Let _matchIndex_ be StringIndexOf(_string_, _separatorString_, 0).
15. Repeat, while _matchIndex_ is not ~not-found~,
   a. Let _substring_ be the substring of _string_ from _searchStart_ to _matchIndex_.
   b. Append _substring_ to _substrings_.
   c. If the number of elements in _substrings_ is _lim_, return CreateArrayFromList(_substrings_).
   d. Set _searchStart_ to _matchIndex_ + _separatorLength_.
   e. Set _matchIndex_ to StringIndexOf(_string_, _separatorString_, _searchStart_).
16. Let _substring_ be the substring of _string_ from _searchStart_.
17. Append _substring_ to _substrings_.
18. Return CreateArrayFromList(_substrings_).

NOTE:
The value of _separator_ may be an empty String. In this case, _separator_ does not match the empty substring at the beginning or end of the input String, nor does it match the empty substring at the end of the previous separator match. If _separator_ is the empty String, the String is split up into individual code unit elements; the length of the result array equals the length of the String, and each substring contains one code unit.
If the *this* value is (or converts to) the empty String, the result depends on whether _separator_ can match the empty String. If it can, the result array contains no elements. Otherwise, the result array contains one element, which is the empty String.
If _separator_ is *undefined*, then the result array contains just one String, which is the *this* value (converted to a String). If _limit_ is not *undefined*, then the output array is truncated so that it contains no more than _limit_ elements.
NOTE:
This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.

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

--- referenced: ToUint32 ( _arg_: an ECMAScript language value, ): either a normal completion containing an integral Number or a throw completion
  id: sec-touint32

description: It converts _arg_ to one of 2^32 integral Number values in the inclusive interval from *+0*_𝔽 to 𝔽(2^32 - 1).

1. Let _int_ be ? ToIntegerOrInfinity(_arg_).
2. Return 𝔽(ToFixedSizeInteger(_int_, ~unsigned~, 32)).

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.

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

- Every test262 run under built-ins/String/prototype/split/ passes (check with `./t262 built-ins/String/prototype/split/`). 16 pass on the main branch right now.

