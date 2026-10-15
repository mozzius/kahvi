# Ticket I-locale: Intl.Locale

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
- The internal slots are `support::intl::IntlLocaleSlots` (read-only
  here: report a missing field rather than adding one).

## Files you may change

- `crates/kahvi-builtins/src/intl_locale/*.rs`
- `crates/kahvi-builtins/src/support/intl_locale.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs`
- `crates/kahvi-builtins/src/disposable_stack/constructor.rs`

## Spec

Intl.Locale ( _tag_ [ , _options_ ] )
  id: sec-Intl.Locale   type: built-in function

When the `Intl.Locale` function is called with an argument _tag_ and an optional argument _options_, the following steps are taken:

1. If NewTarget is *undefined*, throw a *TypeError* exception.
2. Let _localeExtensionKeys_ be %Intl.Locale%.[[LocaleExtensionKeys]].
3. Let _internalSlotsList_ be « [[InitializedLocale]], [[Locale]], [[Calendar]], [[Collation]], [[FirstDayOfWeek]], [[HourCycle]], [[NumberingSystem]] ».
4. If _localeExtensionKeys_ contains *"kf"*, then
   a. Append [[CaseFirst]] to _internalSlotsList_.
5. If _localeExtensionKeys_ contains *"kn"*, then
   a. Append [[Numeric]] to _internalSlotsList_.
6. Let _locale_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.Locale.prototype%"*, _internalSlotsList_).
7. If _tag_ is not a String and _tag_ is not an Object, throw a *TypeError* exception.
8. If _tag_ is an Object and _tag_ has an [[InitializedLocale]] internal slot, then
   a. Set _tag_ to _tag_.[[Locale]].
9. Else,
   a. Set _tag_ to ? ToString(_tag_).
10. Set _options_ to ? CoerceOptionsToObject(_options_).
11. If IsWellFormedLanguageTag(_tag_) is *false*, throw a *RangeError* exception.
12. NOTE: Because LanguageId canonicalization can alter _tag_ in arbitrary ways according to Alias Rules from supplementalMetadata.xml, it is necessary to perform such canonicalization before applying overrides from _options_.
13. Set _tag_ to CanonicalizeUnicodeLocaleId(_tag_).
14. Set _tag_ to ? UpdateLanguageId(_tag_, _options_).
15. Let _opt_ be a new Record.
16. Let _calendar_ be ? GetOption(_options_, *"calendar"*, ~string~, ~empty~, *undefined*).
17. If _calendar_ is not *undefined*, then
   a. If _calendar_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
18. Set _opt_.[[ca]] to _calendar_.
19. Let _collation_ be ? GetOption(_options_, *"collation"*, ~string~, ~empty~, *undefined*).
20. If _collation_ is not *undefined*, then
   a. If _collation_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
21. Set _opt_.[[co]] to _collation_.
22. Let _fw_ be ? GetOption(_options_, *"firstDayOfWeek"*, ~string~, ~empty~, *undefined*).
23. If _fw_ is not *undefined*, then
   a. Set _fw_ to WeekdayToUValue(_fw_).
   b. If _fw_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
24. Set _opt_.[[fw]] to _fw_.
25. Let _hc_ be ? GetOption(_options_, *"hourCycle"*, ~string~, « *"h11"*, *"h12"*, *"h23"*, *"h24"* », *undefined*).
26. Set _opt_.[[hc]] to _hc_.
27. Let _kf_ be ? GetOption(_options_, *"caseFirst"*, ~string~, « *"upper"*, *"lower"*, *"false"* », *undefined*).
28. Set _opt_.[[kf]] to _kf_.
29. Let _kn_ be ? GetOption(_options_, *"numeric"*, ~boolean~, ~empty~, *undefined*).
30. If _kn_ is not *undefined*, set _kn_ to ! ToString(_kn_).
31. Set _opt_.[[kn]] to _kn_.
32. Let _numberingSystem_ be ? GetOption(_options_, *"numberingSystem"*, ~string~, ~empty~, *undefined*).
33. If _numberingSystem_ is not *undefined*, then
   a. If _numberingSystem_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
