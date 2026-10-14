# Ticket B-reg-exp-prototype-symbol-split: RegExp.prototype[Symbol.split]

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

- `crates/kahvi-builtins/src/reg_exp/prototype_symbol_split.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

…ype [ %Symbol.split% ] ( _string_, _limit_ )
  id: sec-regexp.prototype-%symbol.split%   type: built-in function

NOTE:
This method returns an Array into which substrings of the result of converting _string_ to a String have been stored. The substrings are determined by searching from left to right for matches of the *this* value regular expression; these occurrences are not part of any String in the returned array, but serve to divide up the String value.
The *this* value may be an empty regular expression or a regular expression that can match an empty String. In this case, the regular expression does not match the empty substring at the beginning or end of the input String, nor does it match the empty substring at the end of the previous separator match. (For example, if the regular expression matches the empty String, the String is split up into individual code unit elements; the length of the result array equals the length of the String, and each substring contains one code unit.) Only the first match at a given index of the String is considered, even if backtracking could yield a non-empty substring match at that index. (For example, `/a*?/[Symbol.split]("ab")` evaluates to the array `["a", "b"]`, while `/a*/[Symbol.split]("ab")` evaluates to the array `["","b"]`.)
If _string_ is (or converts to) the empty String, the result depends on whether the regular expression can match the empty String. If it can, the result array contains no elements. Otherwise, the result array contains one element, which is the empty String.
If the regular expression contains capturing parentheses, then each time _separator_ is matched the results (including any *undefined* results) of the capturing parentheses are spliced into the output array. For example,
/<(\/)?([^<>]+)>/[Symbol.split]("A<B>bold</B>and<CODE>coded</CODE>")
evaluates to the array
["A", undefined, "B", "bold", "/", "B", "and", undefined, "CODE", "coded", "/", "CODE", ""]
If _limit_ is not *undefined*, then the output array is truncated so that it contains no more than _limit_ elements.
This method performs the following steps when called:

1. Let _regexp_ be the *this* value.
2. If _regexp_ is not an Object, throw a *TypeError* exception.
3. Set _string_ to ? ToString(_string_).
4. Let _speciesCtor_ be ? SpeciesConstructor(_regexp_, %RegExp%).
5. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
6. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _unicodeMatching_ be *true*.
7. Else, let _unicodeMatching_ be *false*.
8. If _flags_ contains *"y"*, let _newFlags_ be _flags_.
9. Else, let _newFlags_ be the string-concatenation of _flags_ and *"y"*.
10. Let _splitter_ be ? Construct(_speciesCtor_, « _regexp_, _newFlags_ »).
11. Let _array_ be ! ArrayCreate(0).
12. Let _lengthA_ be 0.
13. If _limit_ is *undefined*, let _lim_ be 2^32 - 1; else let _lim_ be ℝ(? ToUint32(_limit_)).
14. If _lim_ = 0, return _array_.
15. If _string_ is the empty String, then
   a. Let _matchResult_ be ? RegExpExec(_splitter_, _string_).
   b. If _matchResult_ is not *null*, return _array_.
   c. Perform ! CreateDataPropertyOrThrow(_array_, *"0"*, _string_).
   d. Return _array_.
16. Let _size_ be the length of _string_.
17. Let _lastMatchEnd_ be 0.
18. Let _searchIndex_ be _lastMatchEnd_.
19. Repeat, while _searchIndex_ < _size_,
   a. Perform ? Set(_splitter_, *"lastIndex"*, 𝔽(_searchIndex_), *true*).
   b. Let _matchResult_ be ? RegExpExec(_splitter_, _string_).
   c. If _matchResult_ is *null*, then
      i. Set _searchIndex_ to AdvanceStringIndex(_string_, _searchIndex_, _unicodeMatching_).
   d. Else,
      i. Let _matchEnd_ be ℝ(? ToLength(? Get(_splitter_, *"lastIndex"*))).
      ii. Set _matchEnd_ to min(_matchEnd_, _size_).
      iii. If _matchEnd_ = _lastMatchEnd_, then
         1. Set _searchIndex_ to AdvanceStringIndex(_string_, _searchIndex_, _unicodeMatching_).
      iv. Else,
         1. Let _substring_ be the substring of _string_ from _lastMatchEnd_ to _searchIndex_.
         2. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _substring_).
         3. Set _lengthA_ to _lengthA_ + 1.
         4. If _lengthA_ = _lim_, return _array_.
         5. Set _lastMatchEnd_ to _matchEnd_.
         6. Let _numberOfCaptures_ be ? LengthOfArrayLike(_matchResult_).
         7. Set _numberOfCaptures_ to max(_numberOfCaptures_ - 1, 0).
         8. Let _captureIndex_ be 1.
         9. Repeat, while _captureIndex_ ≤ _numberOfCaptures_,
            a. Let _nextCapture_ be ? Get(_matchResult_, ! ToString(𝔽(_captureIndex_))).
            b. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _nextCapture_).
            c. Set _captureIndex_ to _captureIndex_ + 1.
            d. Set _lengthA_ to _lengthA_ + 1.
            e. If _lengthA_ = _lim_, return _array_.
         10. Set _searchIndex_ to _lastMatchEnd_.
20. Let _substring_ be the substring of _string_ from _lastMatchEnd_ to _size_.
21. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _substring_).
22. Return _array_.

The value of the *"name"* property of this method is *"[Symbol.split]"*.
NOTE:
This method ignores the value of the *"global"* and *"sticky"* properties of this RegExp object.

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

--- referenced: SpeciesConstructor ( _obj_: an Object, _defaultCtor_: a constructor, ): either a normal completion containing a constructor or a throw completion
  id: sec-speciesconstructor

description: It is used to retrieve the constructor that should be used to create new objects that are derived from _obj_. _defaultCtor_ is the constructor to use if a constructor %Symbol.species% property cannot be found starting from _obj_.

1. Let _ctor_ be ? Get(_obj_, *"constructor"*).
2. If _ctor_ is *undefined*, return _defaultCtor_.
3. If _ctor_ is not an Object, throw a *TypeError* exception.
4. Let _species_ be ? Get(_ctor_, %Symbol.species%).
5. If _species_ is either *undefined* or *null*, return _defaultCtor_.
6. If IsConstructor(_species_) is *true*, return _species_.
7. Throw a *TypeError* exception.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: Construct ( _ctor_: a constructor, optional _argList_: a List of ECMAScript language values, optional _newTarget_: a constructor, ): either a normal completion containing an Object or a throw completion
  id: sec-construct

description: It is used to call the [[Construct]] internal method of a function object. _argList_ and _newTarget_ are the values to be passed as the corresponding arguments of the internal method. If _argList_ is not present, a new empty List is used as its value. If _newTarget_ is not present, _ctor_ is used as its value.

1. If _newTarget_ is not present, set _newTarget_ to _ctor_.
2. If _argList_ is not present, set _argList_ to a new empty List.
3. Return ? _ctor_.[[Construct]](_argList_, _newTarget_).

NOTE:
If _newTarget_ is not present, this operation is equivalent to: `new F(...argumentsList)`

--- referenced: ArrayCreate ( _length_: a non-negative integer, optional _proto_: an Object, ): either a normal completion containing an Array exotic object or a throw completion
  id: sec-arraycreate

description: It is used to specify the creation of new Arrays.

1. If _length_ > 2^32 - 1, throw a *RangeError* exception.
2. If _proto_ is not present, set _proto_ to %Array.prototype%.
3. Let _array_ be MakeBasicObject(« [[Prototype]], [[Extensible]] »).
4. Set _array_.[[Prototype]] to _proto_.
5. Set _array_.[[DefineOwnProperty]] as specified in [sec-array-exotic-objects-defineownproperty-p-desc].
6. Perform ! OrdinaryDefineOwnProperty(_array_, *"length"*, PropertyDescriptor { [[Value]]: 𝔽(_length_), [[Writable]]: *true*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
7. Return _array_.

--- referenced: ToUint32 ( _arg_: an ECMAScript language value, ): either a normal completion containing an integral Number or a throw completion
  id: sec-touint32

description: It converts _arg_ to one of 2^32 integral Number values in the inclusive interval from *+0*_𝔽 to 𝔽(2^32 - 1).

1. Let _int_ be ? ToIntegerOrInfinity(_arg_).
2. Return 𝔽(ToFixedSizeInteger(_int_, ~unsigned~, 32)).

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

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: AdvanceStringIndex ( _string_: a String, _index_: a non-negative integer, _unicode_: a Boolean, ): a non-negative integer
  id: sec-advancestringindex

1. Assert: _index_ ≤ 2^53 - 1.
2. If _unicode_ is *false*, return _index_ + 1.
3. Let _length_ be the length of _string_.
4. If _index_ + 1 ≥ _length_, return _index_ + 1.
5. Let _codePoint_ be CodePointAt(_string_, _index_).
6. Return _index_ + _codePoint_.[[CodeUnitCount]].


## Done when

- Every test262 run under built-ins/RegExp/prototype/Symbol.split/ passes (check with `./t262 built-ins/RegExp/prototype/Symbol.split/`). 8 pass on the main branch right now.

