# Ticket B-typed-array-prototype-set: TypedArray.prototype.set

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

- `crates/kahvi-builtins/src/typed_array/prototype_set.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.set ( _source_ [ , _offset_ ] )
  id: sec-%typedarray%.prototype.set   type: built-in function

This method sets multiple values in this _TypedArray_, reading the values from _source_. The details differ based upon the type of _source_. The optional _offset_ value indicates the first element index in this _TypedArray_ where values are written. If omitted, it is assumed to be 0.
It performs the following steps when called:

1. Let _target_ be the *this* value.
2. Perform ? RequireInternalSlot(_target_, [[TypedArrayName]]).
3. Assert: _target_ has a [[ViewedArrayBuffer]] internal slot.
4. Let _targetOffset_ be ? ToIntegerOrInfinity(_offset_).
5. If _targetOffset_ < 0, throw a *RangeError* exception.
6. If _source_ is an Object that has a [[TypedArrayName]] internal slot, then
   a. Perform ? SetTypedArrayFromTypedArray(_target_, _targetOffset_, _source_).
7. Else,
   a. Perform ? SetTypedArrayFromArrayLike(_target_, _targetOffset_, _source_).
8. Return *undefined*.

This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
[subclause sec-settypedarrayfromarraylike: SetTypedArrayFromArrayLike (
_target_: a TypedArray,
_targetOffset_: a non-negative integer or +∞,
_source_: an ECMAScript language value, but not a TypedArray,
): either a normal completion containing ~unused~ or a throw completion]
[subclause sec-settypedarrayfromtypedarray: SetTypedArrayFromTypedArray (
_target_: a TypedArray,
_targetOffset_: a non-negative integer or +∞,
_source_: a TypedArray,
): either a normal completion containing ~unused~ or a throw completion]

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: ToIntegerOrInfinity ( _arg_: an ECMAScript language value, ): either a normal completion containing either an integer, +∞, or -∞, or a throw completion
  id: sec-tointegerorinfinity

description: It converts _arg_ to an integer representing its Number value with fractional part truncated, or to +∞ or -∞ when that Number value is infinite.

1. Let _number_ be ? ToNumber(_arg_).
2. If _number_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return 0.
3. If _number_ is *+∞*_𝔽, return +∞.
4. If _number_ is *-∞*_𝔽, return -∞.
5. Return truncate(ℝ(_number_)).

NOTE:
𝔽(ToIntegerOrInfinity(_x_)) never returns *-0*_𝔽 for any value of _x_. The truncation of the fractional part is performed after converting _x_ to a mathematical value.

--- referenced: SetTypedArrayFromArrayLike ( _target_: a TypedArray, _targetOffset_: a non-negative integer or +∞, _source_: an ECMAScript language value, but not a TypedArray, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-settypedarrayfromarraylike

description: It sets multiple values in _target_, starting at index _targetOffset_, reading the values from _source_.

1. Let _targetRecord_ be ? ValidateTypedArrayBounds(_target_, ~seq-cst~).
2. Let _targetLength_ be TypedArrayLength(_targetRecord_).
3. Set _source_ to ? ToObject(_source_).
4. Let _sourceLength_ be ? LengthOfArrayLike(_source_).
5. If _targetOffset_ = +∞, throw a *RangeError* exception.
6. If _sourceLength_ + _targetOffset_ > _targetLength_, throw a *RangeError* exception.
7. Let _k_ be 0.
8. Repeat, while _k_ < _sourceLength_,
   a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
   b. Let _value_ be ? Get(_source_, _propertyKey_).
   c. Let _targetIndex_ be 𝔽(_targetOffset_ + _k_).
   d. Perform ? TypedArraySetElement(_target_, _targetIndex_, _value_).
   e. Set _k_ to _k_ + 1.
9. Return ~unused~.


## Done when

- Every test262 run under built-ins/TypedArray/prototype/set/ passes (check with `./t262 built-ins/TypedArray/prototype/set/`). 8 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 66: worker reported BLOCKED but changed files: `Math.floor`, `TypedArray.prototype.join` and `ArrayBuffer.prototype.resize` are still unimplemented, and the set tests fail on them.
- attempt 66: not merged (contradicts_spec); continue from this diff. The diff is close to mergeable. Only change needed: add SetTypedArrayFromTypedArray step 10 (sameSharedArrayBuffer). Concretely, compute whether srcBuffer and targetBuffer are both shared and share a data block. Then fold that into the aliasing check in step 11 (`if SameValue(srcBuffer, targetBuffer) or sameSharedArrayBuffer`) so the source is cloned. If SharedArrayBuffer cannot be represented in this engine, return agent.not_implemented for that case instead of silently skipping it. Keep the step comments in place. The `.expect(...)` calls are on invariants and are acceptable as they stand. Do not change the copy-failure TypeError fallback unless it becomes reachable. Everything else (order of checks, RangeError on +∞ offset, content-type TypeError, element loop via typed_array_set_element) matches the spec and should be kept.