34. Set _opt_.[[nu]] to _numberingSystem_.
35. Let _localeRecord_ be MakeLocaleRecord(_tag_, _opt_, _localeExtensionKeys_).
36. Set _locale_.[[Locale]] to _localeRecord_.[[locale]].
37. Set _locale_.[[Calendar]] to _localeRecord_.[[ca]].
38. Set _locale_.[[Collation]] to _localeRecord_.[[co]].
39. Set _locale_.[[FirstDayOfWeek]] to _localeRecord_.[[fw]].
40. Set _locale_.[[HourCycle]] to _localeRecord_.[[hc]].
41. If _localeExtensionKeys_ contains *"kf"*, then
   a. Set _locale_.[[CaseFirst]] to _localeRecord_.[[kf]].
42. If _localeExtensionKeys_ contains *"kn"*, then
   a. If _localeRecord_.[[kn]] is either *"true"* or the empty String, then
      i. Set _locale_.[[Numeric]] to *true*.
   b. Else,
      i. Set _locale_.[[Numeric]] to *false*.
43. Set _locale_.[[NumberingSystem]] to _localeRecord_.[[nu]].
44. Return _locale_.

--- referenced: OrdinaryCreateFromConstructor ( _ctor_: a function object, _intrinsicDefaultProto_: a String, optional _internalSlotsList_: a List of names of internal slots, ): either a normal completion containing an Object or a throw completion
  id: sec-ordinarycreatefromconstructor

description: It creates an ordinary object whose [[Prototype]] value is retrieved from a constructor's *"prototype"* property, if it exists. Otherwise the intrinsic named by _intrinsicDefaultProto_ is used for [[Prototype]]. _internalSlotsList_ contains the names of additional internal slots that must be defined as part of the object. If _internalSlotsList_ is not provided, a new empty List is used.

1. Assert: _intrinsicDefaultProto_ is this specification's name of an intrinsic object. The corresponding object must be an intrinsic that is intended to be used as the [[Prototype]] value of an object.
2. Let _proto_ be ? GetPrototypeFromConstructor(_ctor_, _intrinsicDefaultProto_).
3. If _internalSlotsList_ is present, let _slots_ be _internalSlotsList_.
4. Else, let _slots_ be a new empty List.
5. Return OrdinaryObjectCreate(_proto_, _slots_).

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

--- referenced: WeekdayToUValue ( _fw_: a String, ): a String
  id: sec-weekdaytouvalue

description: It converts a string that numerically represents a day of the week to a valid Unicode First Day Identifier defined in Unicode Technical Standard #35 Part 1 Core Section 3.6.1 Key and Type Definitions, and returns any other string unmodified.

1. For each row of [table-locale-weekday-string-value], except the header row, in table order, do
   a. Let _weekday_ be the Weekday value of the current row.
   b. Let _string_ be the String value of the current row.
   c. If _fw_ is equal to _weekday_, return _string_.
2. Return _fw_.

Weekday String and Value
Weekday |
String |
Value |
*"0"* |
*"sun"* |
*7*_𝔽 |
*"1"* |
*"mon"* |
*1*_𝔽 |
*"2"* |
*"tue"* |
*2*_𝔽 |
*"3"* |
*"wed"* |
*3*_𝔽 |
*"4"* |
*"thu"* |
*4*_𝔽 |
*"5"* |
*"fri"* |
*5*_𝔽 |
*"6"* |
*"sat"* |
*6*_𝔽 |
*"7"* |
*"sun"* |
*7*_𝔽 |


UpdateLanguageId ( _tag_: a well-formed language tag, _options_: an Object, ): either a normal completion containing a well-formed language tag or a throw completion
  id: sec-updatelanguageid   type: abstract operation

description: It updates the unicode_language_id subtags in _tag_ from the corresponding properties of _options_, validates them, and returns the result.

1. Let _baseName_ be GetLocaleBaseName(_tag_).
2. Let _language_ be ? GetOption(_options_, *"language"*, ~string~, ~empty~, GetLocaleLanguage(_baseName_)).
3. If _language_ cannot be matched by the unicode_language_subtag Unicode locale nonterminal, throw a *RangeError* exception.
4. Let _script_ be ? GetOption(_options_, *"script"*, ~string~, ~empty~, GetLocaleScript(_baseName_)).
5. If _script_ is not *undefined*, then
   a. If _script_ cannot be matched by the unicode_script_subtag Unicode locale nonterminal, throw a *RangeError* exception.
6. Let _region_ be ? GetOption(_options_, *"region"*, ~string~, ~empty~, GetLocaleRegion(_baseName_)).
7. If _region_ is not *undefined*, then
   a. If _region_ cannot be matched by the unicode_region_subtag Unicode locale nonterminal, throw a *RangeError* exception.
