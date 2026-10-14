# Ticket B-date-prototype-set-minutes: Date.prototype.setMinutes

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

- `crates/kahvi-builtins/src/date/prototype_set_minutes.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.setMinutes ( _minute_ [ , _second_ [ , _millisecond_ ] ] )
  id: sec-date.prototype.setminutes   type: built-in function

This method performs the following steps when called:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _tv_ be _dateObj_.[[DateValue]].
4. Let _minuteNumber_ be ? ToNumber(_minute_).
5. If _second_ is present, let _secondNumber_ be ? ToNumber(_second_).
6. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_).
7. If _tv_ is *NaN*, return *NaN*.
8. Set _tv_ to LocalTime(_tv_).
9. If _second_ is not present, let _secondNumber_ be 𝔽(SecondFromTime(_tv_)).
10. If _millisecond_ is not present, let _millisecondNumber_ be 𝔽(MillisecondFromTime(_tv_)).
11. Let _date_ be MakeDate(𝔽(Day(_tv_)), MakeTime(𝔽(HourFromTime(_tv_)), _minuteNumber_, _secondNumber_, _millisecondNumber_)).
12. Let _u_ be TimeClip(UTC(_date_)).
13. Set _dateObj_.[[DateValue]] to _u_.
14. Return _u_.

The *"length"* property of this method is *3*_𝔽.
NOTE:
If _second_ is not present, this method behaves as if _second_ was present with the value `getSeconds()`. If _millisecond_ is not present, this behaves as if _millisecond_ was present with the value `getMilliseconds()`.

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

--- referenced: SecondFromTime ( _tv_: a finite time value, ): an integer in the inclusive interval from 0 to 59
  id: sec-secondfromtime

description: It returns the second of the minute in which _tv_ falls.

1. Return floor(ℝ(_tv_) / MillisecondsPerSecond) modulo SecondsPerMinute.

--- referenced: MillisecondFromTime ( _tv_: a finite time value, ): an integer in the inclusive interval from 0 to 999
  id: sec-millisecondfromtime

description: It returns the millisecond of the second in which _tv_ falls.

1. Return ℝ(_tv_) modulo MillisecondsPerSecond.

--- referenced: MakeDate ( _day_: a Number, _time_: a Number, ): a finite Number or *NaN*
  id: sec-makedate

description: It calculates a number of milliseconds.

1. If _day_ is not a finite Number or _time_ is not a finite Number, return *NaN*.
2. Let _tv_ be _day_ × 𝔽(MillisecondsPerDay) + _time_.
3. If _tv_ is not a finite Number, return *NaN*.
4. Return _tv_.

--- referenced: Day ( _tv_: a finite time value, ): an integer
  id: sec-day

description: It returns the day number of the day in which _tv_ falls.

1. Return floor(ℝ(_tv_) / MillisecondsPerDay).

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

--- referenced: HourFromTime ( _tv_: a finite time value, ): an integer in the inclusive interval from 0 to 23
  id: sec-hourfromtime

description: It returns the hour of the day in which _tv_ falls.

1. Return floor(ℝ(_tv_) / MillisecondsPerHour) modulo HoursPerDay.

--- referenced: TimeClip ( _time_: a Number, ): a time value
  id: sec-timeclip

description: It calculates a number of milliseconds.

1. If _time_ is not a finite Number, return *NaN*.
2. If abs(ℝ(_time_)) > 8.64 × 10^15, return *NaN*.
3. Return 𝔽(! ToIntegerOrInfinity(_time_)).


## Done when

- Every test262 run under built-ins/Date/prototype/setMinutes/ passes (check with `./t262 built-ins/Date/prototype/setMinutes/`). 8 pass on the main branch right now.

