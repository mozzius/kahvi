# Ticket B-data-view-prototype-get-big-int64: DataView.prototype.getBigInt64

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

- `crates/kahvi-builtins/src/data_view/prototype_get_big_int64.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

DataView.prototype.getBigInt64 ( _byteOffset_ [ , _littleEndian_ ] )
  id: sec-dataview.prototype.getbigint64   type: built-in function

This method performs the following steps when called:

1. Let _view_ be the *this* value.
2. Return ? GetViewValue(_view_, _byteOffset_, _littleEndian_, ~bigint64~).

--- referenced: GetViewValue ( _view_: an ECMAScript language value, _requestIndex_: an ECMAScript language value, _isLittleEndian_: an ECMAScript language value, _type_: a TypedArray element type, ): either a normal completion containing either a Number or a BigInt, or a throw completion
  id: sec-getviewvalue

description: It is used by functions on DataView instances to retrieve values from the view's buffer.

1. Perform ? RequireInternalSlot(_view_, [[DataView]]).
2. Assert: _view_ has a [[ViewedArrayBuffer]] internal slot.
3. Let _getIndex_ be ? ToIndex(_requestIndex_).
4. Set _isLittleEndian_ to ToBoolean(_isLittleEndian_).
5. Let _viewOffset_ be _view_.[[ByteOffset]].
6. Let _viewRecord_ be MakeDataViewWithBufferWitnessRecord(_view_, ~unordered~).
7. NOTE: Bounds checking is not a synchronizing operation when _view_'s backing buffer is a growable SharedArrayBuffer.
8. If IsViewOutOfBounds(_viewRecord_) is *true*, throw a *TypeError* exception.
9. Let _viewSize_ be GetViewByteLength(_viewRecord_).
10. Let _elementSize_ be the Element Size value specified in [table-the-typedarray-constructors] for Element Type _type_.
11. If _getIndex_ + _elementSize_ > _viewSize_, throw a *RangeError* exception.
12. Let _bufferIndex_ be _getIndex_ + _viewOffset_.
13. Return GetValueFromBuffer(_view_.[[ViewedArrayBuffer]], _bufferIndex_, _type_, *false*, ~unordered~, _isLittleEndian_).


## Done when

- Every test262 run under built-ins/DataView/prototype/getBigInt64/ passes (check with `./t262 built-ins/DataView/prototype/getBigInt64/`). 6 pass on the main branch right now.

