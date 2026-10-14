# Ticket B-array-buffer-prototype-slice: ArrayBuffer.prototype.slice

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

- `crates/kahvi-builtins/src/array_buffer/prototype_slice.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

ArrayBuffer.prototype.slice ( _start_, _end_ )
  id: sec-arraybuffer.prototype.slice   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
4. If IsDetachedBuffer(_obj_) is *true*, throw a *TypeError* exception.
5. Let _length_ be _obj_.[[ArrayBufferByteLength]].
6. Let _first_ be ? ToClampedIndex(_start_, _length_).
7. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
8. Let _newLength_ be max(_final_ - _first_, 0).
9. Let _ctor_ be ? SpeciesConstructor(_obj_, %ArrayBuffer%).
10. Let _new_ be ? Construct(_ctor_, « 𝔽(_newLength_) »).
11. Perform ? RequireInternalSlot(_new_, [[ArrayBufferData]]).
12. If IsSharedArrayBuffer(_new_) is *true*, throw a *TypeError* exception.
13. If IsDetachedBuffer(_new_) is *true*, throw a *TypeError* exception.
14. If SameValue(_new_, _obj_) is *true*, throw a *TypeError* exception.
15. If _new_.[[ArrayBufferByteLength]] < _newLength_, throw a *TypeError* exception.
16. NOTE: Side-effects of the above steps may have detached or resized _obj_.
17. If IsDetachedBuffer(_obj_) is *true*, throw a *TypeError* exception.
18. Let _fromBlock_ be _obj_.[[ArrayBufferData]].
19. Let _toBlock_ be _new_.[[ArrayBufferData]].
20. Let _currentLength_ be _obj_.[[ArrayBufferByteLength]].
21. Let _maxCount_ be _currentLength_ - _first_.
22. If _maxCount_ > 0, then
   a. Let _count_ be min(_newLength_, _maxCount_).
   b. Perform CopyDataBlockBytes(_toBlock_, 0, _fromBlock_, _first_, _count_).
23. Return _new_.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: IsSharedArrayBuffer ( _obj_: an ArrayBuffer or a SharedArrayBuffer, ): a Boolean
  id: sec-issharedarraybuffer

description: It tests whether an object is a SharedArrayBuffer.

1. If _obj_.[[ArrayBufferData]] is a Shared Data Block, return *true*.
2. Return *false*.

--- referenced: IsDetachedBuffer ( _arrayBuffer_: an ArrayBuffer or a SharedArrayBuffer, ): a Boolean
  id: sec-isdetachedbuffer

1. If _arrayBuffer_.[[ArrayBufferData]] is *null*, return *true*.
2. Return *false*.

--- referenced: ToClampedIndex ( _value_: an ECMAScript language value, _length_: a non-negative integer, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-toclampedindex

description: It converts _value_ to an integer (interpreting negative values relative to _length_) and clamps the result to the inclusive interval from 0 to _length_.

1. Let _index_ be ? ToAbsoluteIndex(_value_, _length_).
2. Return the result of clamping _index_ between 0 and _length_.

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

--- referenced: Construct ( _ctor_: a constructor, optional _argList_: a List of ECMAScript language values, optional _newTarget_: a constructor, ): either a normal completion containing an Object or a throw completion
  id: sec-construct

description: It is used to call the [[Construct]] internal method of a function object. _argList_ and _newTarget_ are the values to be passed as the corresponding arguments of the internal method. If _argList_ is not present, a new empty List is used as its value. If _newTarget_ is not present, _ctor_ is used as its value.

1. If _newTarget_ is not present, set _newTarget_ to _ctor_.
2. If _argList_ is not present, set _argList_ to a new empty List.
3. Return ? _ctor_.[[Construct]](_argList_, _newTarget_).

NOTE:
If _newTarget_ is not present, this operation is equivalent to: `new F(...argumentsList)`

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

- Every test262 run under built-ins/ArrayBuffer/prototype/slice/ passes (check with `./t262 built-ins/ArrayBuffer/prototype/slice/`). 12 pass on the main branch right now.

