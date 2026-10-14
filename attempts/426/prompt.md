# Ticket B-date-prototype-to-date-string: Date.prototype.toDateString

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

- `crates/kahvi-builtins/src/date/prototype_to_date_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype.toDateString ( )
  id: sec-date.prototype.todatestring   type: built-in function

This method performs the following steps when called:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _tv_ be _dateObj_.[[DateValue]].
4. If _tv_ is *NaN*, return *"Invalid Date"*.
5. Let _t_ be LocalTime(_tv_).
6. Return DateString(_t_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: DateString ( _tv_: a Number, but not *NaN*, ): a String
  id: sec-datestring

1. Let _weekday_ be the Name of the entry in [sec-todatestring-day-names] whose WeekDay Index = WeekDay(_tv_).
2. Let _month_ be the Name of the entry in [sec-todatestring-month-names] whose Month Index = MonthFromTime(_tv_).
3. Let _day_ be ToZeroPaddedDecimalString(DateFromTime(_tv_), 2).
4. Let _yv_ be YearFromTime(_tv_).
5. If _yv_ ≥ 0, let _yearSign_ be the empty String; else let _yearSign_ be *"-"*.
6. Let _paddedYear_ be ToZeroPaddedDecimalString(abs(_yv_), 4).
7. Return the string-concatenation of _weekday_, the code unit 0x0020 (SPACE), _month_, the code unit 0x0020 (SPACE), _day_, the code unit 0x0020 (SPACE), _yearSign_, and _paddedYear_.

WeekDay Index
|
Name
|
0
|
*"Sun"*
|
1
|
*"Mon"*
|
2
|
*"Tue"*
|
3
|
*"Wed"*
|
4
|
*"Thu"*
|
5
|
*"Fri"*
|
6
|
*"Sat"*
|
Month Index
|
Name
|
0
|
*"Jan"*
|
1
|
*"Feb"*
|
2
|
*"Mar"*
|
3
|
*"Apr"*
|
4
|
*"May"*
|
5
|
*"Jun"*
|
6
|
*"Jul"*
|
7
|
*"Aug"*
|
8
|
*"Sep"*
|
9
|
*"Oct"*
|
10
|
*"Nov"*
|
11
|
*"Dec"*
|


## Done when

- Every test262 run under built-ins/Date/prototype/toDateString/ passes (check with `./t262 built-ins/Date/prototype/toDateString/`). 8 pass on the main branch right now.

