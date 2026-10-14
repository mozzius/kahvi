# Ticket B-string-prototype-to-locale-upper-case: String.prototype.toLocaleUpperCase

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

- `crates/kahvi-builtins/src/string/prototype_to_locale_upper_case.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.toLocaleUpperCase ( [ _reserved1_ [ , _reserved2_ ] ] )
  id: sec-string.prototype.tolocaleuppercase   type: built-in function

An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used:
This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
It works exactly the same as `toUpperCase` except that it is intended to yield a locale-sensitive result corresponding with conventions of the host environment's current locale. There will only be a difference in the few cases (such as Turkish) where the rules for that language conflict with the regular Unicode case mappings.
The meaning of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.
NOTE:
This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.


## Done when

- Every test262 run under built-ins/String/prototype/toLocaleUpperCase/ passes (check with `./t262 built-ins/String/prototype/toLocaleUpperCase/`). 16 pass on the main branch right now.

