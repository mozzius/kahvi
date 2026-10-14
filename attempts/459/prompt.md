# Ticket B-math-imul: Math.imul

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

- `crates/kahvi-builtins/src/math/imul.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.imul ( _x_, _y_ )
  id: sec-math.imul   type: built-in function

This function performs the following steps when called:

1. Let _a_ be ℝ(? ToUint32(_x_)).
2. Let _b_ be ℝ(? ToUint32(_y_)).
3. Let _product_ be (_a_ × _b_) modulo 2^32.
4. If _product_ ≥ 2^31, return 𝔽(_product_ - 2^32).
5. Return 𝔽(_product_).

--- referenced: ToUint32 ( _arg_: an ECMAScript language value, ): either a normal completion containing an integral Number or a throw completion
  id: sec-touint32

description: It converts _arg_ to one of 2^32 integral Number values in the inclusive interval from *+0*_𝔽 to 𝔽(2^32 - 1).

1. Let _int_ be ? ToIntegerOrInfinity(_arg_).
2. Return 𝔽(ToFixedSizeInteger(_int_, ~unsigned~, 32)).


## Done when

- Every test262 run under built-ins/Math/imul/ passes (check with `./t262 built-ins/Math/imul/`). 8 pass on the main branch right now.

