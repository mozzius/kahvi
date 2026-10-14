# Ticket B-math-f16round: Math.f16round

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

- `crates/kahvi-builtins/src/math/f16round.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.f16round ( _x_ )
  id: sec-math.f16round   type: built-in function

This function performs the following steps when called:

1. Let _n_ be ? ToNumber(_x_).
2. If _n_ is *NaN*, return *NaN*.
3. If _n_ is one of *+0*_𝔽, *-0*_𝔽, *+∞*_𝔽, or *-∞*_𝔽, return _n_.
4. Let _n16_ be the result of converting _n_ to IEEE 754-2019 binary16 format using roundTiesToEven mode.
5. Let _n64_ be the result of converting _n16_ to IEEE 754-2019 binary64 format.
6. Return the ECMAScript Number value corresponding to _n64_.

NOTE:
This operation is not the same as casting to binary32 and then to binary16 because of the possibility of double-rounding: consider the number _k_ = *1.00048828125000022204*_𝔽, for example, for which Math.f16round(_k_) is *1.0009765625*_𝔽, but Math.f16round(Math.fround(_k_)) is *1*_𝔽.
Not all platforms provide native support for casting from binary64 to binary16. There are various libraries which can provide this, including the MIT-licensed half library. Alternatively, it is possible to first cast from binary64 to binary32 under roundTiesToEven and then check whether the result could lead to incorrect double-rounding. Such cases can be handled explicitly by adjusting the mantissa of the binary32 value so that it is the value which would be produced by performing the initial cast under roundTiesToOdd. Casting the adjusted value to binary16 under roundTiesToEven then produces the correct value.

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

- Every test262 run under built-ins/Math/f16round/ passes (check with `./t262 built-ins/Math/f16round/`). 8 pass on the main branch right now.

