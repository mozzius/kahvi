# Ticket B-data-view-prototype-set-uint32: DataView.prototype.setUint32

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

- `crates/kahvi-builtins/src/data_view/prototype_set_uint32.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

DataView.prototype.setUint32 ( _byteOffset_, _value_ [ , _littleEndian_ ] )
  id: sec-dataview.prototype.setuint32   type: built-in function

This method performs the following steps when called:

1. Let _view_ be the *this* value.
2. If _littleEndian_ is not present, set _littleEndian_ to *false*.
3. Perform ? SetViewValue(_view_, _byteOffset_, _littleEndian_, ~uint32~, _value_).
4. Return *undefined*.

--- referenced: SetViewValue ( _view_: an ECMAScript language value, _requestIndex_: an ECMAScript language value, _isLittleEndian_: an ECMAScript language value, _type_: a TypedArray element type, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-setviewvalue

description: It is used by functions on DataView instances to store values into the view's buffer.

1. Perform ? RequireInternalSlot(_view_, [[DataView]]).
2. Assert: _view_ has a [[ViewedArrayBuffer]] internal slot.
3. Let _getIndex_ be ? ToIndex(_requestIndex_).
4. If IsBigIntElementType(_type_) is *true*, let _number_ be ? ToBigInt(_value_).
5. Else, let _number_ be ? ToNumber(_value_).
6. Set _isLittleEndian_ to ToBoolean(_isLittleEndian_).
7. Let _viewOffset_ be _view_.[[ByteOffset]].
8. Let _viewRecord_ be MakeDataViewWithBufferWitnessRecord(_view_, ~unordered~).
9. NOTE: Bounds checking is not a synchronizing operation when _view_'s backing buffer is a growable SharedArrayBuffer.
10. If IsViewOutOfBounds(_viewRecord_) is *true*, throw a *TypeError* exception.
11. Let _viewSize_ be GetViewByteLength(_viewRecord_).
12. Let _elementSize_ be the Element Size value specified in [table-the-typedarray-constructors] for Element Type _type_.
13. If _getIndex_ + _elementSize_ > _viewSize_, throw a *RangeError* exception.
14. Let _bufferIndex_ be _getIndex_ + _viewOffset_.
15. Perform SetValueInBuffer(_view_.[[ViewedArrayBuffer]], _bufferIndex_, _type_, _number_, *false*, ~unordered~, _isLittleEndian_).
16. Return ~unused~.


## Done when

- Every test262 run under built-ins/DataView/prototype/setUint32/ passes (check with `./t262 built-ins/DataView/prototype/setUint32/`). 32 pass on the main branch right now.

## Previous attempts

- attempt 185: worker reported BLOCKED but changed files: DataView.prototype.getUint32 is not implemented (its stub returns not_implemented), and the setUint32 tests use it to verify the written values.
- attempt 185: merged partial progress. Still failing:
built-ins/DataView/prototype/setUint32/index-is-out-of-range.js:sloppy  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/index-is-out-of-range.js:strict  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/negative-byteoffset-throws.js:sloppy  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/negative-byteoffset-throws.js:strict  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/no-value-arg.js:sloppy  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/no-value-arg.js:strict  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/set-values-little-endian-order.js:sloppy  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/set-values-little-endian-order.js:strict  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/set-values-return-undefined.js:sloppy  Uncaught InternalError: not implemented: DataView.prototype.getUint32
built-ins/DataView/prototype/setUint32/set-values-return-undefined.js:strict  Uncaught InternalError: not implemented: DataView.prototype.getUint32
- attempt None: its own function reports not implemented again
- attempt 383: produced no changes

