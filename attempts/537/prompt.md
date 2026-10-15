# Ticket B-shared-array-buffer-prototype-slice: SharedArrayBuffer.prototype.slice

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

- `crates/kahvi-builtins/src/shared_array_buffer/prototype_slice.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/shared_array_buffer/constructor.rs`
- `crates/kahvi-builtins/src/array_buffer/prototype_slice.rs`

## Spec

SharedArrayBuffer.prototype.slice ( _start_, _end_ )
  id: sec-sharedarraybuffer.prototype.slice   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
3. If IsSharedArrayBuffer(_obj_) is *false*, throw a *TypeError* exception.
4. Let _length_ be ArrayBufferByteLength(_obj_, ~seq-cst~).
5. Let _first_ be ? ToClampedIndex(_start_, _length_).
6. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
7. Let _newLength_ be max(_final_ - _first_, 0).
8. Let _ctor_ be ? SpeciesConstructor(_obj_, %SharedArrayBuffer%).
9. Let _new_ be ? Construct(_ctor_, « 𝔽(_newLength_) »).
10. Perform ? RequireInternalSlot(_new_, [[ArrayBufferData]]).
11. If IsSharedArrayBuffer(_new_) is *false*, throw a *TypeError* exception.
12. If _new_.[[ArrayBufferData]] is _obj_.[[ArrayBufferData]], throw a *TypeError* exception.
13. If ArrayBufferByteLength(_new_, ~seq-cst~) < _newLength_, throw a *TypeError* exception.
14. Let _fromBlock_ be _obj_.[[ArrayBufferData]].
15. Let _toBlock_ be _new_.[[ArrayBufferData]].
16. Perform CopyDataBlockBytes(_toBlock_, 0, _fromBlock_, _first_, _newLength_).
17. Return _new_.

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

--- referenced: ArrayBufferByteLength ( _arrayBuffer_: an ArrayBuffer or SharedArrayBuffer, _order_: ~seq-cst~ or ~unordered~, ): a non-negative integer
  id: sec-arraybufferbytelength

1. If IsGrowableSharedArrayBuffer(_arrayBuffer_) is *true*, then
   a. Let _bufferByteLengthBlock_ be _arrayBuffer_.[[ArrayBufferByteLengthData]].
   b. Let _rawLength_ be GetRawBytesFromSharedBlock(_bufferByteLengthBlock_, 0, ~biguint64~, *true*, _order_).
   c. Let _agentRecord_ be the Agent Record of the surrounding agent.
   d. Let _isLittleEndian_ be _agentRecord_.[[LittleEndian]].
   e. Return ℝ(RawBytesToNumeric(~biguint64~, _rawLength_, _isLittleEndian_)).
2. Assert: IsDetachedBuffer(_arrayBuffer_) is *false*.
3. Return _arrayBuffer_.[[ArrayBufferByteLength]].

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


## Done when

- Every test262 run under built-ins/SharedArrayBuffer/prototype/slice/ passes (check with `./t262 built-ins/SharedArrayBuffer/prototype/slice/`). 12 pass on the main branch right now.

