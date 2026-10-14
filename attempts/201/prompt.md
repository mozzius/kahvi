# Ticket B-string-prototype-trim-end: String.prototype.trimEnd

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

- `crates/kahvi-builtins/src/string/prototype_trim_end.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.trimEnd ( )
  id: sec-string.prototype.trimend   type: built-in function

This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
It performs the following steps when called:

1. Let _string_ be the *this* value.
2. Return ? TrimString(_string_, ~end~).

NOTE:
This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.

--- referenced: TrimString ( _arg_: an ECMAScript language value, _where_: ~start~, ~end~, or ~start+end~, ): either a normal completion containing a String or a throw completion
  id: sec-trimstring

description: It interprets _arg_ as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].

1. Perform ? RequireObjectCoercible(_arg_).
2. Let _string_ be ? ToString(_arg_).
3. If _where_ is ~start~, then
   a. Let _trimmedString_ be the String value that is a copy of _string_ with leading white space removed.
4. Else if _where_ is ~end~, then
   a. Let _trimmedString_ be the String value that is a copy of _string_ with trailing white space removed.
5. Else,
   a. Assert: _where_ is ~start+end~.
   b. Let _trimmedString_ be the String value that is a copy of _string_ with both leading and trailing white space removed.
6. Return _trimmedString_.

The definition of white space is the union of |WhiteSpace| and |LineTerminator|. When determining whether a Unicode code point is in Unicode general category “Space_Separator” (“Zs”), code unit sequences are interpreted as UTF-16 encoded code point sequences as specified in [sec-ecmascript-language-types-string-type].


## Done when

- Every test262 run under built-ins/String/prototype/trimEnd/ passes (check with `./t262 built-ins/String/prototype/trimEnd/`). 8 pass on the main branch right now.

