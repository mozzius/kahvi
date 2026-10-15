# Ticket I-date-time-format: Intl.DateTimeFormat

Implement this Intl service: the constructor, its internal slots and its
prototype methods, following the spec steps already in each stub (keep each
step as a comment directly above the code for it). Read docs/INTL.md first.

- The negotiation and option operations (ResolveOptions, GetOption, ...)
  are in `support/intl.rs`; do not re-implement them. Put this service's
  internal-slot struct and shared abstract operations in its own
  `support/intl_*.rs` file (in scope).
- Locale data and formatting come from ICU4X (`icu` crate). Never write
  tables of expected outputs.
- Work in steps: the constructor and resolvedOptions first, then the
  methods. Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing.
- No new Rust tests: test262 is the measure.
- Map the component options to an ICU4X field set and let ICU4X pick the
  pattern; resolvedOptions reports the components of the pattern used.
- Intl.DateTimeFormat called without `new` on an Intl object uses
  `support::intl::fallback_symbol` (ChainDateTimeFormat).

## Files you may change

- `crates/kahvi-builtins/src/intl_date_time_format/*.rs`
- `crates/kahvi-builtins/src/support/intl_date_time_format.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs`
- `crates/kahvi-builtins/src/disposable_stack/constructor.rs`

## Spec

Intl.DateTimeFormat ( [ _locales_ [ , _options_ ] ] )
  id: sec-intl.datetimeformat   type: built-in function

