# Ticket B-reg-exp-prototype-symbol-search: RegExp.prototype[Symbol.search]

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

- `crates/kahvi-builtins/src/reg_exp/prototype_symbol_search.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

RegExp.prototype [ %Symbol.search% ] ( _string_ )
  id: sec-regexp.prototype-%symbol.search%   type: built-in function

This method performs the following steps when called:

1. Let _regexp_ be the *this* value.
2. If _regexp_ is not an Object, throw a *TypeError* exception.
3. Set _string_ to ? ToString(_string_).
4. Let _previousLastIndex_ be ? Get(_regexp_, *"lastIndex"*).
5. If _previousLastIndex_ is not *+0*_𝔽, then
   a. Perform ? Set(_regexp_, *"lastIndex"*, *+0*_𝔽, *true*).
6. Let _result_ be ? RegExpExec(_regexp_, _string_).
7. Let _currentLastIndex_ be ? Get(_regexp_, *"lastIndex"*).
8. If SameValue(_currentLastIndex_, _previousLastIndex_) is *false*, then
   a. Perform ? Set(_regexp_, *"lastIndex"*, _previousLastIndex_, *true*).
9. If _result_ is *null*, return *-1*_𝔽.
10. Return ? Get(_result_, *"index"*).

The value of the *"name"* property of this method is *"[Symbol.search]"*.
NOTE:
The *"lastIndex"* and *"global"* properties of this RegExp object are ignored when performing the search. The *"lastIndex"* property is left unchanged.

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

--- referenced: SameValue ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, ): a Boolean
  id: sec-samevalue

description: It determines whether or not the two arguments are the same value.

1. If SameType(_x_, _y_) is *false*, return *false*.
2. If _x_ is a Number, then
   a. Return Number::sameValue(_x_, _y_).
3. Return SameValueNonNumber(_x_, _y_).

NOTE:
This algorithm differs from the IsStrictlyEqual Algorithm by treating all *NaN* values as equivalent and by differentiating *+0*_𝔽 from *-0*_𝔽.


## Done when

- Every test262 run under built-ins/RegExp/prototype/Symbol.search/ passes (check with `./t262 built-ins/RegExp/prototype/Symbol.search/`). 8 pass on the main branch right now.