8. Let _variants_ be ? GetOption(_options_, *"variants"*, ~string~, ~empty~, GetLocaleVariants(_baseName_)).
9. If _variants_ is not *undefined*, then
   a. If _variants_ is the empty String, throw a *RangeError* exception.
   b. Let _lowerVariants_ be the ASCII-lowercase of _variants_.
   c. Let _variantSubtags_ be StringSplitToList(_lowerVariants_, *"-"*).
   d. For each element _variant_ of _variantSubtags_, do
      i. If _variant_ cannot be matched by the unicode_variant_subtag Unicode locale nonterminal, throw a *RangeError* exception.
   e. If _variantSubtags_ contains any duplicate elements, throw a *RangeError* exception.
10. Let _allExtensions_ be the suffix of _tag_ following _baseName_.
11. Let _newTag_ be _language_.
12. If _script_ is not *undefined*, set _newTag_ to the string-concatenation of _newTag_, *"-"*, and _script_.
13. If _region_ is not *undefined*, set _newTag_ to the string-concatenation of _newTag_, *"-"*, and _region_.
14. If _variants_ is not *undefined*, set _newTag_ to the string-concatenation of _newTag_, *"-"*, and _variants_.
15. Set _newTag_ to the string-concatenation of _newTag_ and _allExtensions_.
16. Return _newTag_.

--- referenced: GetLocaleBaseName ( _locale_: a String, ): a String
  id: sec-getlocalebasename

1. Assert: _locale_ can be matched by the unicode_locale_id Unicode locale nonterminal.
2. Return the longest prefix of _locale_ matched by the unicode_language_id Unicode locale nonterminal.

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

--- referenced: GetLocaleLanguage ( _locale_: a String, ): a String
  id: sec-getlocalelanguage

1. Let _baseName_ be GetLocaleBaseName(_locale_).
2. Let _subtags_ be StringSplitToList(_baseName_, *"-"*).
3. Assert: _subtags_[0] is matched by the unicode_language_subtag Unicode locale nonterminal.
4. Return _subtags_[0].

--- referenced: GetLocaleScript ( _locale_: a String, ): a String or *undefined*
  id: sec-getlocalescript

1. Let _baseName_ be GetLocaleBaseName(_locale_).
2. NOTE: A unicode_script_subtag subtag is only valid immediately after an initial unicode_language_subtag subtag. In that position, it cannot be confused with any other valid subtag because all their productions are disjoint.
3. Let _subtags_ be StringSplitToList(_baseName_, *"-"*).
4. Assert: _subtags_[0] is matched by the unicode_language_subtag Unicode locale nonterminal.
5. If _subtags_ has at least 2 elements and _subtags_[1] is matched by the unicode_script_subtag Unicode locale nonterminal, return _subtags_[1].
6. Return *undefined*.

--- referenced: GetLocaleRegion ( _locale_: a String, ): a String or *undefined*
  id: sec-getlocaleregion

1. Let _baseName_ be GetLocaleBaseName(_locale_).
2. NOTE: A unicode_region_subtag subtag is only valid immediately after an initial unicode_language_subtag subtag, optionally with a single unicode_script_subtag subtag between them. In that position, it cannot be confused with any other valid subtag because all their productions are disjoint.
3. Let _subtags_ be StringSplitToList(_baseName_, *"-"*).
4. Assert: _subtags_[0] is matched by the unicode_language_subtag Unicode locale nonterminal.
5. If _subtags_ has at least 2 elements and _subtags_[1] is matched by the unicode_region_subtag Unicode locale nonterminal, return _subtags_[1].
6. If _subtags_ has at least 3 elements and _subtags_[2] is matched by the unicode_region_subtag Unicode locale nonterminal, return _subtags_[2].
7. Return *undefined*.

--- referenced: GetLocaleVariants ( _locale_: a String, ): a String or *undefined*
  id: sec-getlocalevariants

1. Let _baseName_ be GetLocaleBaseName(_locale_).
2. NOTE: Each non-initial subtag in _baseName_ is either a unicode_script_subtag, unicode_region_subtag, or unicode_variant_subtag, whose productions are all mutually disjoint.
3. Let _subtags_ be StringSplitToList(_baseName_, *"-"*).
4. Assert: _subtags_[0] is matched by the unicode_language_subtag Unicode locale nonterminal.
5. Let _dashIndex_ be the length of _subtags_[0].
6. Remove the first element from _subtags_.
7. For each String _subtag_ of _subtags_, do
   a. If _subtag_ is matched by the unicode_variant_subtag Unicode locale nonterminal, then
      i. Return the substring of _baseName_ from _dashIndex_ + 1.
   b. Let _subtagLength_ be the length of _subtag_.
   c. Set _dashIndex_ to _dashIndex_ + _subtagLength_ + 1.