When the `Intl.DateTimeFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:

1. If NewTarget is *undefined*, let _newTarget_ be the active function object, else let _newTarget_ be NewTarget.
2. Let _dateTimeFormat_ be ? CreateDateTimeFormat(_newTarget_, _locales_, _options_, ~any~, ~date~).
3. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
   a. Let _this_ be the *this* value.
   b. Return ? ChainDateTimeFormat(_dateTimeFormat_, NewTarget, _this_).
4. Return _dateTimeFormat_.

[subclause sec-chaindatetimeformat: ChainDateTimeFormat (
_dateTimeFormat_: an Intl.DateTimeFormat,
_newTarget_: an ECMAScript language value,
_this_: an ECMAScript language value,
): either a normal completion containing an Object or a throw completion]

--- referenced: ChainDateTimeFormat ( _dateTimeFormat_: an Intl.DateTimeFormat, _newTarget_: an ECMAScript language value, _this_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-chaindatetimeformat

1. If _newTarget_ is *undefined* and ? OrdinaryHasInstance(%Intl.DateTimeFormat%, _this_) is *true*, then
   a. Perform ? DefinePropertyOrThrow(_this_, %Intl%.[[FallbackSymbol]], PropertyDescriptor { [[Value]]: _dateTimeFormat_, [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
   b. Return _this_.
2. Return _dateTimeFormat_.


ChainDateTimeFormat ( _dateTimeFormat_: an Intl.DateTimeFormat, _newTarget_: an ECMAScript language value, _this_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-chaindatetimeformat   type: abstract operation

1. If _newTarget_ is *undefined* and ? OrdinaryHasInstance(%Intl.DateTimeFormat%, _this_) is *true*, then
   a. Perform ? DefinePropertyOrThrow(_this_, %Intl%.[[FallbackSymbol]], PropertyDescriptor { [[Value]]: _dateTimeFormat_, [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
   b. Return _this_.
2. Return _dateTimeFormat_.

--- referenced: OrdinaryHasInstance ( _ctor_: an ECMAScript language value, _instance_: an ECMAScript language value, ): either a normal completion containing a Boolean or a throw completion
  id: sec-ordinaryhasinstance

description: It implements the default algorithm for determining if _instance_ inherits from the instance object inheritance path provided by _ctor_.

1. If IsCallable(_ctor_) is *false*, return *false*.
2. If _ctor_ has a [[BoundTargetFunction]] internal slot, then
   a. Let _boundCtor_ be _ctor_.[[BoundTargetFunction]].
   b. Return ? InstanceofOperator(_instance_, _boundCtor_).
3. If _instance_ is not an Object, return *false*.
4. Let _proto_ be ? Get(_ctor_, *"prototype"*).
5. If _proto_ is not an Object, throw a *TypeError* exception.
6. Repeat,
   a. Set _instance_ to ? _instance_.[[GetPrototypeOf]]().
   b. If _instance_ is *null*, return *false*.
   c. If SameValue(_proto_, _instance_) is *true*, return *true*.

--- referenced: DefinePropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _propertyDesc_: a Property Descriptor, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-definepropertyorthrow

description: It is used to call the [[DefineOwnProperty]] internal method of an object in a manner that will throw a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? _obj_.[[DefineOwnProperty]](_propertyKey_, _propertyDesc_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.


…dar_ be _resolvedLocale_.[[ca]].
10. Set _dateTimeFormat_.[[Calendar]] to _resolvedCalendar_.
11. Set _dateTimeFormat_.[[NumberingSystem]] to _resolvedLocale_.[[nu]].
12. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
13. Let _hc_ be _resolvedLocale_.[[hc]].
14. If _hour12_ is *true*, then
   a. Set _hc_ to _resolvedLocaleData_.[[hourCycle12]].
15. Else if _hour12_ is *false*, then
   a. Set _hc_ to _resolvedLocaleData_.[[hourCycle24]].
16. Else if _hc_ is *null*, then
   a. Set _hc_ to _resolvedLocaleData_.[[hourCycle]].
17. Let _timeZone_ be ? GetOption(_options_, *"timeZone"*, ~string~, ~empty~, SystemTimeZoneIdentifier()).
18. If IsTimeZoneOffsetString(_timeZone_) is *true*, then
   a. Let _parseResult_ be ParseText(StringToCodePoints(_timeZone_), |UTCOffset|).
   b. Assert: _parseResult_ is a Parse Node.
   c. If _parseResult_ contains more than one |MinuteSecond| Parse Node, throw a *RangeError* exception.
   d. Let _offsetNanoseconds_ be ParseTimeZoneOffsetString(_timeZone_).
   e. Let _offsetMinutes_ be _offsetNanoseconds_ / (6 × 10^10).
   f. Assert: _offsetMinutes_ is an integer.
   g. Set _timeZone_ to FormatOffsetTimeZoneIdentifier(_offsetMinutes_).
19. Else,
   a. Let _timeZoneIdentifierRecord_ be GetAvailableNamedTimeZoneIdentifier(_timeZone_).
   b. If _timeZoneIdentifierRecord_ is ~empty~, throw a *RangeError* exception.
   c. Set _timeZone_ to _timeZoneIdentifierRecord_.[[PrimaryIdentifier]].
20. Set _dateTimeFormat_.[[TimeZone]] to _timeZone_.
21. Let _formatOptions_ be a new Record.
22. Set _formatOptions_.[[hourCycle]] to _hc_.
23. Let _hasExplicitFormatComponents_ be *false*.
24. For each row of [table-datetimeformat-components], except the header row, in table order, do
   a. Let _propertyKey_ be the name given in the Property column of the current row.
   b. If _propertyKey_ is *"fractionalSecondDigits"*, then
      i. Let _value_ be ? GetNumberOption(_options_, *"fractionalSecondDigits"*, 1, 3, *undefined*).
   c. Else,
      i. Let _values_ be a List whose elements are the strings given in the Values column of the current row.
      ii. Let _value_ be ? GetOption(_options_, _propertyKey_, ~string~, _values_, *undefined*).
   d. Set _formatOptions_.[[<_propertyKey_>]] to _value_.
   e. If _value_ is not *undefined*, set _hasExplicitFormatComponents_ to *true*.
25. Let _formatMatcher_ be ? GetOption(_options_, *"formatMatcher"*, ~string~, « *"basic"*, *"best fit"* », *"best fit"*).
26. Let _dateStyle_ be ? GetOption(_options_, *"dateStyle"*, ~string~, « *"full"*, *"long"*, *"medium"*, *"short"* », *undefined*).
27. Set _dateTimeFormat_.[[DateStyle]] to _dateStyle_.
28. Let _timeStyle_ be ? GetOption(_options_, *"timeStyle"*, ~string~, « *"full"*, *"long"*, *"medium"*, *"short"* », *undefined*).
29. Set _dateTimeFormat_.[[TimeStyle]] to _timeStyle_.
30. If _dateStyle_ is not *undefined* or _timeStyle_ is not *undefined*, then
   a. If _hasExplicitFormatComponents_ is *true*, throw a *TypeError* exception.
   b. If _required_ is ~date~ and _timeStyle_ is not *undefined*, throw a *TypeError* exception.
   c. If _required_ is ~time~ and _dateStyle_ is not *undefined*, throw a *TypeError* exception.
   d. Let _styles_ be _resolvedLocaleData_.[[styles]].[[<_resolvedCalendar_>]].
   e. Let _bestFormat_ be DateTimeStyleFormat(_dateStyle_, _timeStyle_, _styles_).
31. Else,
   a. Let _needDefaults_ be *true*.
   b. Let _propertyKeys_ be a new empty List.
   c. If _required_ is ~date~ or ~any~, set _propertyKeys_ to the list-concatenation of _propertyKeys_ and « *"weekday"*, *"year"*, *"month"*, *"day"* ».
   d. If _required_ is ~time~ or ~any~, set _propertyKeys_ to the list-concatenation of _propertyKeys_ and « *"dayPeriod"*, *"hour"*, *"minute"*, *"second"*, *"fractionalSecondDigits"* ».
   e. For each property name _propertyKey_ of _propertyKeys_, do
      i. Let _value_ be _formatOptions_.[[<_propertyKey_>]].
      ii. If _value_ is not *undefined*, set _needDefaults_ to *false*.
   f. If _needDefaults_ is *true*, then
      i. Let _numericProperties_ be a new empty List.
      ii. If _defaults_ is either ~date~ or ~all~, set _numericProperties_ to the list-concatenation of _numericProperties_ and « *"year"*, *"month"*, *"day"* ».
      iii. If _defaults_ is either ~time~ or ~all~, set _numericProperties_ to the list-concatenation of _numericProperties_ and « *"hour"*, *"minute"*, *"second"* ».
      iv. For each property name _propertyKey_ of _numericProperties_, do
         1. Set _formatOptions_.[[<_propertyKey_>]] to *"numeric"*.
   g. Let _formats_ be _resolvedLocaleData_.[[formats]].[[<_resolvedCalendar_>]].
   h. If _formatMatcher_ is *"basic"*, let _bestFormat_ be BasicFormatMatcher(_formatOptions_, _formats_); else let _bestFormat_ be BestFitFormatMatcher(_formatOptions_, _formats_).
32. Set _dateTimeFormat_.[[DateTimeFormat]] to _bestFormat_.
33. If _bestFormat_ has a field [[hour]], set _dateTimeFormat_.[[HourCycle]] to _hc_.
34. Return _dateTimeFormat_.

--- referenced: OrdinaryCreateFromConstructor ( _ctor_: a function object, _intrinsicDefaultProto_: a String, optional _internalSlotsList_: a List of names of internal slots, ): either a normal completion containing an Object or a throw completion
  id: sec-ordinarycreatefromconstructor

description: It creates an ordinary object whose [[Prototype]] value is retrieved from a constructor's *"prototype"* property, if it exists. Otherwise the intrinsic named by _intrinsicDefaultProto_ is used for [[Prototype]]. _internalSlotsList_ contains the names of additional internal slots that must be defined as part of the object. If _internalSlotsList_ is not provided, a new empty List is used.

1. Assert: _intrinsicDefaultProto_ is this specification's name of an intrinsic object. The corresponding object must be an intrinsic that is intended to be used as the [[Prototype]] value of an object.
2. Let _proto_ be ? GetPrototypeFromConstructor(_ctor_, _intrinsicDefaultProto_).
3. If _internalSlotsList_ is present, let _slots_ be _internalSlotsList_.
4. Else, let _slots_ be a new empty List.
5. Return OrdinaryObjectCreate(_proto_, _slots_).

--- referenced: GetOption ( _options_: an Object, _propertyKey_: a property key, _type_: ~boolean~ or ~string~, _values_: ~empty~ or a List of ECMAScript language values, _default_: ~required~ or an ECMAScript language value, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-getoption

description: It extracts the value of the specified property of _options_, converts it to the required _type_, checks whether it is allowed by _values_ if _values_ is not ~empty~, and substitutes _default_ if the value is *undefined*.

1. Let _value_ be ? Get(_options_, _propertyKey_).
2. If _value_ is *undefined*, then
   a. If _default_ is ~required~, throw a *TypeError* exception.
   b. Return _default_.
3. If _type_ is ~boolean~, then
   a. Set _value_ to ToBoolean(_value_).
4. Else,
   a. Assert: _type_ is ~string~.
   b. Set _value_ to ? ToString(_value_).
5. If _values_ is not ~empty~ and _values_ does not contain _value_, throw a *RangeError* exception.
6. Return _value_.

--- referenced: SystemTimeZoneIdentifier ( ): a String
  id: sec-systemtimezoneidentifier

description:
It returns a String representing the host environment's current time zone, which is either a String representing a UTC offset for which IsTimeZoneOffsetString returns *true*, or a primary time zone identifier.

1. If the implementation only supports the UTC time zone, return *"UTC"*.
2. Let _systemTimeZoneString_ be the String representing the host environment's current time zone, either a primary time zone identifier or an offset time zone identifier.
3. Return _systemTimeZoneString_.

NOTE:
To ensure the level of functionality that implementations commonly provide in the methods of the Date object, it is recommended that SystemTimeZoneIdentifier return an IANA time zone name corresponding to the host environment's time zone setting, if such a thing exists.
GetNamedTimeZoneEpochNanoseconds and GetNamedTimeZoneOffsetNanoseconds must reflect the local political rules for standard time and daylight saving time in that time zone, if such rules exist.
For example, if the host environment is a browser on a system where the user has chosen US Eastern Time as their time zone, SystemTimeZoneIdentifier returns *"America/New_York"*.

--- referenced: IsTimeZoneOffsetString ( _offsetString_: a String, ): a Boolean
  id: sec-istimezoneoffsetstring

description: The return value indicates whether _offsetString_ conforms to the grammar given by |UTCOffset|.

1. Let _parseResult_ be ParseText(_offsetString_, |UTCOffset|).
2. If _parseResult_ is a List of errors, return *false*.
3. Return *true*.

--- referenced: FormatOffsetTimeZoneIdentifier ( _offsetMinutes_: an integer, ): a String
  id: sec-formatoffsettimezoneidentifier

description:
It formats a UTC offset, in minutes, into a UTC offset string formatted like ±HH:MM.

1. If _offsetMinutes_ ≥ 0, let _sign_ be the code unit 0x002B (PLUS SIGN); otherwise, let _sign_ be the code unit 0x002D (HYPHEN-MINUS).
2. Let _absoluteMinutes_ be abs(_offsetMinutes_).
3. Let _hours_ be floor(_absoluteMinutes_ / 60).
4. Let _minutes_ be _absoluteMinutes_ modulo 60.
5. Return the string-concatenation of _sign_, ToZeroPaddedDecimalString(_hours_, 2), the code unit 0x003A (COLON), and ToZeroPaddedDecimalString(_minutes_, 2).

--- referenced: GetAvailableNamedTimeZoneIdentifier ( _timeZoneIdentifier_: a String, ): either a Time Zone Identifier Record or ~empty~
  id: sec-getavailablenamedtimezoneidentifier

description:
If _timeZoneIdentifier_ is an available named time zone identifier, then it returns one of the Records in the List returned by AvailableNamedTimeZoneIdentifiers.
Otherwise, ~empty~ will be returned.

1. For each element _record_ of AvailableNamedTimeZoneIdentifiers(), do
   a. If _record_.[[Identifier]] is an ASCII-case-insensitive match for _timeZoneIdentifier_, return _record_.
2. Return ~empty~.

NOTE:
For any _timeZoneIdentifier_, or any value that is an ASCII-case-insensitive match for it, it is required that the resulting Time Zone Identifier Record contain the same field values for the lifetime of the surrounding agent.
Furthermore, it is required that time zone identifiers not dynamically change from primary to non-primary during the lifetime of the surrounding agent, meaning that if _timeZoneIdentifier_ is an ASCII-case-insensitive match for the [[PrimaryIdentifier]] field of the result of a previous call to GetAvailableNamedTimeZoneIdentifier, then GetAvailableNamedTimeZoneIdentifier(_timeZoneIdentifier_) must return a Time Zone Identifier Record where [[Identifier]] is [[PrimaryIdentifier]].
Due to the complexity of supporting these requirements, it is recommended that the result of AvailableNamedTimeZoneIdentifiers (and therefore GetAvailableNamedTimeZoneIdentifier too) remains the same for the lifetime of the surrounding agent.

--- referenced: GetNumberOption ( _options_: an Object, _propertyKey_: a String, _minimum_: an integer, _maximum_: an integer, _fallback_: an integer or *undefined*, ): either a normal completion containing either an integer or *undefined*, or a throw completion
  id: sec-getnumberoption

description: It extracts the value of the property named _propertyKey_ from the provided _options_ object, converts it to an integer, checks whether it is in the allowed range, and fills in a _fallback_ value if necessary.

1. Let _value_ be ? Get(_options_, _propertyKey_).
2. Return ? DefaultNumberOption(_value_, _minimum_, _maximum_, _fallback_).

--- referenced: BestFitFormatMatcher ( _options_: a Record, _formats_: a List of DateTime Format Records, ): a DateTime Format Record
  id: sec-bestfitformatmatcher

description: It returns a set of component representations that a typical user of the selected locale would perceive as at least as good as the one returned by BasicFormatMatcher.


FormatDateTime ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, ): either a normal completion containing a String or a throw completion
  id: sec-formatdatetime   type: abstract operation

1. Let _parts_ be ? PartitionDateTimePattern(_dateTimeFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.

--- referenced: PartitionDateTimePattern ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, ): either a normal completion containing a List of Records with fields [[Type]] (a String) and [[Value]] (a String), or a throw completion
  id: sec-partitiondatetimepattern

description: It interprets _x_ as a time value as specified in ECMA-262, [sec-time-values-and-time-range], and creates the corresponding parts according to the effective locale and the formatting options of _dateTimeFormat_.

1. Set _x_ to TimeClip(_x_).
2. If _x_ is *NaN*, throw a *RangeError* exception.
3. Let _epochNanoseconds_ be ℤ(ℝ(_x_) × 10^6).
4. Let _format_ be _dateTimeFormat_.[[DateTimeFormat]].
5. If _dateTimeFormat_.[[HourCycle]] is *"h11"* or *"h12"*, then
   a. Let _pattern_ be _format_.[[pattern12]].
6. Else,
   a. Let _pattern_ be _format_.[[pattern]].
7. Let _result_ be FormatDateTimePattern(_dateTimeFormat_, _format_, _pattern_, _epochNanoseconds_).
8. Return _result_.


## Done when

- Every test262 run under intl402/DateTimeFormat/ passes (check with `./t262 intl402/DateTimeFormat/`). 0 pass on the main branch right now.

