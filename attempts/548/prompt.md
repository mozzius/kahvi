# Ticket I-collator: Intl.Collator

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

- `crates/kahvi-builtins/src/intl_collator/*.rs`
- `crates/kahvi-builtins/src/support/intl_collator.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs`
- `crates/kahvi-builtins/src/disposable_stack/constructor.rs`

## Spec

Intl.Collator ( [ _locales_ [ , _options_ ] ] )
  id: sec-intl.collator   type: built-in function

When the `Intl.Collator` function is called with optional arguments _locales_ and _options_, the following steps are taken:

1. If NewTarget is *undefined*, let _newTarget_ be the active function object, else let _newTarget_ be NewTarget.
2. Let _internalSlotsList_ be « [[InitializedCollator]], [[Locale]], [[Usage]], [[Collation]], [[Numeric]], [[CaseFirst]], [[Sensitivity]], [[IgnorePunctuation]], [[BoundCompare]] ».
3. Let _collator_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%Intl.Collator.prototype%"*, _internalSlotsList_).
4. NOTE: The source of locale data for ResolveOptions depends upon the *"usage"* property of _options_, but the following two steps must observably precede that lookup (and must not observably repeat inside ResolveOptions).
5. Let _requestedLocales_ be ? CanonicalizeLocaleList(_locales_).
6. Set _options_ to ? CoerceOptionsToObject(_options_).
7. Let _usage_ be ? GetOption(_options_, *"usage"*, ~string~, « *"sort"*, *"search"* », *"sort"*).
8. Set _collator_.[[Usage]] to _usage_.
9. If _usage_ is *"sort"*, then
   a. Let _localeData_ be %Intl.Collator%.[[SortLocaleData]].
10. Else,
   a. Let _localeData_ be %Intl.Collator%.[[SearchLocaleData]].
11. Let _optionsResolution_ be ? ResolveOptions(%Intl.Collator%, _localeData_, CreateArrayFromList(_requestedLocales_), _options_).
12. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
13. Set _collator_.[[Locale]] to _resolvedLocale_.[[Locale]].
14. If _resolvedLocale_.[[co]] is *null*, let _collation_ be *"default"*. Otherwise, let _collation_ be _resolvedLocale_.[[co]].
15. Set _collator_.[[Collation]] to _collation_.
16. Set _collator_.[[Numeric]] to SameValue(_resolvedLocale_.[[kn]], *"true"*).
17. Set _collator_.[[CaseFirst]] to _resolvedLocale_.[[kf]].
18. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
19. If _usage_ is *"sort"*, let _defaultSensitivity_ be *"variant"*. Otherwise, let _defaultSensitivity_ be _resolvedLocaleData_.[[sensitivity]].
20. Set _collator_.[[Sensitivity]] to ? GetOption(_options_, *"sensitivity"*, ~string~, « *"base"*, *"accent"*, *"case"*, *"variant"* », _defaultSensitivity_).
21. Let _defaultIgnorePunctuation_ be _resolvedLocaleData_.[[ignorePunctuation]].
22. Set _collator_.[[IgnorePunctuation]] to ? GetOption(_options_, *"ignorePunctuation"*, ~boolean~, ~empty~, _defaultIgnorePunctuation_).
23. Return _collator_.

--- referenced: OrdinaryCreateFromConstructor ( _ctor_: a function object, _intrinsicDefaultProto_: a String, optional _internalSlotsList_: a List of names of internal slots, ): either a normal completion containing an Object or a throw completion
  id: sec-ordinarycreatefromconstructor

description: It creates an ordinary object whose [[Prototype]] value is retrieved from a constructor's *"prototype"* property, if it exists. Otherwise the intrinsic named by _intrinsicDefaultProto_ is used for [[Prototype]]. _internalSlotsList_ contains the names of additional internal slots that must be defined as part of the object. If _internalSlotsList_ is not provided, a new empty List is used.

1. Assert: _intrinsicDefaultProto_ is this specification's name of an intrinsic object. The corresponding object must be an intrinsic that is intended to be used as the [[Prototype]] value of an object.
2. Let _proto_ be ? GetPrototypeFromConstructor(_ctor_, _intrinsicDefaultProto_).
3. If _internalSlotsList_ is present, let _slots_ be _internalSlotsList_.
4. Else, let _slots_ be a new empty List.
5. Return OrdinaryObjectCreate(_proto_, _slots_).

