# Ticket B-math-atan2: Math.atan2

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

- `crates/kahvi-builtins/src/math/atan2.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.atan2 ( _y_, _x_ )
  id: sec-math.atan2   type: built-in function

This function returns the inverse tangent of the quotient _y_ / _x_ of the arguments _y_ and _x_, where the signs of _y_ and _x_ are used to determine the quadrant of the result. Note that it is intentional and traditional for the two-argument inverse tangent function that the argument named _y_ be first and the argument named _x_ be second. The result is expressed in radians and is in the inclusive interval from -π to +π.
It performs the following steps when called:

1. Let _ny_ be ? ToNumber(_y_).
2. Let _nx_ be ? ToNumber(_x_).
3. If _ny_ is *NaN* or _nx_ is *NaN*, return *NaN*.
4. If _ny_ is *+∞*_𝔽, then
   a. If _nx_ is *+∞*_𝔽, return an implementation-approximated Number value representing π / 4.
   b. If _nx_ is *-∞*_𝔽, return an implementation-approximated Number value representing 3π / 4.
   c. Return an implementation-approximated Number value representing π / 2.
5. If _ny_ is *-∞*_𝔽, then
   a. If _nx_ is *+∞*_𝔽, return an implementation-approximated Number value representing -π / 4.
   b. If _nx_ is *-∞*_𝔽, return an implementation-approximated Number value representing -3π / 4.
   c. Return an implementation-approximated Number value representing -π / 2.
6. If _ny_ is *+0*_𝔽, then
   a. If _nx_ > *+0*_𝔽 or _nx_ is *+0*_𝔽, return *+0*_𝔽.
   b. Return an implementation-approximated Number value representing π.
7. If _ny_ is *-0*_𝔽, then
   a. If _nx_ > *+0*_𝔽 or _nx_ is *+0*_𝔽, return *-0*_𝔽.
   b. Return an implementation-approximated Number value representing -π.
8. Assert: _ny_ is a finite Number and is neither *+0*_𝔽 nor *-0*_𝔽.
9. If _ny_ > *+0*_𝔽, then
   a. If _nx_ is *+∞*_𝔽, return *+0*_𝔽.
   b. If _nx_ is *-∞*_𝔽, return an implementation-approximated Number value representing π.
   c. If _nx_ is either *+0*_𝔽 or *-0*_𝔽, return an implementation-approximated Number value representing π / 2.
10. If _ny_ < *-0*_𝔽, then
   a. If _nx_ is *+∞*_𝔽, return *-0*_𝔽.
   b. If _nx_ is *-∞*_𝔽, return an implementation-approximated Number value representing -π.
   c. If _nx_ is either *+0*_𝔽 or *-0*_𝔽, return an implementation-approximated Number value representing -π / 2.
11. Assert: _nx_ is a finite Number and is neither *+0*_𝔽 nor *-0*_𝔽.
12. Let _result_ be the inverse tangent of abs(ℝ(_ny_) / ℝ(_nx_)).
13. If _nx_ < *-0*_𝔽, then
   a. If _ny_ > *+0*_𝔽, set _result_ to π - _result_.
   b. Else, set _result_ to -π + _result_.
14. Else,
   a. If _ny_ < *-0*_𝔽, set _result_ to -_result_.
15. Return an implementation-approximated Number value representing _result_.

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

- Every test262 run under built-ins/Math/atan2/ passes (check with `./t262 built-ins/Math/atan2/`). 8 pass on the main branch right now.

