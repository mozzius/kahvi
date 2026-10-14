# Ticket B-date-prototype-get-utc-month: Date.prototype.getUTCMonth

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

- `crates/kahvi-builtins/src/date/prototype_get_utc_month.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.getUTCMonth ( )
  id: sec-date.prototype.getutcmonth   type: built-in function

This method performs the following steps when called:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _tv_ be _dateObj_.[[DateValue]].
4. If _tv_ is *NaN*, return *NaN*.
5. Return 𝔽(MonthFromTime(_tv_)).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: MonthFromTime ( _tv_: a finite time value, ): an integer in the inclusive interval from 0 to 11
  id: sec-monthfromtime

description: It returns an integer identifying the month in which _tv_ falls. A month value of 0 specifies January; 1 specifies February; 2 specifies March; 3 specifies April; 4 specifies May; 5 specifies June; 6 specifies July; 7 specifies August; 8 specifies September; 9 specifies October; 10 specifies November; and 11 specifies December. Note that MonthFromTime(*+0*_𝔽) = 0, corresponding to Thursday, 1 January 1970.

1. Let _inLeapYear_ be InLeapYear(_tv_).
2. Let _dayWithinYear_ be DayWithinYear(_tv_).
3. If _dayWithinYear_ < 31, return 0.
4. If _dayWithinYear_ < 59 + _inLeapYear_, return 1.
5. If _dayWithinYear_ < 90 + _inLeapYear_, return 2.
6. If _dayWithinYear_ < 120 + _inLeapYear_, return 3.
7. If _dayWithinYear_ < 151 + _inLeapYear_, return 4.
8. If _dayWithinYear_ < 181 + _inLeapYear_, return 5.
9. If _dayWithinYear_ < 212 + _inLeapYear_, return 6.
10. If _dayWithinYear_ < 243 + _inLeapYear_, return 7.
11. If _dayWithinYear_ < 273 + _inLeapYear_, return 8.
12. If _dayWithinYear_ < 304 + _inLeapYear_, return 9.
13. If _dayWithinYear_ < 334 + _inLeapYear_, return 10.
14. Assert: _dayWithinYear_ < 365 + _inLeapYear_.
15. Return 11.


## Done when

- Every test262 run under built-ins/Date/prototype/getUTCMonth/ passes (check with `./t262 built-ins/Date/prototype/getUTCMonth/`). 8 pass on the main branch right now.

