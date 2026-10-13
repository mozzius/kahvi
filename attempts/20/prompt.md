# Ticket B-string-from-char-code: String.fromCharCode

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

- `crates/kahvi-builtins/src/string/from_char_code.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`

## Spec

String.fromCharCode ( ..._codeUnits_ )
  id: sec-string.fromcharcode   type: built-in function

This function may be called with any number of arguments which form the rest parameter _codeUnits_.
It performs the following steps when called:

1. Let _result_ be the empty String.
2. For each element _next_ of _codeUnits_, do
   a. Let _nextCU_ be the code unit whose numeric value is ℝ(? ToUint16(_next_)).
   b. Set _result_ to the string-concatenation of _result_ and _nextCU_.
3. Return _result_.

The *"length"* property of this function is *1*_𝔽.

--- referenced: ToUint16 ( _arg_: an ECMAScript language value, ): either a normal completion containing an integral Number or a throw completion
  id: sec-touint16

description: It converts _arg_ to one of 2^16 integral Number values in the inclusive interval from *+0*_𝔽 to 𝔽(2^16 - 1).

1. Let _int_ be ? ToIntegerOrInfinity(_arg_).
2. Return 𝔽(ToFixedSizeInteger(_int_, ~unsigned~, 16)).


## Done when

- Every test262 run under built-ins/String/fromCharCode/ passes (check with `./t262 built-ins/String/fromCharCode/`). 6 pass on the main branch right now.

