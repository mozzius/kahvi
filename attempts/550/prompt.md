# Ticket I-list-format: Intl.ListFormat

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

- `crates/kahvi-builtins/src/intl_list_format/*.rs`
- `crates/kahvi-builtins/src/support/intl_list_format.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs`
- `crates/kahvi-builtins/src/disposable_stack/constructor.rs`

## Spec

Intl.ListFormat ( [ _locales_ [ , _options_ ] ] )
  id: sec-Intl.ListFormat   type: built-in function

When the `Intl.ListFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:

1. If NewTarget is *undefined*, throw a *TypeError* exception.
2. Let _listFormat_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.ListFormat.prototype%"*, « [[InitializedListFormat]], [[Locale]], [[Type]], [[Style]], [[Templates]] »).
3. Let _optionsResolution_ be ? ResolveOptions(%Intl.ListFormat%, %Intl.ListFormat%.[[LocaleData]], _locales_, _options_).
4. Set _options_ to _optionsResolution_.[[Options]].
5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
6. Set _listFormat_.[[Locale]] to _resolvedLocale_.[[Locale]].
7. Let _type_ be ? GetOption(_options_, *"type"*, ~string~, « *"conjunction"*, *"disjunction"*, *"unit"* », *"conjunction"*).
8. Set _listFormat_.[[Type]] to _type_.
9. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"long"*, *"short"*, *"narrow"* », *"long"*).
10. Set _listFormat_.[[Style]] to _style_.
11. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
12. Let _dataLocaleTypes_ be _resolvedLocaleData_.[[<_type_>]].
13. Set _listFormat_.[[Templates]] to _dataLocaleTypes_.[[<_style_>]].
14. Return _listFormat_.

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


CreatePartsFromList ( _listFormat_: an Intl.ListFormat, _list_: a List of Strings, ): a List of Records with fields [[Type]] (*"element"* or *"literal"*) and [[Value]] (a String)
  id: sec-createpartsfromlist   type: abstract operation

description: It creates the corresponding List of parts according to the effective locale and the formatting options of _listFormat_.

1. Let _size_ be the number of elements of _list_.
2. If _size_ is 0, then
   a. Return a new empty List.
3. If _size_ is 2, then
   a. Let _n_ be an index into _listFormat_.[[Templates]] based on _listFormat_.[[Locale]], _list_[0], and _list_[1].
   b. Let _pattern_ be _listFormat_.[[Templates]][_n_].[[Pair]].
   c. Let _first_ be the Record { [[Type]]: *"element"*, [[Value]]: _list_[0] }.
   d. Let _second_ be the Record { [[Type]]: *"element"*, [[Value]]: _list_[1] }.
   e. Let _placeables_ be the Record { [[0]]: _first_, [[1]]: _second_ }.
   f. Return DeconstructPattern(_pattern_, _placeables_).
4. Let _last_ be the Record { [[Type]]: *"element"*, [[Value]]: _list_[_size_ - 1] }.
5. Let _parts_ be « _last_ ».
6. Let _i_ be _size_ - 2.
7. Repeat, while _i_ ≥ 0,
   a. Let _head_ be the Record { [[Type]]: *"element"*, [[Value]]: _list_[_i_] }.
   b. Let _n_ be an implementation-defined index into _listFormat_.[[Templates]] based on _listFormat_.[[Locale]], _head_, and _parts_.
   c. If _i_ is 0, then
      i. Let _pattern_ be _listFormat_.[[Templates]][_n_].[[Start]].
   d. Else if _i_ is less than _size_ - 2, then
      i. Let _pattern_ be _listFormat_.[[Templates]][_n_].[[Middle]].
   e. Else,
      i. Let _pattern_ be _listFormat_.[[Templates]][_n_].[[End]].
   f. Let _placeables_ be the Record { [[0]]: _head_, [[1]]: _parts_ }.
   g. Set _parts_ to DeconstructPattern(_pattern_, _placeables_).
   h. Set _i_ to _i_ - 1.
8. Return _parts_.

NOTE:
The index _n_ to select across multiple templates permits the conjunction to be dependent on the context, as in Spanish, where either "y" or "e" may be selected, depending on the following word.

--- referenced: DeconstructPattern ( _pattern_: a Pattern String, _placeables_: a Record, ): a List
  id: sec-deconstructpattern

description:
It deconstructs the pattern string into a List of parts.
_placeables_ is a Record whose keys are placeables tokens used in the pattern string, and values are parts Records (as from PartitionPattern) which will be used in the result List to represent the token part.
Example:
Input:
DeconstructPattern("AA{xx}BB{yy}CC", {
[[xx]]: {[[Type]]: "hour", [[Value]]: "15"},
[[yy]]: {[[Type]]: "minute", [[Value]]: "06"}
})
Output (List of parts Records):
«
{[[Type]]: "literal", [[Value]]: "AA"},
{[[Type]]: "hour", [[Value]]: "15"},
{[[Type]]: "literal", [[Value]]: "BB"},
{[[Type]]: "minute", [[Value]]: "06"},
{[[Type]]: "literal", [[Value]]: "CC"}
»

1. Let _patternParts_ be PartitionPattern(_pattern_).
2. Let _result_ be a new empty List.
3. For each Record { [[Type]], [[Value]] } _patternPart_ of _patternParts_, do
   a. Let _partType_ be _patternPart_.[[Type]].
   b. If _partType_ is *"literal"*, then
      i. Append the Record { [[Type]]: *"literal"*, [[Value]]: _patternPart_.[[Value]] } to _result_.
   c. Else,
      i. Assert: _placeables_ has a field [[<_partType_>]].
      ii. Let _substitution_ be _placeables_.[[<_partType_>]].
      iii. If _substitution_ is a List, then
         1. For each element _part_ of _substitution_, do
            a. Append _part_ to _result_.
      iv. Else,
         1. Append _substitution_ to _result_.
4. Return _result_.


## Done when

- Every test262 run under intl402/ListFormat/ passes (check with `./t262 intl402/ListFormat/`). 0 pass on the main branch right now.