--- referenced: CoerceOptionsToObject ( _options_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-coerceoptionstoobject

description:
It coerces _options_ into an Object suitable for use with GetOption, defaulting to an empty Object.
Because it coerces non-null primitive values into objects, its use is discouraged for new functionality in favour of GetOptionsObject.

1. If _options_ is *undefined*, then
   a. Return OrdinaryObjectCreate(*null*).
2. Return ? ToObject(_options_).

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

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.

--- referenced: SameValue ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, ): a Boolean
  id: sec-samevalue

description: It determines whether or not the two arguments are the same value.

1. If SameType(_x_, _y_) is *false*, return *false*.
2. If _x_ is a Number, then
   a. Return Number::sameValue(_x_, _y_).
3. Return SameValueNonNumber(_x_, _y_).

NOTE:
This algorithm differs from the IsStrictlyEqual Algorithm by treating all *NaN* values as equivalent and by differentiating *+0*_𝔽 from *-0*_𝔽.


CompareStrings ( _collator_: an Intl.Collator, _x_: a String, _y_: a String, ): a Number, but not *NaN*
  id: sec-collator-comparestrings   type: implementation-defined abstract operation

description:
The returned Number represents the result of an implementation-defined locale-sensitive String comparison of _x_ with _y_.
The result is intended to correspond with a sort order of String values according to the effective locale and collation options of _collator_, and will be negative when _x_ is ordered before _y_, positive when _x_ is ordered after _y_, and zero in all other cases (representing no relative ordering between _x_ and _y_).
String values must be interpreted as UTF-16 code unit sequences as described in ECMA-262, [sec-ecmascript-language-types-string-type], and a surrogate pair (a code unit in the range 0xD800 to 0xDBFF followed by a code unit in the range 0xDC00 to 0xDFFF) within a string must be interpreted as the corresponding code point.
Behaviour as described below depends upon locale-sensitive identification of the sequence of collation elements for a string, in particular "base letters", and different base letters always compare as unequal (causing the strings containing them to also compare as unequal). Results of comparing variations of the same base letter with different case, diacritic marks, or potentially other aspects further depends upon _collator_.[[Sensitivity]] as follows:
Effects of Collator Sensitivity
[[Sensitivity]] |
Description |
*"a"* vs. *"á"* |
*"a"* vs. *"A"* |
*"base"* |
Characters with the same base letter do not compare as unequal, regardless of differences in case and/or diacritic marks. |
equal |
equal |
*"accent"* |
Characters with the same base letter compare as unequal only if they differ in accents and/or other diacritic marks, regardless of differences in case. |
not equal |
equal |
*"case"* |
Characters with the same base letter compare as unequal only if they differ in case, regardless of differences in accents and/or other diacritic marks. |
equal |
not equal |
*"variant"* |
Characters with the same base letter compare as unequal if they differ in case, diacritic marks, and/or potentially other differences. |
not equal |
not equal |
NOTE:
The mapping from input code points to base letters can include arbitrary contractions, expansions, and collisions, including those that apply special treatment to certain characters with diacritic marks. For example, in Swedish, "ö" is a base letter that differs from "o", and "v" and "w" are considered to be the same base letter. In Slovak, "ch" is a single base letter, and in English, "æ" is a sequence of base letters starting with "a" and ending with "e".
If _collator_.[[IgnorePunctuation]] is *true*, then punctuation is ignored (e.g., strings that differ only in punctuation compare as equal).
For the interpretation of options settable through locale extension keys, see Unicode Technical Standard #35 Part 1 Core, Section 3.6.1 Key and Type Definitions.
The actual return values are implementation-defined to permit encoding additional information in them, but this operation for any given _collator_, when considered as a function of _x_ and _y_, is required to be a consistent comparator defining a total ordering on the set of all Strings. This operation is also required to recognize and honour canonical equivalence according to the Unicode Standard, including returning *+0*_𝔽 when comparing distinguishable Strings that are canonically equivalent.
NOTE:
It is recommended that the CompareStrings abstract operation be implemented following Unicode Technical Standard #10: Unicode Collation Algorithm, using tailorings for the effective locale and collation options of _collator_. It is recommended that implementations use the tailorings provided by the Common Locale Data Repository (available at https://cldr.unicode.org/).
NOTE:
Applications should not assume that the behaviour of the CompareStrings abstract operation for Collator instances with the same resolved options will remain the same for different versions of the same implementation.


## Done when

- Every test262 run under intl402/Collator/ passes (check with `./t262 intl402/Collator/`). 0 pass on the main branch right now.

## Previous attempts

- attempt 544: worker reported BLOCKED but changed files: ICU4X's compiled data has no search collations (`usage: "search"` for `de`), and `Intl.NumberFormat` is not implemented yet.
- attempt 544: rejected by gates: files outside scope: crates/kahvi-cli/src/main.rs

