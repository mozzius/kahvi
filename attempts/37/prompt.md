# Ticket B-object-prototype-is-prototype-of: Object.prototype.isPrototypeOf

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

- `crates/kahvi-builtins/src/object/prototype_is_prototype_of.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.prototype.isPrototypeOf ( _value_ )
  id: sec-object.prototype.isprototypeof   type: built-in function

This method performs the following steps when called:

1. If _value_ is not an Object, return *false*.
2. Let _obj_ be ? ToObject(*this* value).
3. Repeat,
   a. Set _value_ to ? _value_.[[GetPrototypeOf]]().
   b. If _value_ is *null*, return *false*.
   c. If SameValue(_obj_, _value_) is *true*, return *true*.

NOTE:
The ordering of steps [step-isprototypeof-check-object] and [step-isprototypeof-toobject] preserves the behaviour specified by previous editions of this specification for the case where _value_ is not an object and the *this* value is *undefined* or *null*.

--- referenced: ToObject ( _arg_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-toobject

description: It converts _arg_ to a value of type Object.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. If _arg_ is a Boolean, return a new Boolean object whose [[BooleanData]] internal slot is set to _arg_. See [sec-boolean-objects] for a description of Boolean objects.
3. If _arg_ is a Number, return a new Number object whose [[NumberData]] internal slot is set to _arg_. See [sec-number-objects] for a description of Number objects.
4. If _arg_ is a String, return a new String object whose [[StringData]] internal slot is set to _arg_. See [sec-string-objects] for a description of String objects.
5. If _arg_ is a Symbol, return a new Symbol object whose [[SymbolData]] internal slot is set to _arg_. See [sec-symbol-objects] for a description of Symbol objects.
6. If _arg_ is a BigInt, return a new BigInt object whose [[BigIntData]] internal slot is set to _arg_. See [sec-bigint-objects] for a description of BigInt objects.
7. Assert: _arg_ is an Object.
8. Return _arg_.

--- referenced: SameValue ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, ): a Boolean
  id: sec-samevalue

description: It determines whether or not the two arguments are the same value.

1. If SameType(_x_, _y_) is *false*, return *false*.
2. If _x_ is a Number, then
   a. Return Number::sameValue(_x_, _y_).
3. Return SameValueNonNumber(_x_, _y_).

NOTE:
This algorithm differs from the IsStrictlyEqual Algorithm by treating all *NaN* values as equivalent and by differentiating *+0*_𝔽 from *-0*_𝔽.


## Done when

- Every test262 run under built-ins/Object/prototype/isPrototypeOf/ passes (check with `./t262 built-ins/Object/prototype/isPrototypeOf/`). 8 pass on the main branch right now.

