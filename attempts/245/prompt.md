# Ticket B-reg-exp-escape: RegExp.escape

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

- `crates/kahvi-builtins/src/reg_exp/escape.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

RegExp.escape ( _string_ )
  id: sec-regexp.escape   type: built-in function

This function returns a copy of _string_ in which characters that are potentially special in a regular expression |Pattern| have been replaced by equivalent escape sequences.
It performs the following steps when called:

1. If _string_ is not a String, throw a *TypeError* exception.
2. Let _escaped_ be the empty String.
3. Let _codePointList_ be StringToCodePoints(_string_).
4. For each code point _codePoint_ of _codePointList_, do
   a. If _escaped_ is the empty String and _codePoint_ is matched by either |DecimalDigit| or |AsciiLetter|, then
      i. NOTE: Escaping a leading digit ensures that output corresponds with pattern text which may be used after a `\0` character escape or a |DecimalEscape| such as `\1` and still match _string_ rather than be interpreted as an extension of the preceding escape sequence. Escaping a leading ASCII letter does the same for the context after `\c`.
      ii. Let _numericValue_ be the numeric value of _codePoint_.
      iii. Let _hex_ be Number::toString(𝔽(_numericValue_), 16).
      iv. Assert: The length of _hex_ is 2.
      v. Set _escaped_ to the string-concatenation of the code unit 0x005C (REVERSE SOLIDUS), *"x"*, and _hex_.
   b. Else,
      i. Set _escaped_ to the string-concatenation of _escaped_ and EncodeForRegExpEscape(_codePoint_).
5. Return _escaped_.

NOTE:
Despite having similar names, EscapeRegExpPattern and `RegExp.escape` do not perform similar actions. The former escapes a pattern for representation as a string, while this function escapes a string for representation inside a pattern.
[subclause sec-encodeforregexpescape: EncodeForRegExpEscape (
_codePoint_: a code point,
): a String]


## Done when

- Every test262 run under built-ins/RegExp/escape/ passes (check with `./t262 built-ins/RegExp/escape/`). 10 pass on the main branch right now.

