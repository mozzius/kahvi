# Ticket B-reg-exp-string-iterator-prototype-next: RegExpStringIteratorPrototype.next

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

- `crates/kahvi-builtins/src/reg_exp_string_iterator_prototype/next.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

%RegExpStringIteratorPrototype%.next ( )
  id: sec-%regexpstringiteratorprototype%.next   type: built-in function

1. Let _iteratorObj_ be the *this* value.
2. If _iteratorObj_ is not an Object, throw a *TypeError* exception.
3. If _iteratorObj_ does not have all of the internal slots of a RegExp String Iterator Object Instance (see [sec-properties-of-regexp-string-iterator-instances]), throw a *TypeError* exception.
4. If _iteratorObj_.[[Done]] is *true*, then
   a. Return CreateIteratorResultObject(*undefined*, *true*).
5. Let _regexp_ be _iteratorObj_.[[IteratingRegExp]].
6. Let _string_ be _iteratorObj_.[[IteratedString]].
7. Let _global_ be _iteratorObj_.[[Global]].
8. Let _fullUnicode_ be _iteratorObj_.[[Unicode]].
9. Let _match_ be ? RegExpExec(_regexp_, _string_).
10. If _match_ is *null*, then
   a. Set _iteratorObj_.[[Done]] to *true*.
   b. Return CreateIteratorResultObject(*undefined*, *true*).
11. If _global_ is *false*, then
   a. Set _iteratorObj_.[[Done]] to *true*.
   b. Return CreateIteratorResultObject(_match_, *false*).
12. Let _matchString_ be ? ToString(? Get(_match_, *"0"*)).
13. If _matchString_ is the empty String, then
   a. Let _thisIndex_ be ℝ(? ToLength(? Get(_regexp_, *"lastIndex"*))).
   b. Let _nextIndex_ be AdvanceStringIndex(_string_, _thisIndex_, _fullUnicode_).
   c. Perform ? Set(_regexp_, *"lastIndex"*, 𝔽(_nextIndex_), *true*).
14. Return CreateIteratorResultObject(_match_, *false*).

--- referenced: CreateIteratorResultObject ( _value_: an ECMAScript language value, _done_: a Boolean, ): an Object that conforms to the IteratorResult interface
  id: sec-createiterresultobject

description: It creates an object that conforms to the IteratorResult interface.

1. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
2. Perform ! CreateDataPropertyOrThrow(_obj_, *"value"*, _value_).
3. Perform ! CreateDataPropertyOrThrow(_obj_, *"done"*, _done_).
4. Return _obj_.

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

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.


## Done when

- Every test262 run under built-ins/RegExpStringIteratorPrototype/next/ passes (check with `./t262 built-ins/RegExpStringIteratorPrototype/next/`). 0 pass on the main branch right now.

## Previous attempts

- attempt 487: worker reported BLOCKED but changed files: a RegExp String Iterator internal-slot type (with [[IteratingRegExp]], [[IteratedString]], [[Global]], [[Unicode]], [[Done]]) that `CreateRegExpStringIterator` / `String.prototype.matchAll` must create, in `support/regexp.rs` or the match-all ticket's files; the iterator tests fail because `RegExp.prototype[@@matchAll]` still returns `not_implemented`.
- attempt 487: rejected by gates: no new test262 passes in target

