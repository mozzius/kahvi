# Ticket B-set-prototype-symmetric-difference: Set.prototype.symmetricDifference

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

- `crates/kahvi-builtins/src/set/prototype_symmetric_difference.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Set.prototype.symmetricDifference ( _other_ )
  id: sec-set.prototype.symmetricdifference   type: built-in function

This method performs the following steps when called:

1. Let _set_ be the *this* value.
2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
3. Let _otherRecord_ be ? GetSetRecord(_other_).
4. Let _keysIterator_ be ? GetIteratorFromMethod(_otherRecord_.[[SetObject]], _otherRecord_.[[Keys]]).
5. Let _resultSetData_ be a copy of _set_.[[SetData]].
6. Let _next_ be ~not-started~.
7. Repeat, while _next_ is not ~done~,
   a. Set _next_ to ? IteratorStepValue(_keysIterator_).
   b. If _next_ is not ~done~, then
      i. Set _next_ to CanonicalizeKeyedCollectionKey(_next_).
      ii. Let _resultIndex_ be SetDataIndex(_resultSetData_, _next_).
      iii. If _resultIndex_ is ~not-found~, let _alreadyInResult_ be *false*; else let _alreadyInResult_ be *true*.
      iv. If SetDataHas(_set_.[[SetData]], _next_) is *true*, then
         1. If _alreadyInResult_ is *true*, set _resultSetData_[_resultIndex_] to ~empty~.
      v. Else,
         1. If _alreadyInResult_ is *false*, append _next_ to _resultSetData_.
8. Let _result_ be OrdinaryObjectCreate(%Set.prototype%, « [[SetData]] »).
9. Set _result_.[[SetData]] to _resultSetData_.
10. Return _result_.

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

--- referenced: CanonicalizeKeyedCollectionKey ( _key_: an ECMAScript language value, ): an ECMAScript language value
  id: sec-canonicalizekeyedcollectionkey

1. If _key_ is *-0*_𝔽, return *+0*_𝔽.
2. Return _key_.

--- referenced: SetDataIndex ( _setData_: a List of either ECMAScript language values or ~empty~, _value_: an ECMAScript language value, ): a non-negative integer or ~not-found~
  id: sec-setdataindex

1. Set _value_ to CanonicalizeKeyedCollectionKey(_value_).
2. Let _size_ be the number of elements in _setData_.
3. Let _index_ be 0.
4. Repeat, while _index_ < _size_,
   a. Let _element_ be _setData_[_index_].
   b. If _element_ is not ~empty~ and _element_ is _value_, then
      i. Return _index_.
   c. Set _index_ to _index_ + 1.
5. Return ~not-found~.

--- referenced: SetDataHas ( _setData_: a List of either ECMAScript language values or ~empty~, _value_: an ECMAScript language value, ): a Boolean
  id: sec-setdatahas

1. If SetDataIndex(_setData_, _value_) is ~not-found~, return *false*.
2. Return *true*.

--- referenced: OrdinaryObjectCreate ( _proto_: an Object or *null*, optional _additionalInternalSlotsList_: a List of names of internal slots, ): an Object
  id: sec-ordinaryobjectcreate

description: It is used to specify the runtime creation of new ordinary objects. _additionalInternalSlotsList_ contains the names of additional internal slots that must be defined as part of the object, beyond [[Prototype]] and [[Extensible]]. If _additionalInternalSlotsList_ is not provided, a new empty List is used.

1. Let _internalSlotsList_ be « [[Prototype]], [[Extensible]] ».
2. If _additionalInternalSlotsList_ is present, set _internalSlotsList_ to the list-concatenation of _internalSlotsList_ and _additionalInternalSlotsList_.
3. Let _obj_ be MakeBasicObject(_internalSlotsList_).
4. Set _obj_.[[Prototype]] to _proto_.
5. Return _obj_.

NOTE:
Although OrdinaryObjectCreate does little more than call MakeBasicObject, its use communicates the intention to create an ordinary object, and not an exotic one. Thus, within this specification, it is not called by any algorithm that subsequently modifies the internal methods of the object in ways that would make the result non-ordinary. Operations that create exotic objects invoke MakeBasicObject directly.


## Done when

- Every test262 run under built-ins/Set/prototype/symmetricDifference/ passes (check with `./t262 built-ins/Set/prototype/symmetricDifference/`). 10 pass on the main branch right now.

