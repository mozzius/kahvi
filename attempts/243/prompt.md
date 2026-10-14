# Ticket B-data-view-prototype-get-byte-length: get DataView.prototype.byteLength

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

- `crates/kahvi-builtins/src/data_view/prototype_get_byte_length.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

get DataView.prototype.byteLength
  id: sec-get-dataview.prototype.bytelength   type: built-in function

`DataView.prototype.byteLength` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? RequireInternalSlot(_obj_, [[DataView]]).
3. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
4. Let _viewRecord_ be MakeDataViewWithBufferWitnessRecord(_obj_, ~seq-cst~).
5. If IsViewOutOfBounds(_viewRecord_) is *true*, throw a *TypeError* exception.
6. Let _size_ be GetViewByteLength(_viewRecord_).
7. Return 𝔽(_size_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: MakeDataViewWithBufferWitnessRecord ( _obj_: a DataView, _order_: ~seq-cst~ or ~unordered~, ): a DataView With Buffer Witness Record
  id: sec-makedataviewwithbufferwitnessrecord

1. Let _buffer_ be _obj_.[[ViewedArrayBuffer]].
2. If IsDetachedBuffer(_buffer_) is *true*, then
   a. Let _byteLength_ be ~detached~.
3. Else,
   a. Let _byteLength_ be ArrayBufferByteLength(_buffer_, _order_).
4. Return the DataView With Buffer Witness Record { [[Object]]: _obj_, [[CachedBufferByteLength]]: _byteLength_ }.

--- referenced: IsViewOutOfBounds ( _viewRecord_: a DataView With Buffer Witness Record, ): a Boolean
  id: sec-isviewoutofbounds

1. Let _view_ be _viewRecord_.[[Object]].
2. Let _bufferByteLength_ be _viewRecord_.[[CachedBufferByteLength]].
3. If IsDetachedBuffer(_view_.[[ViewedArrayBuffer]]) is *true*, then
   a. Assert: _bufferByteLength_ is ~detached~.
   b. Return *true*.
4. Assert: _bufferByteLength_ is a non-negative integer.
5. Let _byteOffsetStart_ be _view_.[[ByteOffset]].
6. If _view_.[[ByteLength]] is ~auto~, then
   a. Let _byteOffsetEnd_ be _bufferByteLength_.
7. Else,
   a. Let _byteOffsetEnd_ be _byteOffsetStart_ + _view_.[[ByteLength]].
8. NOTE: A 0-length DataView whose [[ByteOffset]] is _bufferByteLength_ is not considered out-of-bounds.
9. If _byteOffsetStart_ > _bufferByteLength_ or _byteOffsetEnd_ > _bufferByteLength_, return *true*.
10. Return *false*.

--- referenced: GetViewByteLength ( _viewRecord_: a DataView With Buffer Witness Record, ): a non-negative integer
  id: sec-getviewbytelength

1. Assert: IsViewOutOfBounds(_viewRecord_) is *false*.
2. Let _view_ be _viewRecord_.[[Object]].
3. If _view_.[[ByteLength]] is not ~auto~, return _view_.[[ByteLength]].
4. Assert: IsFixedLengthArrayBuffer(_view_.[[ViewedArrayBuffer]]) is *false*.
5. Let _byteOffset_ be _view_.[[ByteOffset]].
6. Let _byteLength_ be _viewRecord_.[[CachedBufferByteLength]].
7. Assert: _byteLength_ is not ~detached~.
8. Return _byteLength_ - _byteOffset_.


## Done when

- Every test262 run under built-ins/DataView/prototype/byteLength/ passes (check with `./t262 built-ins/DataView/prototype/byteLength/`). 6 pass on the main branch right now.

