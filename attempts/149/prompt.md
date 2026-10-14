# Ticket B-typed-array-prototype-to-locale-string: TypedArray.prototype.toLocaleString

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

- `crates/kahvi-builtins/src/typed_array/prototype_to_locale_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
  id: sec-%typedarray%.prototype.tolocalestring   type: built-in function

This is a distinct method that implements the same algorithm as `Array.prototype.toLocaleString` as defined in [sec-array.prototype.tolocalestring] except that TypedArrayLength is called in place of performing a [[Get]] of *"length"*. The implementation of the algorithm may be optimized with the knowledge that the *this* value has a fixed length when the underlying buffer is not resizable and whose integer-indexed properties are not sparse. However, such optimization must not introduce any observable changes in the specified behaviour of the algorithm.
This method is not generic. ValidateTypedArray is called with the *this* value and ~seq-cst~ as arguments prior to evaluating the algorithm. If its result is an abrupt completion that exception is thrown instead of evaluating the algorithm.
NOTE:
If the ECMAScript implementation includes the ECMA-402 Internationalization API this method is based upon the algorithm for `Array.prototype.toLocaleString` that is in ECMA-402.


## Done when

- Every test262 run under built-ins/TypedArray/prototype/toLocaleString/ passes (check with `./t262 built-ins/TypedArray/prototype/toLocaleString/`). 8 pass on the main branch right now.

