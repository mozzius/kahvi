# Ticket I-number-format: Intl.NumberFormat

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
- SetNumberFormatDigitOptions is shared with Intl.PluralRules: make it
  `pub(crate)` in `support/intl_number_format.rs`.
- Intl.NumberFormat called without `new` on an Intl object uses
  `support::intl::fallback_symbol` (ChainNumberFormat).

## Files you may change

- `crates/kahvi-builtins/src/intl_number_format/*.rs`
- `crates/kahvi-builtins/src/support/intl_number_format.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs`
- `crates/kahvi-builtins/src/disposable_stack/constructor.rs`

## Spec

Intl.NumberFormat ( [ _locales_ [ , _options_ ] ] )
  id: sec-intl.numberformat   type: built-in function

When the `Intl.NumberFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:

1. If NewTarget is *undefined*, let _newTarget_ be the active function object, else let _newTarget_ be NewTarget.
2. Let _numberFormat_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%Intl.NumberFormat.prototype%"*, « [[InitializedNumberFormat]], [[Locale]], [[LocaleData]], [[NumberingSystem]], [[Style]], [[Unit]], [[UnitDisplay]], [[Currency]], [[CurrencyDisplay]], [[CurrencySign]], [[MinimumIntegerDigits]], [[MinimumFractionDigits]], [[MaximumFractionDigits]], [[MinimumSignificantDigits]], [[MaximumSignificantDigits]], [[RoundingType]], [[Notation]], [[CompactDisplay]], [[UseGrouping]], [[SignDisplay]], [[RoundingIncrement]], [[RoundingMode]], [[ComputedRoundingPriority]], [[TrailingZeroDisplay]], [[BoundFormat]] »).
3. Let _optionsResolution_ be ? ResolveOptions(%Intl.NumberFormat%, %Intl.NumberFormat%.[[LocaleData]], _locales_, _options_, « ~coerce-options~ »).
4. Set _options_ to _optionsResolution_.[[Options]].
5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
6. Set _numberFormat_.[[Locale]] to _resolvedLocale_.[[Locale]].
7. Set _numberFormat_.[[LocaleData]] to _resolvedLocale_.[[LocaleData]].
8. Set _numberFormat_.[[NumberingSystem]] to _resolvedLocale_.[[nu]].
9. Perform ? SetNumberFormatUnitOptions(_numberFormat_, _options_).
10. Let _style_ be _numberFormat_.[[Style]].
11. Let _notation_ be ? GetOption(_options_, *"notation"*, ~string~, « *"standard"*, *"scientific"*, *"engineering"*, *"compact"* », *"standard"*).
12. Set _numberFormat_.[[Notation]] to _notation_.
13. If _style_ is *"currency"* and _notation_ is *"standard"*, then
   a. Let _currency_ be _numberFormat_.[[Currency]].
   b. Let _cDigits_ be CurrencyDigits(_currency_).
   c. Let _mnfdDefault_ be _cDigits_.
   d. Let _mxfdDefault_ be _cDigits_.
14. Else,
   a. Let _mnfdDefault_ be 0.
   b. If _style_ is *"percent"*, then
      i. Let _mxfdDefault_ be 0.
   c. Else,
      i. Let _mxfdDefault_ be 3.
15. Perform ? SetNumberFormatDigitOptions(_numberFormat_, _options_, _mnfdDefault_, _mxfdDefault_, _notation_).
16. Let _compactDisplay_ be ? GetOption(_options_, *"compactDisplay"*, ~string~, « *"short"*, *"long"* », *"short"*).
17. Let _defaultUseGrouping_ be *"auto"*.
18. If _notation_ is *"compact"*, then
   a. Set _numberFormat_.[[CompactDisplay]] to _compactDisplay_.
   b. Set _defaultUseGrouping_ to *"min2"*.
19. NOTE: For historical reasons, the strings *"true"* and *"false"* are accepted and replaced with the default value.
20. Let _useGrouping_ be ? GetBooleanOrStringNumberFormatOption(_options_, *"useGrouping"*, « *"min2"*, *"auto"*, *"always"*, *"true"*, *"false"* », _defaultUseGrouping_).
21. If _useGrouping_ is *"true"* or _useGrouping_ is *"false"*, set _useGrouping_ to _defaultUseGrouping_.
22. If _useGrouping_ is *true*, set _useGrouping_ to *"always"*.
23. Set _numberFormat_.[[UseGrouping]] to _useGrouping_.
24. Let _signDisplay_ be ? GetOption(_options_, *"signDisplay"*, ~string~, « *"auto"*, *"never"*, *"always"*, *"exceptZero"*, *"negative"* », *"auto"*).
25. Set _numberFormat_.[[SignDisplay]] to _signDisplay_.
26. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
   a. Let _this_ be the *this* value.
   b. Return ? ChainNumberFormat(_numberFormat_, NewTarget, _this_).
27. Return _numberFormat_.

[subclause sec-chainnumberformat: ChainNumberFormat (
_numberFormat_: an Intl.NumberFormat,
_newTarget_: an ECMAScript language value,
_this_: an ECMAScript language value,
): either a normal completion containing an Object or a throw completion]

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

--- referenced: CurrencyDigits ( _currency_: a String, ): a non-negative integer
  id: sec-currencydigits

description:

1. Assert: IsWellFormedCurrencyCode(_currency_) is *true*.
2. Return a non-negative integer indicating the number of fractional digits used when formatting quantities of the currency corresponding to _currency_. If there is no available information on the number of digits to be used, return 2.

--- referenced: GetBooleanOrStringNumberFormatOption ( _options_: an Object, _propertyKey_: a property key, _stringValues_: a List of Strings, _fallback_: an ECMAScript language value, ): either a normal completion containing either a Boolean, String, or _fallback_, or a throw completion
  id: sec-getbooleanorstringnumberformatoption

description: It extracts the value of the property named _propertyKey_ from the provided _options_ object. It returns _fallback_ if that value is *undefined*, *true* if that value is *true*, *false* if that value coerces to *false*, and otherwise coerces it to a String and returns the result if it is allowed by _stringValues_.

1. Let _value_ be ? Get(_options_, _propertyKey_).
2. If _value_ is *undefined*, return _fallback_.
3. If _value_ is *true*, return *true*.
4. If ToBoolean(_value_) is *false*, return *false*.
5. Set _value_ to ? ToString(_value_).
6. If _stringValues_ does not contain _value_, throw a *RangeError* exception.
7. Return _value_.

--- referenced: ChainNumberFormat ( _numberFormat_: an Intl.NumberFormat, _newTarget_: an ECMAScript language value, _this_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-chainnumberformat

1. If _newTarget_ is *undefined* and ? OrdinaryHasInstance(%Intl.NumberFormat%, _this_) is *true*, then
   a. Perform ? DefinePropertyOrThrow(_this_, %Intl%.[[FallbackSymbol]], PropertyDescriptor { [[Value]]: _numberFormat_, [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
   b. Return _this_.
2. Return _numberFormat_.


ChainNumberFormat ( _numberFormat_: an Intl.NumberFormat, _newTarget_: an ECMAScript language value, _this_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-chainnumberformat   type: abstract operation

1. If _newTarget_ is *undefined* and ? OrdinaryHasInstance(%Intl.NumberFormat%, _this_) is *true*, then
   a. Perform ? DefinePropertyOrThrow(_this_, %Intl%.[[FallbackSymbol]], PropertyDescriptor { [[Value]]: _numberFormat_, [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
   b. Return _this_.
2. Return _numberFormat_.

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


SetNumberFormatDigitOptions ( _intlObj_: an Object, _options_: an Object, _mnfdDefault_: an integer, _mxfdDefault_: an integer, _notation_: a String, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-setnumberformatdigitoptions   type: abstract operation

description: It populates the internal slots of _intlObj_ that affect locale-independent number rounding (see [sec-formatnumerictostring]).

1. Let _mnid_ be ? GetNumberOption(_options_, *"minimumIntegerDigits"*, 1, 21, 1).
2. Let _mnfd_ be ? Get(_options_, *"minimumFractionDigits"*).
3. Let _mxfd_ be ? Get(_options_, *"maximumFractionDigits"*).
4. Let _mnsd_ be ? Get(_options_, *"minimumSignificantDigits"*).
5. Let _mxsd_ be ? Get(_options_, *"maximumSignificantDigits"*).
6. Set _intlObj_.[[MinimumIntegerDigits]] to _mnid_.
7. Let _roundingIncrement_ be ? GetNumberOption(_options_, *"roundingIncrement"*, 1, 5000, 1).
8. If _roundingIncrement_ is not in « 1, 2, 5, 10, 20, 25, 50, 100, 200, 250, 500, 1000, 2000, 2500, 5000 », throw a *RangeError* exception.
9. Let _roundingMode_ be ? GetOption(_options_, *"roundingMode"*, ~string~, « *"ceil"*, *"floor"*, *"expand"*, *"trunc"*, *"halfCeil"*, *"halfFloor"*, *"halfExpand"*, *"halfTrunc"*, *"halfEven"* », *"halfExpand"*).
10. Let _roundingPriority_ be ? GetOption(_options_, *"roundingPriority"*, ~string~, « *"auto"*, *"morePrecision"*, *"lessPrecision"* », *"auto"*).
11. Let _trailingZeroDisplay_ be ? GetOption(_options_, *"trailingZeroDisplay"*, ~string~, « *"auto"*, *"stripIfInteger"* », *"auto"*).
12. NOTE: All fields required by SetNumberFormatDigitOptions have now been read from _options_. The remainder of this AO interprets the options and may throw exceptions.
13. If _roundingIncrement_ is not 1, set _mxfdDefault_ to _mnfdDefault_.
14. Set _intlObj_.[[RoundingIncrement]] to _roundingIncrement_.
15. Set _intlObj_.[[RoundingMode]] to _roundingMode_.
16. Set _intlObj_.[[TrailingZeroDisplay]] to _trailingZeroDisplay_.
17. If _mnsd_ is *undefined* and _mxsd_ is *undefined*, let _hasSd_ be *false*. Otherwise, let _hasSd_ be *true*.
18. If _mnfd_ is *undefined* and _mxfd_ is *undefined*, let _hasFd_ be *false*. Otherwise, let _hasFd_ be *true*.
19. Let _needSd_ be *true*.
20. Let _needFd_ be *true*.
21. If _roundingPriority_ is *"auto"*, then
   a. Set _needSd_ to _hasSd_.
   b. If _needSd_ is *true*, or _hasFd_ is *false* and _notation_ is *"compact"*, then
      i. Set _needFd_ to *false*.
22. If _needSd_ is *true*, then
   a. If _hasSd_ is *true*, then
      i. Set _intlObj_.[[MinimumSignificantDigits]] to ? DefaultNumberOption(_mnsd_, 1, 21, 1).
      ii. Set _intlObj_.[[MaximumSignificantDigits]] to ? DefaultNumberOption(_mxsd_, _intlObj_.[[MinimumSignificantDigits]], 21, 21).
   b. Else,
      i. Set _intlObj_.[[MinimumSignificantDigits]] to 1.
      ii. Set _intlObj_.[[MaximumSignificantDigits]] to 21.
23. If _needFd_ is *true*, then
   a. If _hasFd_ is *true*, then
      i. Set _mnfd_ to ? DefaultNumberOption(_mnfd_, 0, 100, *undefined*).
      ii. Set _mxfd_ to ? DefaultNumberOption(_mxfd_, 0, 100, *undefined*).
      iii. If _mnfd_ is *undefined*, set _mnfd_ to min(_mnfdDefault_, _mxfd_).
      iv. Else if _mxfd_ is *undefined*, set _mxfd_ to max(_mxfdDefault_, _mnfd_).
      v. Else if _mnfd_ is greater than _mxfd_, throw a *RangeError* exception.
      vi. Set _intlObj_.[[MinimumFractionDigits]] to _mnfd_.
      vii. Set _intlObj_.[[MaximumFractionDigits]] to _mxfd_.
   b. Else,
      i. Set _intlObj_.[[MinimumFractionDigits]] to _mnfdDefault_.
      ii. Set _intlObj_.[[MaximumFractionDigits]] to _mxfdDefault_.
24. If _needSd_ is *false* and _needFd_ is *false*, then
   a. Set _intlObj_.[[MinimumFractionDigits]] to 0.
   b. Set _intlObj_.[[MaximumFractionDigits]] to 0.
   c. Set _intlObj_.[[MinimumSignificantDigits]] to 1.
   d. Set _intlObj_.[[MaximumSignificantDigits]] to 2.
   e. Set _intlObj_.[[RoundingType]] to ~more-precision~.
   f. Set _intlObj_.[[ComputedRoundingPriority]] to *"morePrecision"*.
25. Else if _roundingPriority_ is *"morePrecision"*, then
   a. Set _intlObj_.[[RoundingType]] to ~more-precision~.
   b. Set _intlObj_.[[ComputedRoundingPriority]] to *"morePrecision"*.
26. Else if _roundingPriority_ is *"lessPrecision"*, then
   a. Set _intlObj_.[[RoundingType]] to ~less-precision~.
   b. Set _intlObj_.[[ComputedRoundingPriority]] to *"lessPrecision"*.
27. Else if _hasSd_ is *true*, then
   a. Set _intlObj_.[[RoundingType]] to ~significant-digits~.
   b. Set _intlObj_.[[ComputedRoundingPriority]] to *"auto"*.
28. Else,
   a. Set _intlObj_.[[RoundingType]] to ~fraction-digits~.
   b. Set _intlObj_.[[ComputedRoundingPriority]] to *"auto"*.
29. If _roundingIncrement_ is not 1, then
   a. If _intlObj_.[[RoundingType]] is not ~fraction-digits~, throw a *TypeError* exception.
   b. If _intlObj_.[[MaximumFractionDigits]] is not _intlObj_.[[MinimumFractionDigits]], throw a *RangeError* exception.
30. Return ~unused~.

--- referenced: GetNumberOption ( _options_: an Object, _propertyKey_: a String, _minimum_: an integer, _maximum_: an integer, _fallback_: an integer or *undefined*, ): either a normal completion containing either an integer or *undefined*, or a throw completion
  id: sec-getnumberoption

description: It extracts the value of the property named _propertyKey_ from the provided _options_ object, converts it to an integer, checks whether it is in the allowed range, and fills in a _fallback_ value if necessary.

1. Let _value_ be ? Get(_options_, _propertyKey_).
2. Return ? DefaultNumberOption(_value_, _minimum_, _maximum_, _fallback_).

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

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

--- referenced: DefaultNumberOption ( _value_: an ECMAScript language value, _minimum_: an integer, _maximum_: an integer, _fallback_: an integer or *undefined*, ): either a normal completion containing either an integer or *undefined*, or a throw completion
  id: sec-defaultnumberoption

description: It converts _value_ to an integer, checks whether it is in the allowed range, and fills in a _fallback_ value if necessary.

1. If _value_ is *undefined*, return _fallback_.
2. Set _value_ to ? ToNumber(_value_).
3. If _value_ is not a finite Number or ℝ(_value_) < _minimum_ or ℝ(_value_) > _maximum_, throw a *RangeError* exception.
4. Return floor(ℝ(_value_)).


FormatNumeric ( _numberFormat_: an Intl.NumberFormat, _x_: an Intl mathematical value, ): a String
  id: sec-formatnumber   type: abstract operation

1. Let _parts_ be PartitionNumberPattern(_numberFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


## Done when

- Every test262 run under intl402/NumberFormat/ passes (check with `./t262 intl402/NumberFormat/`). 0 pass on the main branch right now.

