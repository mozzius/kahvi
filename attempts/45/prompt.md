# Ticket B-array-from-async: Array.fromAsync

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

- `crates/kahvi-builtins/src/array/from_async.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Array.fromAsync ( _items_ [ , _mapper_ [ , _thisArg_ ] ] )
  id: sec-array.fromasync   type: built-in function

This async function performs the following steps when called:

1. Let _ctor_ be the *this* value.
2. Let _mapping_ be *false*.
3. If _mapper_ is not *undefined*, then
   a. If IsCallable(_mapper_) is *false*, throw a *TypeError* exception.
   b. Set _mapping_ to *true*.
4. Let _iteratorRecord_ be *undefined*.
5. Let _usingAsyncIterator_ be ? GetMethod(_items_, %Symbol.asyncIterator%).
6. If _usingAsyncIterator_ is *undefined*, then
   a. Let _usingSyncIterator_ be ? GetMethod(_items_, %Symbol.iterator%).
   b. If _usingSyncIterator_ is not *undefined*, then
      i. Set _iteratorRecord_ to CreateAsyncFromSyncIterator(? GetIteratorFromMethod(_items_, _usingSyncIterator_)).
7. Else,
   a. Set _iteratorRecord_ to ? GetIteratorFromMethod(_items_, _usingAsyncIterator_).
8. If _iteratorRecord_ is not *undefined*, then
   a. If IsConstructor(_ctor_) is *true*, then
      i. Let _array_ be ? Construct(_ctor_).
   b. Else,
      i. Let _array_ be ! ArrayCreate(0).
   c. Let _k_ be 0.
   d. Repeat,
      i. If _k_ ≥ 2^53 - 1, then
         1. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
         2. Return ? AsyncIteratorClose(_iteratorRecord_, _error_).
      ii. Let _propertyKey_ be ! ToString(𝔽(_k_)).
      iii. Let _nextResult_ be ? Call(_iteratorRecord_.[[NextMethod]], _iteratorRecord_.[[Iterator]]).
      iv. Set _nextResult_ to ? Await(_nextResult_).
      v. If _nextResult_ is not an Object, throw a *TypeError* exception.
      vi. Let _done_ be ? IteratorComplete(_nextResult_).
      vii. If _done_ is *true*, then
         1. Perform ? Set(_array_, *"length"*, 𝔽(_k_), *true*).
         2. Return _array_.
      viii. Let _nextValue_ be ? IteratorValue(_nextResult_).
      ix. If _mapping_ is *true*, then
         1. Let _mappedValue_ be Completion(Call(_mapper_, _thisArg_, « _nextValue_, 𝔽(_k_) »)).
         2. IfAbruptCloseAsyncIterator(_mappedValue_, _iteratorRecord_).
         3. Set _mappedValue_ to Completion(Await(_mappedValue_)).
         4. IfAbruptCloseAsyncIterator(_mappedValue_, _iteratorRecord_).
      x. Else,
         1. Let _mappedValue_ be _nextValue_.
      xi. Let _defineStatus_ be Completion(CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_)).
      xii. IfAbruptCloseAsyncIterator(_defineStatus_, _iteratorRecord_).
      xiii. Set _k_ to _k_ + 1.
9. Else,
   a. NOTE: _items_ is neither async iterable nor iterable so assume it is an array-like object.
   b. Let _arrayLike_ be ! ToObject(_items_).
   c. Let _length_ be ? LengthOfArrayLike(_arrayLike_).
   d. If IsConstructor(_ctor_) is *true*, then
      i. Let _array_ be ? Construct(_ctor_, « 𝔽(_length_) »).
   e. Else,
      i. Let _array_ be ? ArrayCreate(_length_).
   f. Let _k_ be 0.
   g. Repeat, while _k_ < _length_,
      i. Let _propertyKey_ be ! ToString(𝔽(_k_)).
      ii. Let _kValue_ be ? Get(_arrayLike_, _propertyKey_).
      iii. Set _kValue_ to ? Await(_kValue_).
      iv. If _mapping_ is *true*, then
         1. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
         2. Set _mappedValue_ to ? Await(_mappedValue_).
      v. Else,
         1. Let _mappedValue_ be _kValue_.
      vi. Perform ? CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_).
      vii. Set _k_ to _k_ + 1.
   h. Perform ? Set(_array_, *"length"*, 𝔽(_length_), *true*).
   i. Return _array_.

NOTE:
This method is an intentionally generic factory method; it does not require that its *this* value be the Array constructor. Therefore it can be transferred to or inherited by any other constructors that may be called with a single numeric argument.

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

--- referenced: GetMethod ( _value_: an ECMAScript language value, _propertyKey_: a property key, ): either a normal completion containing either a function object or *undefined*, or a throw completion
  id: sec-getmethod

description: It is used to get the value of a specific property of an ECMAScript language value when the value of the property is expected to be a function.

1. Let _func_ be ? GetV(_value_, _propertyKey_).
2. If _func_ is either *undefined* or *null*, return *undefined*.
3. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
4. Return _func_.

--- referenced: CreateAsyncFromSyncIterator ( _syncIteratorRecord_: an Iterator Record, ): an Iterator Record
  id: sec-createasyncfromsynciterator

description: It is used to create an async Iterator Record from a synchronous Iterator Record.

1. Let _asyncIterator_ be OrdinaryObjectCreate(%AsyncFromSyncIteratorPrototype%, « [[SyncIteratorRecord]] »).
2. Set _asyncIterator_.[[SyncIteratorRecord]] to _syncIteratorRecord_.
3. Let _nextMethod_ be ! Get(_asyncIterator_, *"next"*).
4. Let _iteratorRecord_ be the Iterator Record { [[Iterator]]: _asyncIterator_, [[NextMethod]]: _nextMethod_, [[Done]]: *false* }.
5. Return _iteratorRecord_.

--- referenced: GetIteratorFromMethod ( _obj_: an ECMAScript language value, _method_: a function object, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratorfrommethod

1. Let _iterator_ be ? Call(_method_, _obj_).
2. If _iterator_ is not an Object, throw a *TypeError* exception.
3. Return ? GetIteratorDirect(_iterator_).

--- referenced: IsConstructor ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-isconstructor

description: It determines if _arg_ is a function object with a [[Construct]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Construct]] internal method, return *true*.
3. Return *false*.

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

--- referenced: ThrowCompletion ( _value_: an ECMAScript language value, ): a throw completion
  id: sec-throwcompletion

1. Return Completion Record { [[Type]]: ~throw~, [[Value]]: _value_, [[Target]]: ~empty~ }.

--- referenced: AsyncIteratorClose ( _iteratorRecord_: an Iterator Record, _completion_: a Completion Record, ): a Completion Record
  id: sec-asynciteratorclose

description: It is used to notify an async iterator that it should perform any actions it would normally perform when it has reached its completed state.

1. Assert: _iteratorRecord_.[[Iterator]] is an Object.
2. Let _iterator_ be _iteratorRecord_.[[Iterator]].
3. Let _innerResult_ be Completion(GetMethod(_iterator_, *"return"*)).
4. If _innerResult_ is a normal completion, then
   a. Let _return_ be _innerResult_.[[Value]].
   b. If _return_ is *undefined*, return ? _completion_.
   c. Set _innerResult_ to Completion(Call(_return_, _iterator_)).
   d. If _innerResult_ is a normal completion, set _innerResult_ to Completion(Await(_innerResult_.[[Value]])).
5. If _completion_ is a throw completion, return ? _completion_.
6. If _innerResult_ is a throw completion, return ? _innerResult_.
7. If _innerResult_.[[Value]] is not an Object, throw a *TypeError* exception.
8. Return ? _completion_.

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


## Done when

- Every test262 run under built-ins/Array/fromAsync/ passes (check with `./t262 built-ins/Array/fromAsync/`). 10 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 43: worker reported BLOCKED but changed files: I could not build or run the Rust compiler or test262 here because the `cargo build` command was denied by the environment, so compile errors and test results are unknown.
- attempt 43: not merged (swallows_errors, skips_observable_steps, contradicts_spec); continue from this diff. Fix the define-failure close path. In define_or_close (used from next_fulfilled, the non-mapping branch at roughly lines 366-368, and from step's Pending::Mapped arm at roughly lines 268-269), a pending AsyncIteratorClose must stop the loop. Right now close_on_error returns Ok(()) once return() is being awaited, and the callers then run iter_step, which calls next() again and overwrites Pending::Close. Change define_or_close so that, when the close is pending, the caller returns immediately (for example, return a bool "continue" flag, or return Ok(()) early from the caller instead of calling iter_step). Also check that the rejection with the original TypeError still happens after the return() await settles (Pending::Close returns the stored error). Everything else in the diff matched the spec on review; keep it.

