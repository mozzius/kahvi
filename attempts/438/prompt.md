# Ticket B-date-prototype-to-time-string: Date.prototype.toTimeString

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

- `crates/kahvi-builtins/src/date/prototype_to_time_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.toTimeString ( )
  id: sec-date.prototype.totimestring   type: built-in function

This method performs the following steps when called:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _tv_ be _dateObj_.[[DateValue]].
4. If _tv_ is *NaN*, return *"Invalid Date"*.
5. Let _localTime_ be LocalTime(_tv_).
6. Return the string-concatenation of TimeString(_localTime_) and TimeZoneString(_tv_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: TimeString ( _tv_: a Number, but not *NaN*, ): a String
  id: sec-timestring

1. Let _hour_ be ToZeroPaddedDecimalString(HourFromTime(_tv_), 2).
2. Let _minute_ be ToZeroPaddedDecimalString(MinuteFromTime(_tv_), 2).
3. Let _second_ be ToZeroPaddedDecimalString(SecondFromTime(_tv_), 2).
4. Return the string-concatenation of _hour_, *":"*, _minute_, *":"*, _second_, the code unit 0x0020 (SPACE), and *"GMT"*.

--- referenced: TimeZoneString ( _tv_: an integral Number, ): a String
  id: sec-timezoneestring

1. Let _systemTimeZoneIdentifier_ be SystemTimeZoneIdentifier().
2. If IsTimeZoneOffsetString(_systemTimeZoneIdentifier_) is *true*, then
   a. Let _offsetNanoseconds_ be ParseTimeZoneOffsetString(_systemTimeZoneIdentifier_).
3. Else,
   a. Let _offsetNanoseconds_ be GetNamedTimeZoneOffsetNanoseconds(_systemTimeZoneIdentifier_, ℝ(_tv_) × NanosecondsPerMillisecond).
4. Let _offsetMilliseconds_ be truncate(_offsetNanoseconds_ / NanosecondsPerMillisecond).
5. If _offsetMilliseconds_ ≥ 0, then
   a. Let _offsetSign_ be *"+"*.
   b. Let _absOffsetMilliseconds_ be _offsetMilliseconds_.
6. Else,
   a. Let _offsetSign_ be *"-"*.
   b. Let _absOffsetMilliseconds_ be -_offsetMilliseconds_.
7. Let _offsetMinute_ be ToZeroPaddedDecimalString(MinuteFromTime(𝔽(_absOffsetMilliseconds_)), 2).
8. Let _offsetHour_ be ToZeroPaddedDecimalString(HourFromTime(𝔽(_absOffsetMilliseconds_)), 2).
9. Let _timeZoneName_ be an implementation-defined string that is either the empty String or the string-concatenation of the code unit 0x0020 (SPACE), the code unit 0x0028 (LEFT PARENTHESIS), an implementation-defined time zone name, and the code unit 0x0029 (RIGHT PARENTHESIS).
10. Return the string-concatenation of _offsetSign_, _offsetHour_, _offsetMinute_, and _timeZoneName_.


## Done when

- Every test262 run under built-ins/Date/prototype/toTimeString/ passes (check with `./t262 built-ins/Date/prototype/toTimeString/`). 8 pass on the main branch right now.

