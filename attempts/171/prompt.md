# Ticket B-string-prototype-includes: String.prototype.includes

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

- `crates/kahvi-builtins/src/string/prototype_includes.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.includes ( _searchString_ [ , _position_ ] )
  id: sec-string.prototype.includes   type: built-in function

This method performs the following steps when called:

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. Let _string_ be ? ToString(_thisValue_).
4. Let _isRegexp_ be ? IsRegExp(_searchString_).
5. If _isRegexp_ is *true*, throw a *TypeError* exception.
6. Set _searchString_ to ? ToString(_searchString_).
7. Let _length_ be the length of _string_.
8. Let _start_ be the result of clamping ? ToIntegerOrInfinity(_position_) between 0 and _length_.
9. Assert: If _position_ is *undefined*, then _start_ is 0.
10. Let _index_ be StringIndexOf(_string_, _searchString_, _start_).
11. If _index_ is ~not-found~, return *false*.
12. Return *true*.

NOTE:
If _searchString_ appears as a substring of the result of converting this object to a String, at one or more indices that are greater than or equal to _position_, this function returns *true*; otherwise, it returns *false*. If _position_ is *undefined*, 0 is assumed, so as to search all of the String.
NOTE:
Throwing an exception if the first argument is a RegExp is specified in order to allow future editions to define extensions that allow such argument values.
NOTE:
This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

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

--- referenced: IsRegExp ( _arg_: an ECMAScript language value, ): either a normal completion containing a Boolean or a throw completion
  id: sec-isregexp

1. If _arg_ is not an Object, return *false*.
2. Let _matcher_ be ? Get(_arg_, %Symbol.match%).
3. If _matcher_ is not *undefined*, return ToBoolean(_matcher_).
4. If _arg_ has a [[RegExpMatcher]] internal slot, return *true*.
5. Return *false*.

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

- Every test262 run under built-ins/String/prototype/includes/ passes (check with `./t262 built-ins/String/prototype/includes/`). 10 pass on the main branch right now.

