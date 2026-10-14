# Ticket B-math-round: Math.round

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

- `crates/kahvi-builtins/src/math/round.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.round ( _x_ )
  id: sec-math.round   type: built-in function

This function returns the Number value that is closest to _x_ and is integral. If two integral Numbers are equally close to _x_, then the result is the Number value that is closer to +∞. If _x_ is already integral, the result is _x_.
It performs the following steps when called:

1. Let _n_ be ? ToNumber(_x_).
2. If _n_ is not a finite Number or _n_ is an integral Number, return _n_.
3. If _n_ < *0.5*_𝔽 and _n_ > *+0*_𝔽, return *+0*_𝔽.
4. If _n_ < *-0*_𝔽 and _n_ ≥ *-0.5*_𝔽, return *-0*_𝔽.
5. Return the integral Number closest to _n_, preferring the Number closer to +∞ in the case of a tie.

NOTE:
`Math.round(3.5)` returns 4, but `Math.round(-3.5)` returns -3.
NOTE:
The value of `Math.round(x)` is not always the same as the value of `Math.floor(x + 0.5)`. When `x` is *-0*_𝔽 or `x` is less than *-0*_𝔽 but greater than or equal to *-0.5*_𝔽, `Math.round(x)` returns *-0*_𝔽, but `Math.floor(x + 0.5)` returns *+0*_𝔽. `Math.round(x)` may also differ from the value of `Math.floor(x + 0.5)`because of internal rounding when computing `x + 0.5`.

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

- Every test262 run under built-ins/Math/round/ passes (check with `./t262 built-ins/Math/round/`). 8 pass on the main branch right now.

