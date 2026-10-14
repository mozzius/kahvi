# Ticket B-math-expm1: Math.expm1

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

- `crates/kahvi-builtins/src/math/expm1.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.expm1 ( _x_ )
  id: sec-math.expm1   type: built-in function

This function returns the result of subtracting 1 from the exponential function of _x_ (_e_ raised to the power of _x_, where _e_ is the base of the natural logarithms). The result is computed in a way that is accurate even when the value of _x_ is close to 0.
It performs the following steps when called:

1. Let _n_ be ? ToNumber(_x_).
2. If _n_ is one of *NaN*, *+0*_𝔽, *-0*_𝔽, or *+∞*_𝔽, return _n_.
3. If _n_ is *-∞*_𝔽, return *-1*_𝔽.
4. Let _exp_ be the exponential function of ℝ(_n_).
5. Return an implementation-approximated Number value representing _exp_ - 1.

--- referenced: ToNumber ( _arg_: an ECMAScript language value, ): either a normal completion containing a Number or a throw completion
  id: sec-tonumber

description: It converts _arg_ to a value of type Number.

1. If _arg_ is a Number, return _arg_.
2. If _arg_ is either a Symbol or a BigInt, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *NaN*.
4. If _arg_ is either *null* or *false*, return *+0*_𝔽.
5. If _arg_ is *true*, return *1*_𝔽.
6. If _arg_ is a String, return StringToNumber(_arg_).
7. Assert: _arg_ is an Object.
8. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~number~).
9. Assert: _primitiveValue_ is not an Object.
10. Return ? ToNumber(_primitiveValue_).

[subclause sec-tonumber-applied-to-the-string-type: ToNumber Applied to the String Type]


## Done when

- Every test262 run under built-ins/Math/expm1/ passes (check with `./t262 built-ins/Math/expm1/`). 8 pass on the main branch right now.

