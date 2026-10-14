# Ticket B-date-prototype-to-utc-string: Date.prototype.toUTCString

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

- `crates/kahvi-builtins/src/date/prototype_to_utc_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.toUTCString ( )
  id: sec-date.prototype.toutcstring   type: built-in function

This method returns a String value representing the instant in time corresponding to the *this* value. The format of the String is based upon HTTP-date from RFC 7231, generalized to support the full range of times supported by ECMAScript Dates.
It performs the following steps when called:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _tv_ be _dateObj_.[[DateValue]].
4. If _tv_ is *NaN*, return *"Invalid Date"*.
5. Let _weekday_ be the Name of the entry in [sec-todatestring-day-names] whose WeekDay Index = WeekDay(_tv_).
6. Let _month_ be the Name of the entry in [sec-todatestring-month-names] whose Month Index = MonthFromTime(_tv_).
7. Let _day_ be ToZeroPaddedDecimalString(DateFromTime(_tv_), 2).
8. Let _yv_ be YearFromTime(_tv_).
9. If _yv_ ≥ 0, let _yearSign_ be the empty String; else let _yearSign_ be *"-"*.
10. Let _paddedYear_ be ToZeroPaddedDecimalString(abs(_yv_), 4).
11. Return the string-concatenation of _weekday_, *","*, the code unit 0x0020 (SPACE), _day_, the code unit 0x0020 (SPACE), _month_, the code unit 0x0020 (SPACE), _yearSign_, _paddedYear_, the code unit 0x0020 (SPACE), and TimeString(_tv_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: WeekDay ( _tv_: a finite time value, ): an integer in the inclusive interval from 0 to 6
  id: sec-weekday

description: It returns an integer identifying the day of the week in which _tv_ falls. A weekday value of 0 specifies Sunday; 1 specifies Monday; 2 specifies Tuesday; 3 specifies Wednesday; 4 specifies Thursday; 5 specifies Friday; and 6 specifies Saturday. Note that WeekDay(*+0*_𝔽) = 4, corresponding to Thursday, 1 January 1970.

1. Return (Day(_tv_) + 4) modulo 7.

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

--- referenced: ToZeroPaddedDecimalString ( _n_: a non-negative integer, _minLength_: a non-negative integer, ): a String
  id: sec-tozeropaddeddecimalstring

1. Let _string_ be the String representation of _n_, formatted as a decimal number.
2. Return StringPad(_string_, _minLength_, *"0"*, ~start~).

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

--- referenced: YearFromTime ( _tv_: a finite time value, ): an integer
  id: sec-yearfromtime

description: It returns the year in which _tv_ falls.

1. Return the largest integer _y_ (closest to +∞) such that TimeFromYear(_y_) ≤ _tv_.

--- referenced: TimeString ( _tv_: a Number, but not *NaN*, ): a String
  id: sec-timestring

1. Let _hour_ be ToZeroPaddedDecimalString(HourFromTime(_tv_), 2).
2. Let _minute_ be ToZeroPaddedDecimalString(MinuteFromTime(_tv_), 2).
3. Let _second_ be ToZeroPaddedDecimalString(SecondFromTime(_tv_), 2).
4. Return the string-concatenation of _hour_, *":"*, _minute_, *":"*, _second_, the code unit 0x0020 (SPACE), and *"GMT"*.


## Done when

- Every test262 run under built-ins/Date/prototype/toUTCString/ passes (check with `./t262 built-ins/Date/prototype/toUTCString/`). 8 pass on the main branch right now.

