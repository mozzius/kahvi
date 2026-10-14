# Ticket B-typed-array-prototype-reduce-right: TypedArray.prototype.reduceRight

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

- `crates/kahvi-builtins/src/typed_array/prototype_reduce_right.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.reduceRight ( _callback_ [ , _initialValue_ ] )
  id: sec-%typedarray%.prototype.reduceright   type: built-in function

The interpretation and use of the arguments of this method are the same as for `Array.prototype.reduceRight` as defined in [sec-array.prototype.reduceright].
This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Let _length_ be TypedArrayLength(_taRecord_).
4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
5. If _length_ = 0 and _initialValue_ is not present, throw a *TypeError* exception.
6. Let _k_ be _length_ - 1.
7. Let _accumulator_ be *undefined*.
8. If _initialValue_ is present, then
   a. Set _accumulator_ to _initialValue_.
9. Else,
   a. Set _accumulator_ to TypedArrayGetElement(_obj_, 𝔽(_k_)).
   b. Set _k_ to _k_ - 1.
10. Repeat, while _k_ ≥ 0,
   a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
   b. Set _accumulator_ to ? Call(_callback_, *undefined*, « _accumulator_, _kValue_, 𝔽(_k_), _obj_ »).
   c. Set _k_ to _k_ - 1.
11. Return _accumulator_.

This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.

--- referenced: ValidateTypedArray ( _obj_: an ECMAScript language value, _order_: ~seq-cst~ or ~unordered~, ): either a normal completion containing a TypedArray With Buffer Witness Record or a throw completion
  id: sec-validatetypedarray

1. Perform ? RequireInternalSlot(_obj_, [[TypedArrayName]]).
2. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
3. Return ? ValidateTypedArrayBounds(_obj_, _order_).

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

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

--- referenced: TypedArrayGetElement ( _obj_: a TypedArray, _index_: a Number, ): a Number, a BigInt, or *undefined*
  id: sec-typedarraygetelement

1. If IsValidIntegerIndex(_obj_, _index_) is *false*, return *undefined*.
2. Let _offset_ be _obj_.[[ByteOffset]].
3. Let _elementSize_ be TypedArrayElementSize(_obj_).
4. Let _byteIndexInBuffer_ be (ℝ(_index_) × _elementSize_) + _offset_.
5. Let _elementType_ be TypedArrayElementType(_obj_).
6. Return GetValueFromBuffer(_obj_.[[ViewedArrayBuffer]], _byteIndexInBuffer_, _elementType_, *true*, ~unordered~).

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).


## Done when

- Every test262 run under built-ins/TypedArray/prototype/reduceRight/ passes (check with `./t262 built-ins/TypedArray/prototype/reduceRight/`). 8 pass on the main branch right now.

## Previous attempts

- attempt 117: rejected by gates: files outside scope: crates/kahvi-core/src/host.rs, crates/kahvi-core/src/object/host_object.rs, crates/kahvi-ffi/src/lib.rs; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:227: unsafe fn enter_callback<T>(; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:281: String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(source, len) }).into_owned();; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:299: unsafe {; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:372: pub unsafe extern "C" fn kahvi_strict_equals(; lint: unsafe is architect-on
- attempt 117: escalated from haiku to sonnet
- attempt 121: worker reported BLOCKED but changed files: BigInt support and `Array.prototype.reverse` are not implemented, and the remaining `resizable-buffer*.js` failures depend on them.
- attempt 121: rejected by gates: files outside scope: crates/kahvi-core/src/host.rs, crates/kahvi-core/src/object/host_object.rs, crates/kahvi-ffi/src/lib.rs; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:227: unsafe fn enter_callback<T>(; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:281: String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(source, len) }).into_owned();; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:299: unsafe {; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:372: pub unsafe extern "C" fn kahvi_strict_equals(; lint: unsafe is architect-on

