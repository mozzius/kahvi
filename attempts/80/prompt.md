# Ticket B-reg-exp-prototype-get-source: get RegExp.prototype.source

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

- `crates/kahvi-builtins/src/reg_exp/prototype_get_source.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

get RegExp.prototype.source
  id: sec-get-regexp.prototype.source   type: built-in function

`RegExp.prototype.source` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:

1. Let _regexp_ be the *this* value.
2. If _regexp_ is not an Object, throw a *TypeError* exception.
3. If _regexp_ does not have an [[OriginalSource]] internal slot, then
   a. If SameValue(_regexp_, %RegExp.prototype%) is *true*, return *"(?:)"*.
   b. Throw a *TypeError* exception.
4. Assert: _regexp_ has an [[OriginalFlags]] internal slot.
5. Let _source_ be _regexp_.[[OriginalSource]].
6. Let _flags_ be _regexp_.[[OriginalFlags]].
7. Return EscapeRegExpPattern(_source_, _flags_).

[subclause sec-escaperegexppattern: EscapeRegExpPattern (
_pattern_: a String,
_flags_: a String,
): a String]

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

- Every test262 run under built-ins/RegExp/prototype/source/ passes (check with `./t262 built-ins/RegExp/prototype/source/`). 6 pass on the main branch right now.

