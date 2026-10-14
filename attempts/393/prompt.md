# Ticket B-date-prototype-get-date: Date.prototype.getDate

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

- `crates/kahvi-builtins/src/date/prototype_get_date.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.getDate ( )
  id: sec-date.prototype.getdate   type: built-in function

This method performs the following steps when called:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _tv_ be _dateObj_.[[DateValue]].
4. If _tv_ is *NaN*, return *NaN*.
5. Return 𝔽(DateFromTime(LocalTime(_tv_))).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: DateFromTime ( _tv_: a finite time value, ): an integer in the inclusive interval from 1 to 31
  id: sec-datefromtime

description: It returns the day of the month in which _tv_ falls.

1. Let _inLeapYear_ be InLeapYear(_tv_).
2. Let _dayWithinYear_ be DayWithinYear(_tv_).
3. Let _month_ be MonthFromTime(_tv_).
4. If _month_ = 0, return _dayWithinYear_ + 1.
5. If _month_ = 1, return _dayWithinYear_ - 30.
6. If _month_ = 2, return _dayWithinYear_ - 58 - _inLeapYear_.
7. If _month_ = 3, return _dayWithinYear_ - 89 - _inLeapYear_.
8. If _month_ = 4, return _dayWithinYear_ - 119 - _inLeapYear_.
9. If _month_ = 5, return _dayWithinYear_ - 150 - _inLeapYear_.
10. If _month_ = 6, return _dayWithinYear_ - 180 - _inLeapYear_.
11. If _month_ = 7, return _dayWithinYear_ - 211 - _inLeapYear_.
12. If _month_ = 8, return _dayWithinYear_ - 242 - _inLeapYear_.
13. If _month_ = 9, return _dayWithinYear_ - 272 - _inLeapYear_.
14. If _month_ = 10, return _dayWithinYear_ - 303 - _inLeapYear_.
15. Assert: _month_ = 11.
16. Return _dayWithinYear_ - 333 - _inLeapYear_.


## Done when

- Every test262 run under built-ins/Date/prototype/getDate/ passes (check with `./t262 built-ins/Date/prototype/getDate/`). 8 pass on the main branch right now.

