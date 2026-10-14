# Ticket B-math-min: Math.min

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

- `crates/kahvi-builtins/src/math/min.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.min ( ..._args_ )
  id: sec-math.min   type: built-in function

Given zero or more arguments, this function calls ToNumber on each of the arguments and returns the smallest of the resulting values.
It performs the following steps when called:

1. Let _coerced_ be a new empty List.
2. For each element _arg_ of _args_, do
   a. Let _n_ be ? ToNumber(_arg_).
   b. Append _n_ to _coerced_.
3. Let _lowest_ be *+∞*_𝔽.
4. For each element _number_ of _coerced_, do
   a. If _number_ is *NaN*, return *NaN*.
   b. If _number_ is *-0*_𝔽 and _lowest_ is *+0*_𝔽, set _lowest_ to *-0*_𝔽.
   c. If _number_ < _lowest_, set _lowest_ to _number_.
5. Return _lowest_.

NOTE:
The comparison of values to determine the smallest value is done using the IsLessThan algorithm except that *+0*_𝔽 is considered to be larger than *-0*_𝔽.
The *"length"* property of this function is *2*_𝔽.

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

- Every test262 run under built-ins/Math/min/ passes (check with `./t262 built-ins/Math/min/`). 10 pass on the main branch right now.

