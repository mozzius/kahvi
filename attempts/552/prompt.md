# Ticket I-display-names: Intl.DisplayNames

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

## Files you may change

- `crates/kahvi-builtins/src/intl_display_names/*.rs`
- `crates/kahvi-builtins/src/support/intl_display_names.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs`
- `crates/kahvi-builtins/src/disposable_stack/constructor.rs`

## Spec

Intl.DisplayNames ( _locales_, _options_ )
  id: sec-Intl.DisplayNames   type: built-in function

When the `Intl.DisplayNames` function is called with arguments _locales_ and _options_, the following steps are taken:

1. If NewTarget is *undefined*, throw a *TypeError* exception.
2. Let _displayNames_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.DisplayNames.prototype%"*, « [[InitializedDisplayNames]], [[Locale]], [[Style]], [[Type]], [[Fallback]], [[LanguageDisplay]], [[Fields]] »).
3. Let _optionsResolution_ be ? ResolveOptions(%Intl.DisplayNames%, %Intl.DisplayNames%.[[LocaleData]], _locales_, _options_, « ~require-options~ »).
4. Set _options_ to _optionsResolution_.[[Options]].
5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
6. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"narrow"*, *"short"*, *"long"* », *"long"*).
7. Set _displayNames_.[[Style]] to _style_.
8. Let _type_ be ? GetOption(_options_, *"type"*, ~string~, « *"language"*, *"region"*, *"script"*, *"currency"*, *"calendar"*, *"dateTimeField"* », ~required~).
9. Set _displayNames_.[[Type]] to _type_.
10. Let _fallback_ be ? GetOption(_options_, *"fallback"*, ~string~, « *"code"*, *"none"* », *"code"*).
11. Set _displayNames_.[[Fallback]] to _fallback_.
12. Set _displayNames_.[[Locale]] to _resolvedLocale_.[[Locale]].
13. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
14. Let _types_ be _resolvedLocaleData_.[[types]].
15. Assert: _types_ is a Record (see [sec-Intl.DisplayNames-internal-slots]).
16. Let _languageDisplay_ be ? GetOption(_options_, *"languageDisplay"*, ~string~, « *"dialect"*, *"standard"* », *"dialect"*).
17. Let _typeFields_ be _types_.[[<_type_>]].
18. Assert: _typeFields_ is a Record (see [sec-Intl.DisplayNames-internal-slots]).
19. If _type_ is *"language"*, then
   a. Set _displayNames_.[[LanguageDisplay]] to _languageDisplay_.
   b. Set _typeFields_ to _typeFields_.[[<_languageDisplay_>]].
   c. Assert: _typeFields_ is a Record (see [sec-Intl.DisplayNames-internal-slots]).
20. Let _styleFields_ be _typeFields_.[[<_style_>]].
21. Assert: _styleFields_ is a Record (see [sec-Intl.DisplayNames-internal-slots]).
22. Set _displayNames_.[[Fields]] to _styleFields_.
23. Return _displayNames_.

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


CanonicalCodeForDisplayNames ( _type_: a String, _code_: a String, ): either a normal completion containing a String or a throw completion
  id: sec-canonicalcodefordisplaynames   type: abstract operation

description: It verifies that _code_ represents a well-formed code according to _type_ and returns the case-regularized form of _code_.

1. If _type_ is *"language"*, then
   a. If _code_ cannot be matched by the unicode_language_id Unicode locale nonterminal, throw a *RangeError* exception.
   b. If IsWellFormedLanguageTag(_code_) is *false*, throw a *RangeError* exception.
   c. Return CanonicalizeUnicodeLocaleId(_code_).
2. If _type_ is *"region"*, then
   a. If _code_ cannot be matched by the unicode_region_subtag Unicode locale nonterminal, throw a *RangeError* exception.
   b. Return the ASCII-uppercase of _code_.
3. If _type_ is *"script"*, then
   a. If _code_ cannot be matched by the unicode_script_subtag Unicode locale nonterminal, throw a *RangeError* exception.
   b. Assert: The length of _code_ is 4, and every code unit of _code_ represents an ASCII letter (0x0041 through 0x005A and 0x0061 through 0x007A, both inclusive).
   c. Let _first_ be the ASCII-uppercase of the substring of _code_ from 0 to 1.
   d. Let _rest_ be the ASCII-lowercase of the substring of _code_ from 1.
   e. Return the string-concatenation of _first_ and _rest_.
4. If _type_ is *"calendar"*, then
   a. If _code_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
   b. If _code_ uses any of the backwards compatibility syntax described in Unicode Technical Standard #35 Part 1 Core, Section 3.3 BCP 47 Conformance, throw a *RangeError* exception.
   c. Return the ASCII-lowercase of _code_.
5. If _type_ is *"dateTimeField"*, then
   a. If the result of IsValidDateTimeFieldCode(_code_) is *false*, throw a *RangeError* exception.
   b. Return _code_.
6. Assert: _type_ is *"currency"*.
7. If IsWellFormedCurrencyCode(_code_) is *false*, throw a *RangeError* exception.
8. Return the ASCII-uppercase of _code_.

--- referenced: IsValidDateTimeFieldCode ( _field_: a String, ): a Boolean
  id: sec-isvaliddatetimefieldcode

description: It verifies that the _field_ argument represents a valid date time field code.

1. If _field_ is listed in the Code column of [table-validcodefordatetimefield], return *true*.
2. Return *false*.

Codes For Date Time Field of DisplayNames
Code |
Description |
*"era"* |
The field indicating the era, e.g. AD or BC in the Gregorian or Julian calendar. |
*"year"* |
The field indicating the year (within an era). |
*"quarter"* |
The field indicating the quarter, e.g. Q2, 2nd quarter, etc. |
*"month"* |
The field indicating the month, e.g. Sep, September, etc. |
*"weekOfYear"* |
The field indicating the week number within a year. |
*"weekday"* |
The field indicating the day of week, e.g. Tue, Tuesday, etc. |
*"day"* |
The field indicating the day in month. |
*"dayPeriod"* |
The field indicating the day period, either am, pm, etc. or noon, evening, etc.. |
*"hour"* |
The field indicating the hour. |
*"minute"* |
The field indicating the minute. |
*"second"* |
The field indicating the second. |
*"timeZoneName"* |
The field indicating the time zone name, e.g. PDT, Pacific Daylight Time, etc. |

--- referenced: IsWellFormedCurrencyCode ( _currency_: a String, ): a Boolean
  id: sec-iswellformedcurrencycode

description: It verifies that the _currency_ argument represents a well-formed 3-letter ISO 4217 currency code.

1. If the length of _currency_ is not 3, return *false*.
2. Let _normalized_ be the ASCII-uppercase of _currency_.
3. If _normalized_ contains any code unit outside of 0x0041 through 0x005A (corresponding to Unicode characters LATIN CAPITAL LETTER A through LATIN CAPITAL LETTER Z), return *false*.
4. Return *true*.


## Done when

- Every test262 run under intl402/DisplayNames/ passes (check with `./t262 intl402/DisplayNames/`). 0 pass on the main branch right now.

