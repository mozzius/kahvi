# Ticket I-supported-values-of: Intl.supportedValuesOf

Implement Intl.supportedValuesOf following the spec steps in the stub (keep
each step as a comment directly above the code for it). Read docs/INTL.md
first.

- Each list comes from the data ICU4X ships (calendars ICU4X's
  `icu::calendar` supports, collations in its collation data, the
  numbering systems in its decimal data, the IANA time zones of
  `icu::time`, ...). Never write lists to match test expectations; a list
  ICU4X cannot provide stays `not_implemented` naming it.
- No new Rust tests.

## Files you may change

- `crates/kahvi-builtins/src/intl/supported_values_of.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`

## Spec

Intl.supportedValuesOf ( _key_ )
  id: sec-intl.supportedvaluesof   type: built-in function

When the `supportedValuesOf` method is called with argument _key_ , the following steps are taken:

1. Set _key_ to ? ToString(_key_).
2. If _key_ is *"calendar"*, then
   a. Let _list_ be a new empty List.
   b. For each element _identifier_ of AvailableCalendars(), do
      i. Let _canonical_ be CanonicalizeUValue(*"ca"*, _identifier_).
      ii. If _identifier_ is _canonical_, then
         1. Append _identifier_ to _list_.
3. Else if _key_ is *"collation"*, then
   a. Let _list_ be AvailableCanonicalCollations( ).
4. Else if _key_ is *"currency"*, then
   a. Let _list_ be AvailableCanonicalCurrencies( ).
5. Else if _key_ is *"numberingSystem"*, then
   a. Let _list_ be AvailableCanonicalNumberingSystems( ).
6. Else if _key_ is *"timeZone"*, then
   a. Let _list_ be AvailablePrimaryTimeZoneIdentifiers( ).
7. Else if _key_ is *"unit"*, then
   a. Let _list_ be AvailableCanonicalUnits( ).
8. Else,
   a. Throw a *RangeError* exception.
9. Return CreateArrayFromList( _list_ ).

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

--- referenced: AvailableCalendars ( ): a List of Strings
  id: sec-availablecalendars

description: The returned List is sorted according to lexicographic code unit order, and contains unique calendar types in canonical form ([sec-calendar-types]) identifying the calendars for which the implementation provides the functionality of Intl.DateTimeFormat objects, including their aliases (e.g., either both or neither of *"islamicc"* and *"islamic-civil"*). The List must include *"iso8601"*.

