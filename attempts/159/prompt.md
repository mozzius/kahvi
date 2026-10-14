# Ticket B-date-utc: Date.UTC

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

- `crates/kahvi-builtins/src/date/utc.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.UTC ( _year_ [ , _month_ [ , _day_ [ , _hour_ [ , _minute_ [ , _second_ [ , _millisecond_ ] ] ] ] ] ] )
  id: sec-date.utc   type: built-in function

This function performs the following steps when called:

1. Let _yearNumber_ be ? ToNumber(_year_).
2. If _month_ is present, let _monthNumber_ be ? ToNumber(_month_); else let _monthNumber_ be *+0*_𝔽.
3. If _day_ is present, let _dayNumber_ be ? ToNumber(_day_); else let _dayNumber_ be *1*_𝔽.
4. If _hour_ is present, let _hourNumber_ be ? ToNumber(_hour_); else let _hourNumber_ be *+0*_𝔽.
5. If _minute_ is present, let _minuteNumber_ be ? ToNumber(_minute_); else let _minuteNumber_ be *+0*_𝔽.
6. If _second_ is present, let _secondNumber_ be ? ToNumber(_second_); else let _secondNumber_ be *+0*_𝔽.
7. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_); else let _millisecondNumber_ be *+0*_𝔽.
8. Set _yearNumber_ to MakeFullYear(_yearNumber_).
9. Return TimeClip(MakeDate(MakeDay(_yearNumber_, _monthNumber_, _dayNumber_), MakeTime(_hourNumber_, _minuteNumber_, _secondNumber_, _millisecondNumber_))).

The *"length"* property of this function is *7*_𝔽.
NOTE:
This function differs from the Date constructor in two ways: it returns a time value as a Number, rather than creating a Date, and it interprets the arguments in UTC rather than as local time.

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

--- referenced: MakeFullYear ( _year_: a Number, ): an integral Number or *NaN*
  id: sec-makefullyear

description: It returns the full year associated with the integer part of _year_, interpreting any value in the inclusive interval from 0 to 99 as a count of years since the start of 1900. For alignment with the proleptic Gregorian calendar, “full year” is defined as the signed count of complete years since the start of year 0 (1 B.C.).

1. If _year_ is one of *NaN*, *+∞*_𝔽, or *-∞*_𝔽, return *NaN*.
2. Let _truncated_ be ! ToIntegerOrInfinity(_year_).
3. If _truncated_ is in the inclusive interval from 0 to 99, return *1900*_𝔽 + 𝔽(_truncated_).
4. Return 𝔽(_truncated_).

--- referenced: TimeClip ( _time_: a Number, ): a time value
  id: sec-timeclip

description: It calculates a number of milliseconds.

1. If _time_ is not a finite Number, return *NaN*.
2. If abs(ℝ(_time_)) > 8.64 × 10^15, return *NaN*.
3. Return 𝔽(! ToIntegerOrInfinity(_time_)).

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

--- referenced: MakeTime ( _hour_: a Number, _minute_: a Number, _second_: a Number, _millisecond_: a Number, ): a Number
  id: sec-maketime

description: It calculates a number of milliseconds.

1. If _hour_ is not a finite Number, _minute_ is not a finite Number, _second_ is not a finite Number, or _millisecond_ is not a finite Number, return *NaN*.
2. Let _hourMV_ be ! ToIntegerOrInfinity(_hour_).
3. Let _minuteMV_ be ! ToIntegerOrInfinity(_minute_).
4. Let _secondMV_ be ! ToIntegerOrInfinity(_second_).
5. Let _millisecondMV_ be ! ToIntegerOrInfinity(_millisecond_).
6. Return ((𝔽(_hourMV_) × 𝔽(MillisecondsPerHour) + 𝔽(_minuteMV_) × 𝔽(MillisecondsPerMinute)) + 𝔽(_secondMV_) × 𝔽(MillisecondsPerSecond)) + 𝔽(_millisecondMV_).

NOTE:
The arithmetic in MakeTime is floating-point arithmetic, which is not associative, so the operations must be performed in the correct order.


## Done when

- Every test262 run under built-ins/Date/UTC/ passes (check with `./t262 built-ins/Date/UTC/`). 8 pass on the main branch right now.

