# Ticket B-reg-exp-prototype-symbol-match: RegExp.prototype[Symbol.match]

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

- `crates/kahvi-builtins/src/reg_exp/prototype_symbol_match.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

RegExp.prototype [ %Symbol.match% ] ( _string_ )
  id: sec-regexp.prototype-%symbol.match%   type: built-in function

This method performs the following steps when called:

1. Let _regexp_ be the *this* value.
2. If _regexp_ is not an Object, throw a *TypeError* exception.
3. Set _string_ to ? ToString(_string_).
4. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
5. If _flags_ does not contain *"g"*, return ? RegExpExec(_regexp_, _string_).
6. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*; else let _fullUnicode_ be *false*.
7. Perform ? Set(_regexp_, *"lastIndex"*, *+0*_𝔽, *true*).
8. Let _array_ be ! ArrayCreate(0).
9. Let _matchCount_ be 0.
10. Repeat,
   a. Let _result_ be ? RegExpExec(_regexp_, _string_).
   b. If _result_ is *null*, then
      i. If _matchCount_ = 0, return *null*.
      ii. Return _array_.
   c. Let _matchString_ be ? ToString(? Get(_result_, *"0"*)).
   d. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_matchCount_)), _matchString_).
   e. If _matchString_ is the empty String, then
      i. Let _thisIndex_ be ℝ(? ToLength(? Get(_regexp_, *"lastIndex"*))).
      ii. Let _nextIndex_ be AdvanceStringIndex(_string_, _thisIndex_, _fullUnicode_).
      iii. Perform ? Set(_regexp_, *"lastIndex"*, 𝔽(_nextIndex_), *true*).
   f. Set _matchCount_ to _matchCount_ + 1.

The value of the *"name"* property of this method is *"[Symbol.match]"*.
NOTE:
The %Symbol.match% property is used by the IsRegExp abstract operation to identify objects that have the basic behaviour of regular expressions. The absence of a %Symbol.match% property or the existence of such a property whose value does not Boolean coerce to *true* indicates that the object is not intended to be used as a regular expression object.

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

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.

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

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.

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


## Done when

- Every test262 run under built-ins/RegExp/prototype/Symbol.match/ passes (check with `./t262 built-ins/RegExp/prototype/Symbol.match/`). 8 pass on the main branch right now.

