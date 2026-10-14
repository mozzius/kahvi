# Ticket B-string-prototype-pad-end: String.prototype.padEnd

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

- `crates/kahvi-builtins/src/string/prototype_pad_end.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.padEnd ( _maxLength_ [ , _fillString_ ] )
  id: sec-string.prototype.padend   type: built-in function

This method performs the following steps when called:

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. Return ? StringPaddingBuiltinsImpl(_thisValue_, _maxLength_, _fillString_, ~end~).

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

--- referenced: StringPaddingBuiltinsImpl ( _thisValue_: an ECMAScript language value, _maxLength_: an ECMAScript language value, _fillString_: an ECMAScript language value, _placement_: ~start~ or ~end~, ): either a normal completion containing a String or a throw completion
  id: sec-stringpaddingbuiltinsimpl

1. Let _string_ be ? ToString(_thisValue_).
2. Let _intMaxLength_ be ℝ(? ToLength(_maxLength_)).
3. Let _stringLength_ be the length of _string_.
4. If _intMaxLength_ ≤ _stringLength_, return _string_.
5. If _fillString_ is *undefined*, set _fillString_ to the String value consisting solely of the code unit 0x0020 (SPACE).
6. Else, set _fillString_ to ? ToString(_fillString_).
7. Return StringPad(_string_, _intMaxLength_, _fillString_, _placement_).


## Done when

- Every test262 run under built-ins/String/prototype/padEnd/ passes (check with `./t262 built-ins/String/prototype/padEnd/`). 8 pass on the main branch right now.

## Previous attempts

- attempt 310: worker reported BLOCKED but changed files: none
- attempt 310: rejected by gates: lint: lint suppression is architect-only at crates/kahvi-builtins/src/string/prototype_pad_end.rs:36: #[allow(dead_code)]

