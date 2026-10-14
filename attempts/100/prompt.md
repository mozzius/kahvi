# Ticket B-reg-exp-prototype-get-has-indices: get RegExp.prototype.hasIndices

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

- `crates/kahvi-builtins/src/reg_exp/prototype_get_has_indices.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

get RegExp.prototype.hasIndices
  id: sec-get-regexp.prototype.hasIndices   type: built-in function

`RegExp.prototype.hasIndices` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:

1. Let _regexp_ be the *this* value.
2. Let _codeUnit_ be the code unit 0x0064 (LATIN SMALL LETTER D).
3. Return ? RegExpHasFlag(_regexp_, _codeUnit_).

--- referenced: RegExpHasFlag ( _regexp_: an ECMAScript language value, _codeUnit_: a code unit, ): either a normal completion containing either a Boolean or *undefined*, or a throw completion
  id: sec-regexphasflag

1. If _regexp_ is not an Object, throw a *TypeError* exception.
2. If _regexp_ does not have an [[OriginalFlags]] internal slot, then
   a. If SameValue(_regexp_, %RegExp.prototype%) is *true*, return *undefined*.
   b. Throw a *TypeError* exception.
3. Let _flags_ be _regexp_.[[OriginalFlags]].
4. If _flags_ contains _codeUnit_, return *true*.
5. Return *false*.


## Done when

- Every test262 run under built-ins/RegExp/prototype/hasIndices/ passes (check with `./t262 built-ins/RegExp/prototype/hasIndices/`). 6 pass on the main branch right now.

