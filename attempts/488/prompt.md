# Ticket B-number-prototype-to-locale-string: Number.prototype.toLocaleString

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

- `crates/kahvi-builtins/src/number/prototype_to_locale_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Number.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
  id: sec-number.prototype.tolocalestring   type: built-in function

An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used:
This method produces a String value that represents this Number value formatted according to the conventions of the host environment's current locale. This method is implementation-defined, and it is permissible, but not encouraged, for it to return the same thing as `toString`.
The meanings of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.


## Done when

- Every test262 run under built-ins/Number/prototype/toLocaleString/ passes (check with `./t262 built-ins/Number/prototype/toLocaleString/`). 8 pass on the main branch right now.

## Previous attempts

- attempt 486: worker reported BLOCKED but changed files: none
- attempt 486: rejected by gates: no new test262 passes in target

