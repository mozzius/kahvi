# Ticket B-date-prototype-to-string: Date.prototype.toString

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

- `crates/kahvi-builtins/src/date/prototype_to_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.toString ( )
  id: sec-date.prototype.tostring   type: built-in function

This method performs the following steps when called:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _tv_ be _dateObj_.[[DateValue]].
4. Return ToDateString(_tv_).

NOTE:
For any Date `d` such that `d.[[DateValue]]` is evenly divisible by 1000, the result of `Date.parse(d.toString())` = `d.valueOf()`. See [sec-date.parse].
NOTE:
This method is not generic; it throws a *TypeError* exception if its *this* value is not a Date. Therefore, it cannot be transferred to other kinds of objects for use as a method.
[subclause sec-timestring: TimeString (
_tv_: a Number, but not *NaN*,
): a String]
[subclause sec-datestring: DateString (
_tv_: a Number, but not *NaN*,
): a String]
[subclause sec-timezoneestring: TimeZoneString (
_tv_: an integral Number,
): a String]
[subclause sec-todatestring: ToDateString (
_tv_: an integral Number or *NaN*,
): a String]

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: ToDateString ( _tv_: an integral Number or *NaN*, ): a String
  id: sec-todatestring

1. If _tv_ is *NaN*, return *"Invalid Date"*.
2. Let _localTime_ be LocalTime(_tv_).
3. Return the string-concatenation of DateString(_localTime_), the code unit 0x0020 (SPACE), TimeString(_localTime_), and TimeZoneString(_tv_).


## Done when

- Every test262 run under built-ins/Date/prototype/toString/ passes (check with `./t262 built-ins/Date/prototype/toString/`). 8 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 339: worker reported BLOCKED but changed files: `crates/kahvi-builtins/src/support/date.rs` `year_string` must pad negative years to 4 digits (`{:04}`), not 6 (`{:06}`), per DateString's ToZeroPaddedDecimalString(abs(yv), 4)
- attempt 339: not merged (contradicts_spec); continue from this diff. The main file is spec-shaped and can be kept. The remaining failure, built-ins/Date/prototype/toString/negative-year.js, comes from the shared date helper, not from prototype_to_string.rs. The helper that produces the DateString year (support::date, reached through to_date_string) pads negative years to six digits. Per spec DateString, the year should be yearSign plus ToZeroPaddedDecimalString(abs(yv), 4), so year -1 should give "-0001". That fix belongs in the helper or in a separate architect-owned ticket, and it must not be special-cased in this file. Also check that this_time_value throws a TypeError for non-Date receivers (RequireInternalSlot steps 1 and 2), since steps 2 and 3 are handled in that one call.

