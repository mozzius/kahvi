# Ticket B-date-prototype-to-locale-date-string: Date.prototype.toLocaleDateString

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

- `crates/kahvi-builtins/src/date/prototype_to_locale_date_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.toLocaleDateString ( [ _reserved1_ [ , _reserved2_ ] ] )
  id: sec-date.prototype.tolocaledatestring   type: built-in function

An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used:
This method returns a String value. The contents of the String are implementation-defined, but are intended to represent the “date” portion of the Date in the current time zone in a convenient, human-readable form that corresponds to the conventions of the host environment's current locale.
The meaning of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.


## Done when

- Every test262 run under built-ins/Date/prototype/toLocaleDateString/ passes (check with `./t262 built-ins/Date/prototype/toLocaleDateString/`). 8 pass on the main branch right now.