8. Return *undefined*.

--- referenced: StringSplitToList ( _string_: a String, _separator_: a String, ): a List of Strings
  id: sec-string-split-to-list

description:
The returned List contains all disjoint substrings of _string_ that do not contain _separator_ but are immediately preceded and/or immediately followed by an occurrence of _separator_.
Each such substring will be the empty String between adjacent occurrences of _separator_, before a _separator_ at the very start of _string_, or after a _separator_ at the very end of _string_, but otherwise will not be empty.

1. Assert: _string_ is not the empty String.
2. Assert: _separator_ is not the empty String.
3. Let _separatorLength_ be the length of _separator_.
4. Let _substrings_ be a new empty List.
5. Let _i_ be 0.
6. Let _j_ be StringIndexOf(_string_, _separator_, 0).
7. Repeat, while _j_ is not ~not-found~,
   a. Let _substring_ be the substring of _string_ from _i_ to _j_.
   b. Append _substring_ to _substrings_.
   c. Set _i_ to _j_ + _separatorLength_.
   d. Set _j_ to StringIndexOf(_string_, _separator_, _i_).
8. Let _substring_ be the substring of _string_ from _i_.
9. Append _substring_ to _substrings_.
10. Return _substrings_.


MakeLocaleRecord ( _tag_: a language tag, _options_: a Record, _localeExtensionKeys_: a List of Strings, ): a Record
  id: sec-makelocalerecord   type: abstract operation

description: It constructs and returns a Record in which each element of _localeExtensionKeys_ defines a corresponding field with data from any Unicode locale extension sequence of _tag_ as overridden by a corresponding field of _options_, and which additionally includes a [[locale]] field containing a Unicode canonicalized locale identifier resulting from incorporating those fields into _tag_.

1. If _tag_ contains a substring that is a Unicode locale extension sequence, then
   a. Let _extension_ be the String value consisting of the substring of the Unicode locale extension sequence within _tag_.
   b. Let _components_ be UnicodeExtensionComponents(_extension_).
   c. Let _attributes_ be _components_.[[Attributes]].
   d. Let _keywords_ be _components_.[[Keywords]].
2. Else,
   a. Let _attributes_ be a new empty List.
   b. Let _keywords_ be a new empty List.
3. Let _result_ be a new Record.
4. For each element _key_ of _localeExtensionKeys_, do
   a. If _keywords_ contains an element whose [[Key]] is _key_, then
      i. Let _entry_ be the element of _keywords_ whose [[Key]] is _key_.
      ii. Let _value_ be _entry_.[[Value]].
   b. Else,
      i. Let _entry_ be ~empty~.
      ii. Let _value_ be *undefined*.
   c. Assert: _options_ has a field [[<_key_>]].
   d. Let _overrideValue_ be _options_.[[<_key_>]].
   e. If _overrideValue_ is not *undefined*, then
      i. Set _value_ to CanonicalizeUValue(_key_, _overrideValue_).
      ii. If _entry_ is not ~empty~, then
         1. Set _entry_.[[Value]] to _value_.
      iii. Else,
         1. Append the Record { [[Key]]: _key_, [[Value]]: _value_ } to _keywords_.
   f. Set _result_.[[<_key_>]] to _value_.
5. Let _locale_ be the String value that is _tag_ with any Unicode locale extension sequences removed.
6. If _attributes_ is not empty or _keywords_ is not empty, then
   a. Set _result_.[[locale]] to InsertUnicodeExtensionAndCanonicalize(_locale_, _attributes_, _keywords_).
7. Else,
   a. Set _result_.[[locale]] to CanonicalizeUnicodeLocaleId(_locale_).
8. Return _result_.

