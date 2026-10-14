# Ticket B-string-prototype-sup: String.prototype.sup

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

- `crates/kahvi-builtins/src/string/prototype_sup.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.sup ( )
  id: sec-string.prototype.sup   type: built-in function

This method performs the following steps when called:

1. Let _string_ be the *this* value.
2. Return ? CreateHTML(_string_, *"sup"*, *""*, *""*).

--- referenced: CreateHTML ( _contents_: an ECMAScript language value, _tag_: a String, _attr_: a String, _attrValue_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-createhtml

1. Perform ? RequireObjectCoercible(_contents_).
2. Let _contentsString_ be ? ToString(_contents_).
3. Let _part1_ be the string-concatenation of *"<"* and _tag_.
4. If _attr_ is not the empty String, then
   a. Let _attrValueString_ be ? ToString(_attrValue_).
   b. Let _escapedAttrValue_ be the String value that is the same as _attrValueString_ except that each occurrence of the code unit 0x0022 (QUOTATION MARK) in _attrValueString_ has been replaced with the six code unit sequence *"&quot;"*.
   c. Set _part1_ to the string-concatenation of:
      • _part1_
      • the code unit 0x0020 (SPACE)
      • _attr_
      • the code unit 0x003D (EQUALS SIGN)
      • the code unit 0x0022 (QUOTATION MARK)
      • _escapedAttrValue_
      • the code unit 0x0022 (QUOTATION MARK)
5. Let _part2_ be the string-concatenation of _part1_ and *">"*.
6. Let _part3_ be the string-concatenation of _part2_ and _contentsString_.
7. Let _part4_ be the string-concatenation of _part3_, *"</"*, _tag_, and *">"*.
8. Return _part4_.


## Done when

- Every test262 run under annexB/built-ins/String/prototype/sup/ passes (check with `./t262 annexB/built-ins/String/prototype/sup/`). 8 pass on the main branch right now.

