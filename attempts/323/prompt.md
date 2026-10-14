# Ticket B-string-prototype-at: String.prototype.at

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

- `crates/kahvi-builtins/src/string/prototype_at.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.at ( _index_ )
  id: sec-string.prototype.at   type: built-in function

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. Let _string_ be ? ToString(_thisValue_).
4. Let _length_ be the length of _string_.
5. Let _k_ be ? ToAbsoluteIndex(_index_, _length_).
6. If _k_ < 0 or _k_ ≥ _length_, return *undefined*.
7. Return the substring of _string_ from _k_ to _k_ + 1.

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

--- referenced: ToString ( _arg_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-tostring

description: It converts _arg_ to a value of type String.

1. If _arg_ is a String, return _arg_.
2. If _arg_ is a Symbol, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *"undefined"*.
4. If _arg_ is *null*, return *"null"*.
5. If _arg_ is *true*, return *"true"*.
6. If _arg_ is *false*, return *"false"*.
7. If _arg_ is a Number, return Number::toString(_arg_, 10).
8. If _arg_ is a BigInt, return BigInt::toString(_arg_, 10).
9. Assert: _arg_ is an Object.
10. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~string~).
11. Assert: _primitiveValue_ is not an Object.
12. Return ? ToString(_primitiveValue_).

--- referenced: ToAbsoluteIndex ( _value_: an ECMAScript language value, _length_: a non-negative integer, ): either a normal completion containing either an integer, +∞, or -∞, or a throw completion
  id: sec-toabsoluteindex

description: It converts _value_ to an integer, +∞, or -∞, interpreting negative values relative to _length_.

1. Let _int_ be ? ToIntegerOrInfinity(_value_).
2. If _int_ is finite and _int_ < 0, set _int_ to _length_ + _int_.
3. Return _int_.


## Done when

- Every test262 run under built-ins/String/prototype/at/ passes (check with `./t262 built-ins/String/prototype/at/`). 6 pass on the main branch right now.