--- referenced: CanonicalizeUValue ( _ukey_: a Unicode locale extension sequence key defined in Unicode Technical Standard #35 Part 1 Core Section 3.6.1 Key and Type Definitions, _uvalue_: a String, ): a String
  id: sec-canonicalizeuvalue

description: The returned String is the canonical and case-regularized form of _uvalue_ as a value of _ukey_.

1. Let _lowerValue_ be the ASCII-lowercase of _uvalue_.
2. Let _canonicalized_ be the String value resulting from canonicalizing _lowerValue_ as a value of key _ukey_ per Unicode Technical Standard #35 Part 1 Core, Annex C LocaleId Canonicalization Section 5 Canonicalizing Syntax, Processing LocaleIds.
3. NOTE: It is recommended that implementations use the 'u' extension data in common/bcp47 provided by the Common Locale Data Repository (available at https://cldr.unicode.org/).
4. Return _canonicalized_.

--- referenced: InsertUnicodeExtensionAndCanonicalize ( _locale_: a language tag, _attributes_: a List of Strings, _keywords_: a List of Records, ): a Unicode canonicalized locale identifier
  id: sec-insert-unicode-extension-and-canonicalize

description: It incorporates _attributes_ and _keywords_ into _locale_ as a Unicode locale extension sequence and returns the canonicalized result.

1. Assert: _locale_ does not contain a Unicode locale extension sequence.
2. Let _extension_ be *"-u"*.
3. For each element _attr_ of _attributes_, do
   a. Set _extension_ to the string-concatenation of _extension_, *"-"*, and _attr_.
4. For each Record { [[Key]], [[Value]] } _keyword_ of _keywords_, do
   a. Set _extension_ to the string-concatenation of _extension_, *"-"*, and _keyword_.[[Key]].
   b. If _keyword_.[[Value]] is not the empty String, set _extension_ to the string-concatenation of _extension_, *"-"*, and _keyword_.[[Value]].
5. If _extension_ is *"-u"*, return CanonicalizeUnicodeLocaleId(_locale_).
6. Let _privateIndex_ be StringIndexOf(_locale_, *"-x-"*, 0).
7. If _privateIndex_ is ~not-found~, then
   a. Let _newLocale_ be the string-concatenation of _locale_ and _extension_.
8. Else,
   a. Let _preExtension_ be the substring of _locale_ from 0 to _privateIndex_.
   b. Let _postExtension_ be the substring of _locale_ from _privateIndex_.
   c. Let _newLocale_ be the string-concatenation of _preExtension_, _extension_, and _postExtension_.
9. Assert: IsWellFormedLanguageTag(_newLocale_) is *true*.
10. Return CanonicalizeUnicodeLocaleId(_newLocale_).


## Done when

- Every test262 run under intl402/Locale/ passes (check with `./t262 intl402/Locale/`). 0 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 545: worker reported BLOCKED but changed files: the locale-data methods need CLDR region preference, calendar, collation, numbering system, time zone and week data that I did not wire up from ICU4X (`./spec` was also broken in this worktree).
- attempt 545: rejected by gates: files outside scope: crates/kahvi-cli/src/main.rs
- attempt 549: worker reported BLOCKED but changed files: the six locale-data methods (getCalendars, getCollations, getHourCycles, getNumberingSystems, getTimeZones, getWeekInfo) need CLDR region-preference data wired up from ICU4X; bare `und` maximize needs ICU4X's `und` → `en-Latn-US`; and `posix` needs `is_well_formed_language_tag` in `support/intl.rs` (out of scope) to accept 5–8 letter language subtags.
- attempt 549: not merged (swallows_errors, contradicts_spec); continue from this diff. 1) In support/intl_locale.rs make_locale_record (about line 326), apply the "true" to "" canonicalization only for boolean keys (kn). Keep other values verbatim after CanonicalizeUValue. Check that calendar, collation and numberingSystem set to "true" round-trip as "-u-ca-true" and so on.
2) In prototype_get_text_info.rs (about line 38), do not map an ICU parse failure to undefined. Either compute the direction from the parsed id or return agent.not_implemented naming the gap. Do not silently return undefined.
3) Investigate intl402/Locale/likely-subtags.js: "und".maximize() returns "und". The construct_with_likely_subtags path either hits the `if let Ok` fallback or LocaleExpander::new_extended() does not expand. The spec allows the unchanged fallback only when an error is signaled. Verify the ICU data is present and that the parse succeeds for "und".
4) Step 42 of the constructor: Numeric should be stored as false when kn is not in the extension keys, not left as None. The getter masks this, but storing false matches the spec.
5) getCalendars, getCollations, getHourCycles, getNumberingSystems, getTimeZones and getWeekInfo should keep throwing not_implemented until implemented. That is correct as is.

