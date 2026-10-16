# Ticket I-date-to-locale-string: Date.prototype toLocaleString / toLocaleDateString / toLocaleTimeString (ECMA-402)

This built-in already implements ECMA-262. ECMA-402 supersedes it (the
spec text below): when `agent.intl` is true, follow the ECMA-402 steps;
when it is false, keep today's ECMA-262 behaviour exactly as it is. Keep
each spec step as a comment directly above the code for it. Read
docs/INTL.md first.

- Construct the Intl service through its intrinsic (`ops::construct` on
  `agent.intrinsic("%Intl.X%")`), as the spec does, unless a support
  helper does the same thing without the observable lookups.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))`. No new Rust tests.

## Files you may change

- `crates/kahvi-builtins/src/date/prototype_to_locale_string.rs`
- `crates/kahvi-builtins/src/date/prototype_to_locale_date_string.rs`
- `crates/kahvi-builtins/src/date/prototype_to_locale_time_string.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/support/intl_date_time_format.rs`

## Spec

Date.prototype.toLocaleString ( [ _locales_ [ , _options_ ] ] )
  id: sup-date.prototype.tolocalestring   type: built-in function

This definition supersedes the definition provided in ECMA-262, [sec-date.prototype.tolocalestring].
When the `toLocaleString` method is called with optional arguments _locales_ and _options_, the following steps are taken:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _x_ be _dateObj_.[[DateValue]].
4. If _x_ is *NaN*, return *"Invalid Date"*.
5. Let _dateFormat_ be ? CreateDateTimeFormat(%Intl.DateTimeFormat%, _locales_, _options_, ~any~, ~all~).
6. Return ! FormatDateTime(_dateFormat_, _x_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: FormatDateTime ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, ): either a normal completion containing a String or a throw completion
  id: sec-formatdatetime

1. Let _parts_ be ? PartitionDateTimePattern(_dateTimeFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


Date.prototype.toLocaleDateString ( [ _locales_ [ , _options_ ] ] )
  id: sup-date.prototype.tolocaledatestring   type: built-in function

This definition supersedes the definition provided in [sec-date.prototype.tolocaledatestring].
When the `toLocaleDateString` method is called with optional arguments _locales_ and _options_, the following steps are taken:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _x_ be _dateObj_.[[DateValue]].
4. If _x_ is *NaN*, return *"Invalid Date"*.
5. Let _dateFormat_ be ? CreateDateTimeFormat(%Intl.DateTimeFormat%, _locales_, _options_, ~date~, ~date~).
6. Return ! FormatDateTime(_dateFormat_, _x_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: FormatDateTime ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, ): either a normal completion containing a String or a throw completion
  id: sec-formatdatetime

1. Let _parts_ be ? PartitionDateTimePattern(_dateTimeFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


Date.prototype.toLocaleTimeString ( [ _locales_ [ , _options_ ] ] )
  id: sup-date.prototype.tolocaletimestring   type: built-in function

This definition supersedes the definition provided in [sec-date.prototype.tolocaletimestring].
When the `toLocaleTimeString` method is called with optional arguments _locales_ and _options_, the following steps are taken:

1. Let _dateObj_ be the *this* value.
2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
3. Let _x_ be _dateObj_.[[DateValue]].
4. If _x_ is *NaN*, return *"Invalid Date"*.
5. Let _timeFormat_ be ? CreateDateTimeFormat(%Intl.DateTimeFormat%, _locales_, _options_, ~time~, ~time~).
6. Return ! FormatDateTime(_timeFormat_, _x_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: FormatDateTime ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, ): either a normal completion containing a String or a throw completion
  id: sec-formatdatetime

1. Let _parts_ be ? PartitionDateTimePattern(_dateTimeFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


## Done when

- Every test262 run under intl402/Date/ passes (check with `./t262 intl402/Date/`). 20 pass on the main branch right now.

