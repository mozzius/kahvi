# Ticket B-set-prototype-is-superset-of: Set.prototype.isSupersetOf

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

- `crates/kahvi-builtins/src/set/prototype_is_superset_of.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Set.prototype.isSupersetOf ( _other_ )
  id: sec-set.prototype.issupersetof   type: built-in function

This method performs the following steps when called:

1. Let _set_ be the *this* value.
2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
3. Let _otherRecord_ be ? GetSetRecord(_other_).
4. If SetDataSize(_set_.[[SetData]]) < _otherRecord_.[[Size]], return *false*.
5. Let _keysIterator_ be ? GetIteratorFromMethod(_otherRecord_.[[SetObject]], _otherRecord_.[[Keys]]).
6. Let _next_ be ~not-started~.
7. Repeat, while _next_ is not ~done~,
   a. Set _next_ to ? IteratorStepValue(_keysIterator_).
   b. If _next_ is not ~done~, then
      i. If SetDataHas(_set_.[[SetData]], _next_) is *false*, then
         1. Perform ? IteratorClose(_keysIterator_, NormalCompletion(~unused~)).
         2. Return *false*.
8. Return *true*.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: GetSetRecord ( _obj_: an ECMAScript language value, ): either a normal completion containing a Set Record or a throw completion
  id: sec-getsetrecord

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. Let _rawSize_ be ? Get(_obj_, *"size"*).
3. Let _numberSize_ be ? ToNumber(_rawSize_).
4. NOTE: If _rawSize_ is *undefined*, then _numberSize_ will be *NaN*.
5. If _numberSize_ is *NaN*, throw a *TypeError* exception.
6. Let _intSize_ be ! ToIntegerOrInfinity(_numberSize_).
7. If _intSize_ < 0, throw a *RangeError* exception.
8. Let _has_ be ? Get(_obj_, *"has"*).
9. If IsCallable(_has_) is *false*, throw a *TypeError* exception.
10. Let _keys_ be ? Get(_obj_, *"keys"*).
11. If IsCallable(_keys_) is *false*, throw a *TypeError* exception.
12. Return a new Set Record { [[SetObject]]: _obj_, [[Size]]: _intSize_, [[Has]]: _has_, [[Keys]]: _keys_ }.

--- referenced: SetDataSize ( _setData_: a List of either ECMAScript language values or ~empty~, ): a non-negative integer
  id: sec-setdatasize

1. Let _count_ be 0.
2. For each element _element_ of _setData_, do
   a. If _element_ is not ~empty~, set _count_ to _count_ + 1.
3. Return _count_.

--- referenced: GetIteratorFromMethod ( _obj_: an ECMAScript language value, _method_: a function object, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratorfrommethod

1. Let _iterator_ be ? Call(_method_, _obj_).
2. If _iterator_ is not an Object, throw a *TypeError* exception.
3. Return ? GetIteratorDirect(_iterator_).

--- referenced: IteratorStepValue ( _iteratorRecord_: an Iterator Record, ): either a normal completion containing either an ECMAScript language value or ~done~, or a throw completion
  id: sec-iteratorstepvalue

description: It requests the next value from _iteratorRecord_.[[Iterator]] by calling _iteratorRecord_.[[NextMethod]] and returns either ~done~ indicating that the iterator has reached its end or the value from the IteratorResult object if a next value is available.

1. Let _result_ be ? IteratorStep(_iteratorRecord_).
2. If _result_ is ~done~, then
   a. Return ~done~.
3. Let _value_ be Completion(IteratorValue(_result_)).
4. If _value_ is a throw completion, then
   a. Set _iteratorRecord_.[[Done]] to *true*.
5. Return ? _value_.

--- referenced: SetDataHas ( _setData_: a List of either ECMAScript language values or ~empty~, _value_: an ECMAScript language value, ): a Boolean
  id: sec-setdatahas

1. If SetDataIndex(_setData_, _value_) is ~not-found~, return *false*.
2. Return *true*.

--- referenced: IteratorClose ( _iteratorRecord_: an Iterator Record, _completion_: a Completion Record, ): a Completion Record
  id: sec-iteratorclose

description: It is used to notify an iterator that it should perform any actions it would normally perform when it has reached its completed state.

1. Assert: _iteratorRecord_.[[Iterator]] is an Object.
2. Let _iterator_ be _iteratorRecord_.[[Iterator]].
3. Let _innerResult_ be Completion(GetMethod(_iterator_, *"return"*)).
4. If _innerResult_ is a normal completion, then
   a. Let _return_ be _innerResult_.[[Value]].
   b. If _return_ is *undefined*, return ? _completion_.
   c. Set _innerResult_ to Completion(Call(_return_, _iterator_)).
5. If _completion_ is a throw completion, return ? _completion_.
6. If _innerResult_ is a throw completion, return ? _innerResult_.
7. If _innerResult_.[[Value]] is not an Object, throw a *TypeError* exception.
8. Return ? _completion_.

--- referenced: NormalCompletion ( _value_: any value except a Completion Record, ): a normal completion
  id: sec-normalcompletion

skip return checks: true

1. Return Completion Record { [[Type]]: ~normal~, [[Value]]: _value_, [[Target]]: ~empty~ }.


## Done when

- Every test262 run under built-ins/Set/prototype/isSupersetOf/ passes (check with `./t262 built-ins/Set/prototype/isSupersetOf/`). 10 pass on the main branch right now.

