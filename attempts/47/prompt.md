# Ticket B-reg-exp-prototype-symbol-replace: RegExp.prototype[Symbol.replace]

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

- `crates/kahvi-builtins/src/reg_exp/prototype_symbol_replace.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

RegExp.prototype [ %Symbol.replace% ] ( _string_, _replaceValue_ )
  id: sec-regexp.prototype-%symbol.replace%   type: built-in function

This method performs the following steps when called:

1. Let _regexp_ be the *this* value.
2. If _regexp_ is not an Object, throw a *TypeError* exception.
3. Set _string_ to ? ToString(_string_).
4. Let _stringLength_ be the length of _string_.
5. Let _functionalReplace_ be IsCallable(_replaceValue_).
6. If _functionalReplace_ is *false*, then
   a. Set _replaceValue_ to ? ToString(_replaceValue_).
7. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
8. If _flags_ contains *"g"*, let _global_ be *true*; else let _global_ be *false*.
9. If _global_ is *true*, then
   a. Perform ? Set(_regexp_, *"lastIndex"*, *+0*_𝔽, *true*).
10. Let _results_ be a new empty List.
11. Let _done_ be *false*.
12. Repeat, while _done_ is *false*,
   a. Let _result_ be ? RegExpExec(_regexp_, _string_).
   b. If _result_ is *null*, then
      i. Set _done_ to *true*.
   c. Else,
      i. Append _result_ to _results_.
      ii. If _global_ is *false*, then
         1. Set _done_ to *true*.
      iii. Else,
         1. Let _matchString_ be ? ToString(? Get(_result_, *"0"*)).
         2. If _matchString_ is the empty String, then
            a. Let _thisIndex_ be ℝ(? ToLength(? Get(_regexp_, *"lastIndex"*))).
            b. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*; else let _fullUnicode_ be *false*.
            c. Let _nextIndex_ be AdvanceStringIndex(_string_, _thisIndex_, _fullUnicode_).
            d. Perform ? Set(_regexp_, *"lastIndex"*, 𝔽(_nextIndex_), *true*).
13. Let _accumulatedResult_ be the empty String.
14. Let _nextSourcePosition_ be 0.
15. For each element _result_ of _results_, do
   a. Let _resultLength_ be ? LengthOfArrayLike(_result_).
   b. Let _capturesCount_ be max(_resultLength_ - 1, 0).
   c. Let _matched_ be ? ToString(? Get(_result_, *"0"*)).
   d. Let _matchLength_ be the length of _matched_.
   e. Let _position_ be ? ToIntegerOrInfinity(? Get(_result_, *"index"*)).
   f. Set _position_ to the result of clamping _position_ between 0 and _stringLength_.
   g. Let _captures_ be a new empty List.
   h. Let _captureNumber_ be 1.
   i. Repeat, while _captureNumber_ ≤ _capturesCount_,
      i. Let _capture_ be ? Get(_result_, ! ToString(𝔽(_captureNumber_))).
      ii. If _capture_ is not *undefined*, then
         1. Set _capture_ to ? ToString(_capture_).
      iii. Append _capture_ to _captures_.
      iv. NOTE: When _captureNumber_ = 1, the preceding step puts the first element into _captures_ (at index 0). More generally, the _captureNumber_^th capture (the characters captured by the _captureNumber_^th set of capturing parentheses) is at _captures_[_captureNumber_ - 1].
      v. Set _captureNumber_ to _captureNumber_ + 1.
   j. Let _namedCaptures_ be ? Get(_result_, *"groups"*).
   k. If _functionalReplace_ is *true*, then
      i. Let _replacerArgs_ be the list-concatenation of « _matched_ », _captures_, and « 𝔽(_position_), _string_ ».
      ii. If _namedCaptures_ is not *undefined*, then
         1. Append _namedCaptures_ to _replacerArgs_.
      iii. Let _replacementValue_ be ? Call(_replaceValue_, *undefined*, _replacerArgs_).
      iv. Let _replacementString_ be ? ToString(_replacementValue_).
   l. Else,
      i. If _namedCaptures_ is not *undefined*, then
         1. Set _namedCaptures_ to ? ToObject(_namedCaptures_).
      ii. Let _replacementString_ be ? GetSubstitution(_matched_, _string_, _position_, _captures_, _namedCaptures_, _replaceValue_).
   m. If _position_ ≥ _nextSourcePosition_, then
      i. NOTE: _position_ should not normally move backwards. If it does, it is an indication of an ill-behaving RegExp subclass or use of an access triggered side-effect to change the global flag or other characteristics of _regexp_. In such cases, the corresponding substitution is ignored.
      ii. Set _accumulatedResult_ to the string-concatenation of _accumulatedResult_, the substring of _string_ from _nextSourcePosition_ to _position_, and _replacementString_.
      iii. Set _nextSourcePosition_ to _position_ + _matchLength_.
16. If _nextSourcePosition_ ≥ _stringLength_, return _accumulatedResult_.
17. Return the string-concatenation of _accumulatedResult_ and the substring of _string_ from _nextSourcePosition_.

The value of the *"name"* property of this method is *"[Symbol.replace]"*.

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

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: RegExpExec ( _regexp_: an Object, _string_: a String, ): either a normal completion containing either an Object or *null*, or a throw completion
  id: sec-regexpexec

1. Let _exec_ be ? Get(_regexp_, *"exec"*).
2. If IsCallable(_exec_) is *true*, then
   a. Let _result_ be ? Call(_exec_, _regexp_, « _string_ »).
   b. If _result_ is not an Object and _result_ is not *null*, throw a *TypeError* exception.
   c. Return _result_.
3. Perform ? RequireInternalSlot(_regexp_, [[RegExpMatcher]]).
4. Return ? RegExpBuiltinExec(_regexp_, _string_).

NOTE:
If a callable *"exec"* property is not found this algorithm falls back to attempting to use the built-in RegExp matching algorithm. This provides compatible behaviour for code written for prior editions where most built-in algorithms that use regular expressions did not perform a dynamic property lookup of *"exec"*.

--- referenced: ToLength ( _arg_: an ECMAScript language value, ): either a normal completion containing a non-negative integral Number or a throw completion
  id: sec-tolength

description: It clamps and truncates _arg_ to a non-negative integral Number suitable for use as the length of an array-like object.

1. Let _length_ be ? ToIntegerOrInfinity(_arg_).
2. Let _clampedLength_ be the result of clamping _length_ between 0 and 2^53 - 1.
3. Return 𝔽(_clampedLength_).

--- referenced: AdvanceStringIndex ( _string_: a String, _index_: a non-negative integer, _unicode_: a Boolean, ): a non-negative integer
  id: sec-advancestringindex

1. Assert: _index_ ≤ 2^53 - 1.
2. If _unicode_ is *false*, return _index_ + 1.
3. Let _length_ be the length of _string_.
4. If _index_ + 1 ≥ _length_, return _index_ + 1.
5. Let _codePoint_ be CodePointAt(_string_, _index_).
6. Return _index_ + _codePoint_.[[CodeUnitCount]].

--- referenced: LengthOfArrayLike ( _obj_: an Object, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-lengthofarraylike

description: It returns the value of the *"length"* property of an array-like object.

1. Return ℝ(? ToLength(? Get(_obj_, *"length"*))).

An array-like object is any object for which this operation returns a normal completion.
NOTE:
Typically, an array-like object would also have some properties with integer index names. However, that is not a requirement of this definition.
NOTE:
Arrays and String objects are examples of array-like objects.

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

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).


## Done when

- Every test262 run under built-ins/RegExp/prototype/Symbol.replace/ passes (check with `./t262 built-ins/RegExp/prototype/Symbol.replace/`). 8 pass on the main branch right now.

