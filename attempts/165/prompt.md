# Ticket B-string-prototype-ends-with: String.prototype.endsWith

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

- `crates/kahvi-builtins/src/string/prototype_ends_with.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.endsWith ( _searchString_ [ , _endPosition_ ] )
  id: sec-string.prototype.endswith   type: built-in function

This method performs the following steps when called:

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. Let _string_ be ? ToString(_thisValue_).
4. Let _isRegexp_ be ? IsRegExp(_searchString_).
5. If _isRegexp_ is *true*, throw a *TypeError* exception.
6. Set _searchString_ to ? ToString(_searchString_).
7. Let _length_ be the length of _string_.
8. If _endPosition_ is *undefined*, let _end_ be _length_; else let _end_ be the result of clamping ? ToIntegerOrInfinity(_endPosition_) between 0 and _length_.
9. Let _searchLength_ be the length of _searchString_.
10. If _searchLength_ = 0, return *true*.
11. Let _start_ be _end_ - _searchLength_.
12. If _start_ < 0, return *false*.
13. Let _substring_ be the substring of _string_ from _start_ to _end_.
14. If _substring_ is _searchString_, return *true*.
15. Return *false*.

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


## Done when

- Every test262 run under built-ins/String/prototype/endsWith/ passes (check with `./t262 built-ins/String/prototype/endsWith/`). 8 pass on the main branch right now.