--- referenced: CanonicalizeUValue ( _ukey_: a Unicode locale extension sequence key defined in Unicode Technical Standard #35 Part 1 Core Section 3.6.1 Key and Type Definitions, _uvalue_: a String, ): a String
  id: sec-canonicalizeuvalue

description: The returned String is the canonical and case-regularized form of _uvalue_ as a value of _ukey_.

1. Let _lowerValue_ be the ASCII-lowercase of _uvalue_.
2. Let _canonicalized_ be the String value resulting from canonicalizing _lowerValue_ as a value of key _ukey_ per Unicode Technical Standard #35 Part 1 Core, Annex C LocaleId Canonicalization Section 5 Canonicalizing Syntax, Processing LocaleIds.
3. NOTE: It is recommended that implementations use the 'u' extension data in common/bcp47 provided by the Common Locale Data Repository (available at https://cldr.unicode.org/).
4. Return _canonicalized_.

--- referenced: AvailableCanonicalCollations ( ): a List of Strings
  id: sec-availablecanonicalcollations

description: The returned List is sorted according to lexicographic code unit order, and contains unique canonical collation types identifying the collations for which the implementation provides the functionality of Intl.Collator objects.

--- referenced: AvailableCanonicalCurrencies ( ): a List of Strings
  id: sec-availablecanonicalcurrencies

description: The returned List is sorted according to lexicographic code unit order, and contains unique, well-formed, and upper case canonicalized 3-letter ISO 4217 currency codes, identifying the currencies for which the implementation provides the functionality of Intl.DisplayNames and Intl.NumberFormat objects.

--- referenced: AvailableCanonicalNumberingSystems ( ): a List of Strings
  id: sec-availablecanonicalnumberingsystems

description: The returned List is sorted according to lexicographic code unit order, and contains unique canonical numbering systems identifiers identifying the numbering systems for which the implementation provides the functionality of Intl.DateTimeFormat, Intl.NumberFormat, and Intl.RelativeTimeFormat objects. The List must include the Numbering System value of every row of [table-numbering-system-digits], except the header row.

--- referenced: AvailablePrimaryTimeZoneIdentifiers ( ): a List of Strings
  id: sec-availableprimarytimezoneidentifiers

description: The returned List is a sorted List of supported Zone and Link names in the IANA Time Zone Database.

1. Let _records_ be AvailableNamedTimeZoneIdentifiers().
2. Let _result_ be a new empty List.
3. For each element _timeZoneIdentifierRecord_ of _records_, do
   a. If _timeZoneIdentifierRecord_.[[Identifier]] is _timeZoneIdentifierRecord_.[[PrimaryIdentifier]], then
      i. Append _timeZoneIdentifierRecord_.[[Identifier]] to _result_.
4. Return _result_.

--- referenced: AvailableCanonicalUnits ( ): a List of Strings
  id: sec-availablecanonicalunits

description: The returned List is sorted according to lexicographic code unit order, and consists of the unique values of simple unit identifiers listed in every row of [table-sanctioned-single-unit-identifiers], except the header row.

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.


AvailableCalendars ( ): a List of Strings
  id: sec-availablecalendars   type: implementation-defined abstract operation

description: The returned List is sorted according to lexicographic code unit order, and contains unique calendar types in canonical form ([sec-calendar-types]) identifying the calendars for which the implementation provides the functionality of Intl.DateTimeFormat objects, including their aliases (e.g., either both or neither of *"islamicc"* and *"islamic-civil"*). The List must include *"iso8601"*.


AvailableCanonicalCollations ( ): a List of Strings
  id: sec-availablecanonicalcollations   type: implementation-defined abstract operation

description: The returned List is sorted according to lexicographic code unit order, and contains unique canonical collation types identifying the collations for which the implementation provides the functionality of Intl.Collator objects.


AvailableCanonicalCurrencies ( ): a List of Strings
  id: sec-availablecanonicalcurrencies   type: implementation-defined abstract operation

description: The returned List is sorted according to lexicographic code unit order, and contains unique, well-formed, and upper case canonicalized 3-letter ISO 4217 currency codes, identifying the currencies for which the implementation provides the functionality of Intl.DisplayNames and Intl.NumberFormat objects.


AvailableCanonicalNumberingSystems ( ): a List of Strings
  id: sec-availablecanonicalnumberingsystems   type: implementation-defined abstract operation

description: The returned List is sorted according to lexicographic code unit order, and contains unique canonical numbering systems identifiers identifying the numbering systems for which the implementation provides the functionality of Intl.DateTimeFormat, Intl.NumberFormat, and Intl.RelativeTimeFormat objects. The List must include the Numbering System value of every row of [table-numbering-system-digits], except the header row.


AvailableNamedTimeZoneIdentifiers ( ): a List of Time Zone Identifier Records
  id: sup-availablenamedtimezoneidentifiers   type: implementation-defined abstract operation

description:
Its result describes all available named time zone identifiers in this implementation, as well as the primary time zone identifier corresponding to each available named time zone identifier.
The List is ordered according to the [[Identifier]] field of each Time Zone Identifier Record.
This definition supersedes the definition provided in [sec-availablenamedtimezoneidentifiers].

1. Let _identifiers_ be a List containing the String value of each Zone or Link name in the IANA Time Zone Database.
2. Assert: No element of _identifiers_ is an ASCII-case-insensitive match for any other element.
3. Sort _identifiers_ according to lexicographic code unit order.
4. Let _result_ be a new empty List.
5. For each element _identifier_ of _identifiers_, do
   a. Let _primary_ be _identifier_.
   b. If _identifier_ is a Link name in the IANA Time Zone Database and _identifier_ is not present in the “TZ” column of zone.tab of the IANA Time Zone Database, then
      i. Let _zone_ be the Zone name that _identifier_ resolves to, according to the rules for resolving Link names in the IANA Time Zone Database.
      ii. If _zone_ starts with *"Etc/"*, then
         1. Set _primary_ to _zone_.
      iii. Else,
         1. Let _identifierCountryCode_ be the ISO 3166-1 Alpha-2 country code whose territory contains the geographical area corresponding to _identifier_.
         2. Let _zoneCountryCode_ be the ISO 3166-1 Alpha-2 country code whose territory contains the geographical area corresponding to _zone_.
         3. If _identifierCountryCode_ is _zoneCountryCode_, then
            a. Set _primary_ to _zone_.
         4. Else,
            a. Let _countryCodeLineCount_ be the number of lines in file zone.tab of the IANA Time Zone Database where the “country-code” column is _identifierCountryCode_.
            b. If _countryCodeLineCount_ is 1, then
               i. Let _countryCodeLine_ be the line in file zone.tab of the IANA Time Zone Database where the “country-code” column is _identifierCountryCode_.
               ii. Set _primary_ to the contents of the “TZ” column of _countryCodeLine_.
            c. Else,
               i. Let _backzone_ be *undefined*.
               ii. Let _backzoneLinkLines_ be the List of lines in the file backzone of the IANA Time Zone Database that start with either *"Link "* or *"#PACKRATLIST zone.tab Link "*.
               iii. For each element _line_ of _backzoneLinkLines_, do
                  1. Let _i_ be StringIndexOf(_line_, *"Link "*, 0).
                  2. Set _line_ to the substring of _line_ from _i_ + 5.
                  3. Let _backzoneAndLink_ be StringSplitToList(_line_, *" "*).
                  4. Assert: _backzoneAndLink_ has at least two elements, and both _backzoneAndLink_[0] and _backzoneAndLink_[1] are available named time zone identifiers.
                  5. If _backzoneAndLink_[1] is _identifier_, then
                     a. Assert: _backzone_ is *undefined*.
                     b. Set _backzone_ to _backzoneAndLink_[0].
               iv. Assert: _backzone_ is not *undefined*.
               v. Set _primary_ to _backzone_.
   c. If _primary_ is one of *"Etc/UTC"*, *"Etc/GMT"*, or *"GMT"*, set _primary_ to *"UTC"*.
   d. If _primary_ is a replacement time zone identifier and its rename waiting period has not concluded, then
      i. Let _renamedIdentifier_ be the renamed time zone identifier that _primary_ replaced.
      ii. Set _primary_ to _renamedIdentifier_.
   e. Let _record_ be the Time Zone Identifier Record { [[Identifier]]: _identifier_, [[PrimaryIdentifier]]: _primary_ }.
   f. Append _record_ to _result_.
6. Assert: _result_ contains a Time Zone Identifier Record _utcRecord_ such that _utcRecord_.[[Identifier]] is *"UTC"* and _utcRecord_.[[PrimaryIdentifier]] is *"UTC"*.
7. Return _result_.

NOTE:
The algorithm above for resolving Links to primary time zone identifiers is intended to correspond to the behaviour of icu::TimeZone::getIanaID() in the International Components for Unicode (ICU) and the processes for maintaining time zone identifier data in the Unicode Common Locale Data Repository (CLDR).
This algorithm resolves Links to primary time zone identifiers without crossing the boundaries of ISO 3166-1 Alpha-2 country codes, using data from files zone.tab and backzone of the IANA Time Zone Database.
If the country code of a Link has only one line in zone.tab, then that line will determine the primary time zone identifier.
However, if that country code has multiple lines in zone.tab, then historical mappings in backzone must be used to identify the correct primary time zone identifier.
For example, to resolve "Pacific/Truk" (in country code "FM") if the default build options of the IANA Time Zone Database identify it as a Link to "Pacific/Port_Moresby" (in country code "PG"), then the “country-code” column of zone.tab must be checked for lines corresponding with "FM".
If there were only one such line, then the “TZ” column of that line would determine the primary time zone identifier associated with "Pacific/Truk".
But if there are multiple "FM" lines in zone.tab, then backzone must be inspected and a line such as "Link Pacific/Chuuk Pacific/Truk" would result in using "Pacific/Chuuk" as the primary time zone identifier.
Note that zone.tab is the preferred source of mapping data because backzone mappings may, in rare cases, cross the boundaries of ISO 3166-1 Alpha-2 country codes.
For example, "Atlantic/Jan_Mayen" (in country code "SJ") is mapped in backzone to "Europe/Oslo" (in country code "NO").
As of the 2024a release of the IANA Time Zone Database, "Atlantic/Jan_Mayen" is the only case where this happens.
NOTE:
Time zone identifiers in the IANA Time Zone Database can change over time.
At a minimum, it is recommended that implementations limit changes to the result of AvailableNamedTimeZoneIdentifiers to the changes allowed by GetAvailableNamedTimeZoneIdentifier, for the lifetime of the surrounding agent.
Due to the complexity of supporting these recommendations, it is recommended that the result of AvailableNamedTimeZoneIdentifiers remains the same for the lifetime of the surrounding agent.

--- referenced: StringIndexOf ( _string_: a String, _searchValue_: a String, _fromIndex_: a non-negative integer, ): a non-negative integer or ~not-found~
  id: sec-stringindexof

1. Let _length_ be the length of _string_.
2. If _searchValue_ is the empty String and _fromIndex_ ≤ _length_, return _fromIndex_.
3. Let _searchLength_ be the length of _searchValue_.
4. For each integer _i_ such that _fromIndex_ ≤ _i_ ≤ _length_ - _searchLength_, in ascending order, do
   a. Let _candidate_ be the substring of _string_ from _i_ to _i_ + _searchLength_.
   b. If _candidate_ is _searchValue_, return _i_.
5. Return ~not-found~.

NOTE:
If _searchValue_ is the empty String and _fromIndex_ ≤ the length of _string_, this algorithm returns _fromIndex_. The empty String is effectively found at every position within a string, including after the last code unit.
NOTE:
This algorithm always returns ~not-found~ if _fromIndex_ + the length of _searchValue_ > the length of _string_.

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


AvailablePrimaryTimeZoneIdentifiers ( ): a List of Strings
  id: sec-availableprimarytimezoneidentifiers   type: abstract operation

description: The returned List is a sorted List of supported Zone and Link names in the IANA Time Zone Database.

1. Let _records_ be AvailableNamedTimeZoneIdentifiers().
2. Let _result_ be a new empty List.
3. For each element _timeZoneIdentifierRecord_ of _records_, do
   a. If _timeZoneIdentifierRecord_.[[Identifier]] is _timeZoneIdentifierRecord_.[[PrimaryIdentifier]], then
      i. Append _timeZoneIdentifierRecord_.[[Identifier]] to _result_.
4. Return _result_.


AvailableCanonicalUnits ( ): a List of Strings
  id: sec-availablecanonicalunits   type: abstract operation

description: The returned List is sorted according to lexicographic code unit order, and consists of the unique values of simple unit identifiers listed in every row of [table-sanctioned-single-unit-identifiers], except the header row.


## Done when

- Every test262 run under intl402/Intl/supportedValuesOf/ passes (check with `./t262 intl402/Intl/supportedValuesOf/`). 0 pass on the main branch right now.

