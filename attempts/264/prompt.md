# Ticket B-date-prototype-set-month: Date.prototype.setMonth

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

- `crates/kahvi-builtins/src/date/prototype_set_month.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.setMonth ( _month_ [ , _day_ ] )
  id: sec-date.prototype.setmonth   type: built-in function

This method performs the following steps when called:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _tv_ be _dateObj_.[[DateValue]].
4. Let _monthNumber_ be ? ToNumber(_month_).
5. If _day_ is present, let _dayNumber_ be ? ToNumber(_day_).
6. If _tv_ is *NaN*, return *NaN*.
7. Set _tv_ to LocalTime(_tv_).
8. If _day_ is not present, let _dayNumber_ be 𝔽(DateFromTime(_tv_)).
9. Let _newDate_ be MakeDate(MakeDay(𝔽(YearFromTime(_tv_)), _monthNumber_, _dayNumber_), 𝔽(TimeWithinDay(_tv_))).
10. Let _u_ be TimeClip(UTC(_newDate_)).
11. Set _dateObj_.[[DateValue]] to _u_.
12. Return _u_.

The *"length"* property of this method is *2*_𝔽.
NOTE:
If _day_ is not present, this method behaves as if _day_ was present with the value `getDate()`.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: ToNumber ( _arg_: an ECMAScript language value, ): either a normal completion containing a Number or a throw completion
  id: sec-tonumber

description: It converts _arg_ to a value of type Number.

1. If _arg_ is a Number, return _arg_.
2. If _arg_ is either a Symbol or a BigInt, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *NaN*.
4. If _arg_ is either *null* or *false*, return *+0*_𝔽.
5. If _arg_ is *true*, return *1*_𝔽.
6. If _arg_ is a String, return StringToNumber(_arg_).
7. Assert: _arg_ is an Object.
8. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~number~).
9. Assert: _primitiveValue_ is not an Object.
10. Return ? ToNumber(_primitiveValue_).

[subclause sec-tonumber-applied-to-the-string-type: ToNumber Applied to the String Type]

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

--- referenced: MakeDate ( _day_: a Number, _time_: a Number, ): a finite Number or *NaN*
  id: sec-makedate

description: It calculates a number of milliseconds.

1. If _day_ is not a finite Number or _time_ is not a finite Number, return *NaN*.
2. Let _tv_ be _day_ × 𝔽(MillisecondsPerDay) + _time_.
3. If _tv_ is not a finite Number, return *NaN*.
4. Return _tv_.

--- referenced: MakeDay ( _year_: a Number, _month_: a Number, _day_: a Number, ): a finite Number or *NaN*
  id: sec-makeday

description: It calculates a number of days.

1. If _year_ is not a finite Number, _month_ is not a finite Number, or _day_ is not a finite Number, return *NaN*.
2. Let _yearMV_ be ! ToIntegerOrInfinity(_year_).
3. Let _monthMV_ be ! ToIntegerOrInfinity(_month_).
4. Let _dayMV_ be ! ToIntegerOrInfinity(_day_).
5. Let _balancedYear_ be 𝔽(_yearMV_) + 𝔽(floor(_monthMV_ / 12)).
6. If _balancedYear_ is not a finite Number, return *NaN*.
7. Let _balancedMonthMV_ be _monthMV_ modulo 12.
8. Find a finite time value _tv_ such that YearFromTime(_tv_) = ℝ(_balancedYear_), MonthFromTime(_tv_) = _balancedMonthMV_, and DateFromTime(_tv_) = 1; but if this is not possible (because some argument is out of range), return *NaN*.
9. Return 𝔽(Day(_tv_)) + 𝔽(_dayMV_) - *1*_𝔽.

--- referenced: YearFromTime ( _tv_: a finite time value, ): an integer
  id: sec-yearfromtime

description: It returns the year in which _tv_ falls.

1. Return the largest integer _y_ (closest to +∞) such that TimeFromYear(_y_) ≤ _tv_.

--- referenced: TimeWithinDay ( _tv_: a finite time value, ): an integer in the interval from 0 (inclusive) to MillisecondsPerDay (exclusive)
  id: sec-timewithinday

description: It returns the number of milliseconds since the start of the day in which _tv_ falls.

1. Return ℝ(_tv_) modulo MillisecondsPerDay.

--- referenced: TimeClip ( _time_: a Number, ): a time value
  id: sec-timeclip

description: It calculates a number of milliseconds.

1. If _time_ is not a finite Number, return *NaN*.
2. If abs(ℝ(_time_)) > 8.64 × 10^15, return *NaN*.
3. Return 𝔽(! ToIntegerOrInfinity(_time_)).


## Done when

- Every test262 run under built-ins/Date/prototype/setMonth/ passes (check with `./t262 built-ins/Date/prototype/setMonth/`). 8 pass on the main branch right now.

