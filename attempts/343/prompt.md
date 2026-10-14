# Ticket B-math-clz32: Math.clz32

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

- `crates/kahvi-builtins/src/math/clz32.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.clz32 ( _x_ )
  id: sec-math.clz32   type: built-in function

This function performs the following steps when called:

1. Let _n_ be ? ToUint32(_x_).
2. Let _p_ be the number of leading zero bits in the unsigned 32-bit binary representation of _n_.
3. Return 𝔽(_p_).

NOTE:
If _n_ is either *+0*_𝔽 or *-0*_𝔽, this method returns *32*_𝔽. If the most significant bit of the 32-bit binary encoding of _n_ is 1, this method returns *+0*_𝔽.

--- referenced: ToUint32 ( _arg_: an ECMAScript language value, ): either a normal completion containing an integral Number or a throw completion
  id: sec-touint32

description: It converts _arg_ to one of 2^32 integral Number values in the inclusive interval from *+0*_𝔽 to 𝔽(2^32 - 1).

1. Let _int_ be ? ToIntegerOrInfinity(_arg_).
2. Return 𝔽(ToFixedSizeInteger(_int_, ~unsigned~, 32)).


## Done when

- Every test262 run under built-ins/Math/clz32/ passes (check with `./t262 built-ins/Math/clz32/`). 8 pass on the main branch right now.

