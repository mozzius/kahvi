# Ticket B-uint8-array-prototype-set-from-hex: Uint8Array.prototype.setFromHex

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

Call FromHex from `uint8_array/from_hex.rs` (`super::from_hex::from_hex`) rather than writing it again.

## Files you may change

- `crates/kahvi-builtins/src/uint8_array/prototype_set_from_hex.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-builtins/src/uint8_array/from_hex.rs`

## Spec

Uint8Array.prototype.setFromHex ( _string_ )
  id: sec-uint8array.prototype.setfromhex   type: built-in function

1. Let _into_ be the *this* value.
2. Perform ? ValidateUint8Array(_into_).
3. If _string_ is not a String, throw a *TypeError* exception.
4. Let _taRecord_ be ? ValidateTypedArrayBounds(_into_, ~seq-cst~).
5. Let _byteLength_ be TypedArrayLength(_taRecord_).
6. Let _result_ be FromHex(_string_, _byteLength_).
7. Let _bytes_ be _result_.[[Bytes]].
8. Let _written_ be the number of elements in _bytes_.
9. NOTE: FromHex does not invoke any user code, so the ArrayBuffer backing _into_ cannot have been detached or shrunk.
10. Assert: _written_ ≤ _byteLength_.
11. Perform SetUint8ArrayBytes(_into_, _bytes_).
12. If _result_.[[Error]] is not ~none~, then
   a. Throw _result_.[[Error]].
13. Let _resultObj_ be OrdinaryObjectCreate(%Object.prototype%).
14. Perform ! CreateDataPropertyOrThrow(_resultObj_, *"read"*, 𝔽(_result_.[[Read]])).
15. Perform ! CreateDataPropertyOrThrow(_resultObj_, *"written"*, 𝔽(_written_)).
16. Return _resultObj_.

--- referenced: ValidateUint8Array ( _ta_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-validateuint8array

1. Perform ? RequireInternalSlot(_ta_, [[TypedArrayName]]).
2. If _ta_.[[TypedArrayName]] is not *"Uint8Array"*, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: ValidateTypedArrayBounds ( _ta_: a TypedArray, _order_: ~seq-cst~ or ~unordered~, ): either a normal completion containing a TypedArray With Buffer Witness Record or a throw completion
  id: sec-validatetypedarraybounds

description: It validates that _ta_ does not exceed the bounds of its backing buffer.

1. Let _taRecord_ be MakeTypedArrayWithBufferWitnessRecord(_ta_, _order_).
2. If IsTypedArrayOutOfBounds(_taRecord_) is *true*, throw a *TypeError* exception.
3. Return _taRecord_.

--- referenced: TypedArrayLength ( _taRecord_: a TypedArray With Buffer Witness Record, ): a non-negative integer
  id: sec-typedarraylength

1. Assert: IsTypedArrayOutOfBounds(_taRecord_) is *false*.
2. Let _obj_ be _taRecord_.[[Object]].
3. If _obj_.[[ArrayLength]] is not ~auto~, return _obj_.[[ArrayLength]].
4. Assert: IsFixedLengthArrayBuffer(_obj_.[[ViewedArrayBuffer]]) is *false*.
5. Let _byteOffset_ be _obj_.[[ByteOffset]].
6. Let _elementSize_ be TypedArrayElementSize(_obj_).
7. Let _byteLength_ be _taRecord_.[[CachedBufferByteLength]].
8. Assert: _byteLength_ is not ~detached~.
9. Return floor((_byteLength_ - _byteOffset_) / _elementSize_).

--- referenced: FromHex ( _string_: a String, optional _maxLength_: a non-negative integer, ): a Record with fields [[Read]] (an integer), [[Bytes]] (a List of byte values), and [[Error]] (a *SyntaxError* object or ~none~)
  id: sec-fromhex

1. If _maxLength_ is not present, set _maxLength_ to 2^53 - 1.
2. Let _length_ be the length of _string_.
3. Let _bytes_ be a new empty List.
4. Let _read_ be 0.
5. If _length_ modulo 2 ≠ 0, then
   a. Let _error_ be a newly created *SyntaxError* object.
   b. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
6. Repeat, while _read_ < _length_ and the number of elements in _bytes_ < _maxLength_,
   a. Let _hexits_ be the substring of _string_ from _read_ to _read_ + 2.
   b. If _hexits_ contains any code units which are not in *"0123456789abcdefABCDEF"*, then
      i. Let _error_ be a newly created *SyntaxError* object.
      ii. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
   c. Set _read_ to _read_ + 2.
   d. Let _byte_ be the integer value represented by _hexits_ in base-16 notation, using the letters *A* through *F* and *a* through *f* for digits with values 10 through 15.
   e. Append _byte_ to _bytes_.
7. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.

--- referenced: SetUint8ArrayBytes ( _into_: a Uint8Array, _bytes_: a List of byte values, ): ~unused~
  id: sec-setuint8arraybytes

1. Let _offset_ be _into_.[[ByteOffset]].
2. Let _length_ be the number of elements in _bytes_.
3. Let _index_ be 0.
4. Repeat, while _index_ < _length_,
   a. Let _byte_ be _bytes_[_index_].
   b. Let _byteIndexInBuffer_ be _index_ + _offset_.
   c. Perform SetValueInBuffer(_into_.[[ViewedArrayBuffer]], _byteIndexInBuffer_, ~uint8~, 𝔽(_byte_), *true*, ~unordered~).
   d. Set _index_ to _index_ + 1.
5. Return ~unused~.

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

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.


## Done when

- Every test262 run under built-ins/Uint8Array/prototype/setFromHex/ passes (check with `./t262 built-ins/Uint8Array/prototype/setFromHex/`). 8 pass on the main branch right now.

