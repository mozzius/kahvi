# Ticket I-date-time-format: Intl.DateTimeFormat

The constructor, CreateDateTimeFormat and resolvedOptions are done (the
architect's ICU4X layer is `support/intl_dtf_icu.rs`; read it first). This
ticket is the formatting: the `format` getter, FormatDateTime,
`formatToParts`, `formatRange` and `formatRangeToParts`, following the spec
steps already in each stub (keep each step as a comment directly above the
code for it). Read docs/INTL.md first.

- Format with ICU4X, never by hand: `intl_dtf_icu::formatter(&prefs,
  &slots.fields)` gives the `DateTimeFormatter` for the format the
  constructor chose (`Prefs` from the slots: [[Locale]], [[Calendar]],
  [[NumberingSystem]], [[HourCycle]]). Convert the time value (epoch
  milliseconds) to an ICU4X zoned date-time in [[TimeZone]] (a named zone
  or a UTC offset) and format it; for formatToParts, map ICU4X's parts
  (`writeable` parts, `icu::datetime::parts`) to the spec's part types.
- [[HourCycle]] "h24" has no ICU4X pattern: throw not-implemented there.
- A range is two formatted dates unless ICU4X has range patterns for the
  field set; whatever ICU4X cannot do throws not-implemented naming it.
- Never write tables of expected outputs. No new Rust tests.

## Files you may change

- `crates/kahvi-builtins/src/intl_date_time_format/*.rs`
- `crates/kahvi-builtins/src/support/intl_date_time_format.rs`

## Read first

- `crates/kahvi-builtins/src/support/intl_dtf_icu.rs`
- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs`
- `crates/kahvi-builtins/src/disposable_stack/constructor.rs`

## Spec

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


… CreateDataPropertyOrThrow(_nf2Options_, *"minimumIntegerDigits"*, *2*_𝔽).
8. Perform ! CreateDataPropertyOrThrow(_nf2Options_, *"numberingSystem"*, _dateTimeFormat_.[[NumberingSystem]]).
9. Perform ! CreateDataPropertyOrThrow(_nf2Options_, *"useGrouping"*, *false*).
10. Let _nf2_ be ! Construct(%Intl.NumberFormat%, « _locale_, _nf2Options_ »).
11. If _format_ has a field [[fractionalSecondDigits]], then
   a. Let _fractionalSecondDigits_ be _format_.[[fractionalSecondDigits]].
   b. Let _nf3Options_ be OrdinaryObjectCreate(*null*).
   c. Perform ! CreateDataPropertyOrThrow(_nf3Options_, *"minimumIntegerDigits"*, 𝔽(_fractionalSecondDigits_)).
   d. Perform ! CreateDataPropertyOrThrow(_nf3Options_, *"numberingSystem"*, _dateTimeFormat_.[[NumberingSystem]]).
   e. Perform ! CreateDataPropertyOrThrow(_nf3Options_, *"useGrouping"*, *false*).
   f. Let _nf3_ be ! Construct(%Intl.NumberFormat%, « _locale_, _nf3Options_ »).
12. Let _localTime_ be ToLocalTime(_epochNanoseconds_, _dateTimeFormat_.[[Calendar]], _dateTimeFormat_.[[TimeZone]]).
13. Let _patternParts_ be PartitionPattern(_pattern_).
14. Let _result_ be a new empty List.
15. For each Record { [[Type]], [[Value]] } _patternPart_ of _patternParts_, do
   a. Let _partType_ be _patternPart_.[[Type]].
   b. If _partType_ is *"literal"*, then
      i. Append the Record { [[Type]]: *"literal"*, [[Value]]: _patternPart_.[[Value]] } to _result_.
   c. Else if _partType_ is *"fractionalSecondDigits"*, then
      i. Assert: _format_ has a field [[fractionalSecondDigits]].
      ii. Let _value_ be _localTime_.[[Millisecond]].
      iii. Set _value_ to floor(_value_ × 10^( _fractionalSecondDigits_ - 3 )).
      iv. Let _formattedValue_ be FormatNumeric(_nf3_, _value_).
      v. Append the Record { [[Type]]: *"fractionalSecond"*, [[Value]]: _formattedValue_ } to _result_.
   d. Else if _partType_ is *"dayPeriod"*, then
      i. Assert: _format_ has a field [[dayPeriod]].
      ii. Let _fieldFormat_ be _format_.[[dayPeriod]].
      iii. Let _formattedValue_ be an ILD String representing the day period of _localTime_ in the form given by _fieldFormat_.
      iv. Append the Record { [[Type]]: _partType_, [[Value]]: _formattedValue_ } to _result_.
   e. Else if _partType_ is *"timeZoneName"*, then
      i. Assert: _format_ has a field [[timeZoneName]].
      ii. Let _fieldFormat_ be _format_.[[timeZoneName]].
      iii. Let _value_ be _dateTimeFormat_.[[TimeZone]].
      iv. Let _formattedValue_ be an ILD String representing _value_ in the form given by _fieldFormat_. Its value may depend on the value of the [[InDST]] field of _localTime_ if _fieldFormat_ is *"short"*, *"long"*, *"shortOffset"*, or *"longOffset"*. If the implementation does not have such a localized representation of _fieldFormat_, then use _value_ itself.
      v. Append the Record { [[Type]]: _partType_, [[Value]]: _formattedValue_ } to _result_.
   f. Else if _partType_ matches a Property column of the row in [table-datetimeformat-components], then
      i. Assert: _format_ has a field [[<_partType_>]].
      ii. Let _fieldFormat_ be _format_.[[<_partType_>]].
      iii. Let _value_ be the value of _localTime_'s field whose name is the Internal Slot column of the matching row.
      iv. If _partType_ is *"year"* and _value_ ≤ 0, set _value_ to 1 - _value_.
      v. If _partType_ is *"month"*, set _value_ to _value_ + 1.
      vi. If _partType_ is *"hour"* and _dateTimeFormat_.[[HourCycle]] is *"h11"* or *"h12"*, then
         1. Set _value_ to _value_ modulo 12.
         2. If _value_ = 0 and _dateTimeFormat_.[[HourCycle]] is *"h12"*, set _value_ to 12.
      vii. If _partType_ is *"hour"* and _dateTimeFormat_.[[HourCycle]] is *"h24"*, then
         1. If _value_ = 0, set _value_ to 24.
      viii. If _fieldFormat_ is *"numeric"*, then
         1. Let _formattedValue_ be FormatNumeric(_nf_, _value_).
      ix. Else if _fieldFormat_ is *"2-digit"*, then
         1. Let _formattedValue_ be FormatNumeric(_nf2_, _value_).
         2. Let _codePoints_ be StringToCodePoints(_formattedValue_).
         3. Let _count_ be the number of elements in _codePoints_.
         4. If _count_ > 2, then
            a. Let _tens_ be _codePoints_[_count_ - 2].
            b. Let _ones_ be _codePoints_[_count_ - 1].
            c. Set _formattedValue_ to CodePointsToString(« _tens_, _ones_ »).
      x. Else if _fieldFormat_ is *"narrow"*, *"short"*, or *"long"*, then
         1. Let _formattedValue_ be an ILD String representing _value_ in the form given by _fieldFormat_. If _partType_ is *"month"*, then its value may depend on whether _format_ has a [[day]] field. If the implementation does not have a localized representation of _fieldFormat_, then use _value_ itself.
      xi. Append the Record { [[Type]]: _partType_, [[Value]]: _formattedValue_ } to _result_.
   g. Else if _partType_ is *"ampm"*, then
      i. Let _value_ be _localTime_.[[Hour]].
      ii. If _value_ > 11, let _formattedValue_ be an ILD String representing *"post meridiem"*; else let _formattedValue_ be an ILD String representing *"ante meridiem"*.
      iii. Append the Record { [[Type]]: *"dayPeriod"*, [[Value]]: _formattedValue_ } to _result_.
   h. Else if _partType_ is *"relatedYear"*, then
      i. Let _value_ be _localTime_.[[RelatedYear]].
      ii. Let _formattedValue_ be FormatNumeric(_nf_, _value_).
      iii. Append the Record { [[Type]]: *"relatedYear"*, [[Value]]: _formattedValue_ } to _result_.
   i. Else if _partType_ is *"yearName"*, then
      i. Let _value_ be _localTime_.[[YearName]].
      ii. Let _formattedValue_ be an ILD String representing _value_.
      iii. Append the Record { [[Type]]: *"yearName"*, [[Value]]: _formattedValue_ } to _result_.
   j. Else,
      i. Let _unknown_ be an ILND String based on _epochNanoseconds_ and _partType_.
      ii. Append the Record { [[Type]]: *"unknown"*, [[Value]]: _unknown_ } to _result_.
16. Return _result_.

NOTE:
It is recommended that implementations use the locale and calendar dependent strings provided by the Common Locale Data Repository (available at https://cldr.unicode.org/), and use CLDR *"abbreviated"* strings for DateTimeFormat *"short"* strings, and CLDR *"wide"* strings for DateTimeFormat *"long"* strings.

--- referenced: OrdinaryObjectCreate ( _proto_: an Object or *null*, optional _additionalInternalSlotsList_: a List of names of internal slots, ): an Object
  id: sec-ordinaryobjectcreate

description: It is used to specify the runtime creation of new ordinary objects. _additionalInternalSlotsList_ contains the names of additional internal slots that must be defined as part of the object, beyond [[Prototype]] and [[Extensible]]. If _additionalInternalSlotsList_ is not provided, a new empty List is used.

1. Let _internalSlotsList_ be « [[Prototype]], [[Extensible]] ».
2. If _additionalInternalSlotsList_ is present, set _internalSlotsList_ to the list-concatenation of _internalSlotsList_ and _additionalInternalSlotsList_.
3. Let _obj_ be MakeBasicObject(_internalSlotsList_).
4. Set _obj_.[[Prototype]] to _proto_.
5. Return _obj_.

NOTE:
Although OrdinaryObjectCreate does little more than call MakeBasicObject, its use communicates the intention to create an ordinary object, and not an exotic one. Thus, within this specification, it is not called by any algorithm that subsequently modifies the internal methods of the object in ways that would make the result non-ordinary. Operations that create exotic objects invoke MakeBasicObject directly.

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.

--- referenced: Construct ( _ctor_: a constructor, optional _argList_: a List of ECMAScript language values, optional _newTarget_: a constructor, ): either a normal completion containing an Object or a throw completion
  id: sec-construct

description: It is used to call the [[Construct]] internal method of a function object. _argList_ and _newTarget_ are the values to be passed as the corresponding arguments of the internal method. If _argList_ is not present, a new empty List is used as its value. If _newTarget_ is not present, _ctor_ is used as its value.

1. If _newTarget_ is not present, set _newTarget_ to _ctor_.
2. If _argList_ is not present, set _argList_ to a new empty List.
3. Return ? _ctor_.[[Construct]](_argList_, _newTarget_).

NOTE:
If _newTarget_ is not present, this operation is equivalent to: `new F(...argumentsList)`

--- referenced: ToLocalTime ( _epochNs_: a BigInt, _calendar_: a String, _timeZoneIdentifier_: a String, ): a ToLocalTime Record
  id: sec-tolocaltime

description:

1. If IsTimeZoneOffsetString(_timeZoneIdentifier_) is *true*, then
   a. Let _offsetNs_ be ParseTimeZoneOffsetString(_timeZoneIdentifier_).
2. Else,
   a. Assert: GetAvailableNamedTimeZoneIdentifier(_timeZoneIdentifier_) is not ~empty~.
   b. Let _offsetNs_ be GetNamedTimeZoneOffsetNanoseconds(_timeZoneIdentifier_, _epochNs_).
3. Let _tz_ be ℝ(_epochNs_) + _offsetNs_.
4. If _calendar_ is *"gregory"*, then
   a. Return a ToLocalTime Record with fields calculated from _tz_ according to [table-datetimeformat-tolocaltime-record].
5. Else,
   a. Return a ToLocalTime Record with the fields calculated from _tz_ for the given _calendar_. The calculations should use best available information about the specified _calendar_.

--- referenced: PartitionPattern ( _pattern_: a Pattern String, ): a List of Records with fields [[Type]] (a String) and [[Value]] (a String or *undefined*)
  id: sec-partitionpattern

description:
The [[Value]] field will be a String value if [[Type]] is *"literal"*, and *undefined* otherwise.

1. Let _result_ be a new empty List.
2. Let _placeholderEnd_ be -1.
3. Let _placeholderStart_ be StringIndexOf(_pattern_, *"{"*, 0).
4. Repeat, while _placeholderStart_ is not ~not-found~,
   a. Let _literal_ be the substring of _pattern_ from _placeholderEnd_ + 1 to _placeholderStart_.
   b. If _literal_ is not the empty String, then
      i. Append the Record { [[Type]]: *"literal"*, [[Value]]: _literal_ } to _result_.
   c. Set _placeholderEnd_ to StringIndexOf(_pattern_, *"}"*, _placeholderStart_).
   d. Assert: _placeholderEnd_ is not ~not-found~ and _placeholderStart_ < _placeholderEnd_.
   e. Let _placeholderName_ be the substring of _pattern_ from _placeholderStart_ + 1 to _placeholderEnd_.
   f. Append the Record { [[Type]]: _placeholderName_, [[Value]]: *undefined* } to _result_.
   g. Set _placeholderStart_ to StringIndexOf(_pattern_, *"{"*, _placeholderEnd_).
5. Let _tail_ be the substring of _pattern_ from _placeholderEnd_ + 1.
6. If _tail_ is not the empty String, then
   a. Append the Record { [[Type]]: *"literal"*, [[Value]]: _tail_ } to _result_.
7. Return _result_.

--- referenced: FormatNumeric ( _numberFormat_: an Intl.NumberFormat, _x_: an Intl mathematical value, ): a String
  id: sec-formatnumber

1. Let _parts_ be PartitionNumberPattern(_numberFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


PartitionDateTimePattern ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, ): either a normal completion containing a List of Records with fields [[Type]] (a String) and [[Value]] (a String), or a throw completion
  id: sec-partitiondatetimepattern   type: abstract operation

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

--- referenced: TimeClip ( _time_: a Number, ): a time value
  id: sec-timeclip

description: It calculates a number of milliseconds.

1. If _time_ is not a finite Number, return *NaN*.
2. If abs(ℝ(_time_)) > 8.64 × 10^15, return *NaN*.
3. Return 𝔽(! ToIntegerOrInfinity(_time_)).


FormatDateTimeToParts ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, ): either a normal completion containing an Array or a throw completion
  id: sec-formatdatetimetoparts   type: abstract operation

1. Let _parts_ be ? PartitionDateTimePattern(_dateTimeFormat_, _x_).
2. Let _result_ be ! ArrayCreate(0).
3. Let _n_ be 0.
4. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Let _partObj_ be OrdinaryObjectCreate(%Object.prototype%).
   b. Perform ! CreateDataPropertyOrThrow(_partObj_, *"type"*, _part_.[[Type]]).
   c. Perform ! CreateDataPropertyOrThrow(_partObj_, *"value"*, _part_.[[Value]]).
   d. Perform ! CreateDataPropertyOrThrow(_result_, ! ToString(𝔽(_n_)), _partObj_).
   e. Set _n_ to _n_ + 1.
5. Return _result_.

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

--- referenced: ArrayCreate ( _length_: a non-negative integer, optional _proto_: an Object, ): either a normal completion containing an Array exotic object or a throw completion
  id: sec-arraycreate

description: It is used to specify the creation of new Arrays.

1. If _length_ > 2^32 - 1, throw a *RangeError* exception.
2. If _proto_ is not present, set _proto_ to %Array.prototype%.
3. Let _array_ be MakeBasicObject(« [[Prototype]], [[Extensible]] »).
4. Set _array_.[[Prototype]] to _proto_.
5. Set _array_.[[DefineOwnProperty]] as specified in [sec-array-exotic-objects-defineownproperty-p-desc].
6. Perform ! OrdinaryDefineOwnProperty(_array_, *"length"*, PropertyDescriptor { [[Value]]: 𝔽(_length_), [[Writable]]: *true*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
7. Return _array_.

--- referenced: OrdinaryObjectCreate ( _proto_: an Object or *null*, optional _additionalInternalSlotsList_: a List of names of internal slots, ): an Object
  id: sec-ordinaryobjectcreate

description: It is used to specify the runtime creation of new ordinary objects. _additionalInternalSlotsList_ contains the names of additional internal slots that must be defined as part of the object, beyond [[Prototype]] and [[Extensible]]. If _additionalInternalSlotsList_ is not provided, a new empty List is used.

1. Let _internalSlotsList_ be « [[Prototype]], [[Extensible]] ».
2. If _additionalInternalSlotsList_ is present, set _internalSlotsList_ to the list-concatenation of _internalSlotsList_ and _additionalInternalSlotsList_.
3. Let _obj_ be MakeBasicObject(_internalSlotsList_).
4. Set _obj_.[[Prototype]] to _proto_.
5. Return _obj_.

NOTE:
Although OrdinaryObjectCreate does little more than call MakeBasicObject, its use communicates the intention to create an ordinary object, and not an exotic one. Thus, within this specification, it is not called by any algorithm that subsequently modifies the internal methods of the object in ways that would make the result non-ordinary. Operations that create exotic objects invoke MakeBasicObject directly.

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.

--- referenced: ToString ( _arg_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-tostring

description: It converts _arg_ to a value of type String.

1. If _arg_ is a String, return _arg_.
2. If _arg_ is a Symbol, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *"undefined"*.
4. If _arg_ is *null*, return *"null"*.
5. If _arg_ is *true*, return *"true"*.
6. If _arg_ is *false*, return *"false"*.
7. If _arg_ is a Number, return Number::toString(_arg_, 10).
8. If _arg_ is a BigInt, return BigInt::toString(_arg_, 10).
9. Assert: _arg_ is an Object.
10. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~string~).
11. Assert: _primitiveValue_ is not an Object.
12. Return ? ToString(_primitiveValue_).


get Intl.DateTimeFormat.prototype.format
  id: sec-intl.datetimeformat.prototype.format   type: built-in function

Intl.DateTimeFormat.prototype.format is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:

1. Let _dtf_ be the *this* value.
2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
   a. Set _dtf_ to ? UnwrapDateTimeFormat(_dtf_).
3. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
4. If _dtf_.[[BoundFormat]] is *undefined*, then
   a. Let _closure_ be a new Abstract Closure with parameters (_date_) that captures _dtf_ and performs the following steps when called:
      i. If _date_ is *undefined*, let _x_ be ! Call(%Date.now%, *undefined*); else let _x_ be ? ToNumber(_date_).
      ii. Return ? FormatDateTime(_dtf_, _x_).
   b. Set _dtf_.[[BoundFormat]] to CreateBuiltinFunction(_closure_, 1, *""*, « »).
5. Return _dtf_.[[BoundFormat]].

NOTE:
The returned function is bound to _dtf_ so that it can be passed directly to `Array.prototype.map` or other functions.
This is considered a historical artefact, as part of a convention which is no longer followed for new features, but is preserved to maintain compatibility with existing programs.

--- referenced: UnwrapDateTimeFormat ( _dtf_: an ECMAScript language value, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-unwrapdatetimeformat

description:
It returns the DateTimeFormat instance of its input object, which is either the value itself or a value associated with it by %Intl.DateTimeFormat% according to the normative optional constructor mode of [legacy-constructor].

1. If _dtf_ is not an Object, throw a *TypeError* exception.
2. If _dtf_ does not have an [[InitializedDateTimeFormat]] internal slot and ? OrdinaryHasInstance(%Intl.DateTimeFormat%, _dtf_) is *true*, then
   a. Return ? Get(_dtf_, %Intl%.[[FallbackSymbol]]).
3. Return _dtf_.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).

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

--- referenced: FormatDateTime ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, ): either a normal completion containing a String or a throw completion
  id: sec-formatdatetime

1. Let _parts_ be ? PartitionDateTimePattern(_dateTimeFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


Intl.DateTimeFormat.prototype.formatRange ( _startDate_, _endDate_ )
  id: sec-intl.datetimeformat.prototype.formatRange   type: built-in function

When the `formatRange` method is called with arguments _startDate_ and _endDate_, the following steps are taken:

1. Let _dtf_ be *this* value.
2. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
3. If _startDate_ is *undefined* or _endDate_ is *undefined*, throw a *TypeError* exception.
4. Let _x_ be ? ToNumber(_startDate_).
5. Let _y_ be ? ToNumber(_endDate_).
6. Return ? FormatDateTimeRange(_dtf_, _x_, _y_).

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

--- referenced: FormatDateTimeRange ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, _y_: a Number, ): either a normal completion containing a String or a throw completion
  id: sec-formatdatetimerange

1. Let _parts_ be ? PartitionDateTimeRangePattern(_dateTimeFormat_, _x_, _y_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]], [[Source]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


FormatDateTimeRange ( _dateTimeFormat_: an Intl.DateTimeFormat, _x_: a Number, _y_: a Number, ): either a normal completion containing a String or a throw completion
  id: sec-formatdatetimerange   type: abstract operation

1. Let _parts_ be ? PartitionDateTimeRangePattern(_dateTimeFormat_, _x_, _y_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]], [[Source]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


## Done when

- Every test262 run under intl402/DateTimeFormat/ passes (check with `./t262 intl402/DateTimeFormat/`). 186 pass on the main branch right now.

## Previous attempts

- attempt 546: worker reported BLOCKED but changed files: formatting (`format`, `formatToParts`, `formatRange`, `formatRangeToParts`) needs the component-options → ICU4X field-set and time zone mapping, which I did not implement.
- attempt 546: rejected (made_up_results, swallows_errors, contradicts_spec): The constructor, ChainDateTimeFormat, and the CreateDateTimeFormat option-resolution order (ResolveOptions, timeZone, component GetOptions, formatMatcher, dateStyle/timeStyle, defaults) follow the spec and keep observable Get order. Three paths are defects. (1) Steps 30.e and 31.g-h (DateTimeStyleFo Start from this diff; the constructor, ChainDateTimeFormat, OrdinaryCreateFromConstructor use, UnwrapDateTimeFormat and the option-resolution order are sound. Fix before merging: (a) Implement BasicFormatMatcher and BestFitFormatMatcher (steps 31.g-h) over a formats table derived from ICU4X, or throw agent.not_implemented naming the matcher. Do not echo the requested components as bestFormat. resolvedOptions must report the components of the pattern chosen, as the ticket says. (b) For dateStyle/timeStyle (step 30.d-e), either compute DateTimeStyleFormat or throw not_implemented. Do not return an empty record. (c) Step 14.a/15.a: read hourCycle12 and hourCycle24 from locale data (h11 vs h12 for 12-hour, h23 vs h24 for 24-hour) instead of hardcoding h12/h23. (d) Remove the .ok()? fallback in locale_hour_cycle. Propagate the failure or throw not_implemented. (e) The CALENDARS list is hardcoded and not from ICU4X. Derive it from ICU4X or throw not_implemented. (f) Verify that get_available_named_time_zone_identifier only accepts AvailableNamedTimeZoneIdentifiers, case-insensitively. IanaParserExtended may accept POSIX or GMT-style strings, which the spec rejects with RangeError. Also verify that ICU4X's canonical equals PrimaryIdentifier for links (e.g. Asia/Calcutta, Etc/UTC). (g) Remove the lossy-string conversion (agent.str_lossy) on the timeZone value if a lossless path exists. (h) Keep the component table order, the hour12 handling and the resolvedOptions key order as they are.
- attempt 560: worker reported BLOCKED but changed files: `formatToParts` and `formatRangeToParts` need `writeable` (`PartsWrite`) to read ICU4X's part types, and `writeable` is not a dependency of `kahvi-builtins`; the Cargo manifests are architect-only. Named-zone offsets need a time zone transition source, and `formatRange` needs interval patterns, which ICU4X 2.3 doesn't provide.
- attempt 560: merged partial progress. Still failing:
intl402/DateTimeFormat/constructor-options-dayPeriod-valid.js:sloppy  Uncaught InternalError: not implemented: BestFitFormatMatcher: a format with a day period and no hour
intl402/DateTimeFormat/constructor-options-dayPeriod-valid.js:strict  Uncaught InternalError: not implemented: BestFitFormatMatcher: a format with a day period and no hour
intl402/DateTimeFormat/prototype/format/dayPeriod-long-en.js:sloppy  Uncaught InternalError: not implemented: BestFitFormatMatcher: a format with a day period and no hour
intl402/DateTimeFormat/prototype/format/dayPeriod-long-en.js:strict  Uncaught InternalError: not implemented: BestFitFormatMatcher: a format with a day period and no hour
intl402/DateTimeFormat/prototype/format/dayPeriod-narrow-en.js:sloppy  Uncaught InternalError: not implemented: BestFitFormatMatcher: a format with a day period and no hour
intl402/DateTimeFormat/prototype/format/dayPeriod-narrow-en.js:strict  Uncaught InternalError: not implemented: BestFitFormatMatcher: a format with a day period and no hour
intl402/DateTimeFormat/prototype/format/dayPeriod-short-en.js:sloppy  Uncaught InternalError: not implemented: BestFitFormatMatcher: a format with a day period and no hour
intl402/DateTimeFormat/prototype/format/dayPeriod-short-en.js:strict  Uncaught InternalError: not implemented: BestFitFormatMatcher: a format with a day period and no hour
intl402/DateTimeFormat/prototype/format/fractionalSecondDigits.js:sloppy  Uncaught Test262Error: no fractionalSecondDigits Expected SameValue(«"1:02:03 AM"», «"02:03"») to be true
intl402/DateTimeFormat/prototype/format/fractionalSecondDigits.js:strict  Uncaught Test262Error: no fractionalSecondDigits Expected SameValue(«"1:02:03 AM"», «"02:03"») to be true

