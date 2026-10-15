# Ticket I-segmenter: Intl.Segmenter

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

- `crates/kahvi-builtins/src/intl_segmenter/*.rs`
- `crates/kahvi-builtins/src/intl_segments_prototype/*.rs`
- `crates/kahvi-builtins/src/intl_segment_iterator_prototype/*.rs`
- `crates/kahvi-builtins/src/support/intl_segmenter.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs`
- `crates/kahvi-builtins/src/disposable_stack/constructor.rs`

## Spec

Intl.Segmenter ( [ _locales_ [ , _options_ ] ] )
  id: sec-intl.segmenter   type: built-in function

When the `Intl.Segmenter` function is called with optional arguments _locales_ and _options_, the following steps are taken:

1. If NewTarget is *undefined*, throw a *TypeError* exception.
2. Let _internalSlotsList_ be « [[InitializedSegmenter]], [[Locale]], [[SegmenterGranularity]] ».
3. Let _segmenter_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.Segmenter.prototype%"*, _internalSlotsList_).
4. Let _optionsResolution_ be ? ResolveOptions(%Intl.Segmenter%, %Intl.Segmenter%.[[LocaleData]], _locales_, _options_).
5. Set _options_ to _optionsResolution_.[[Options]].
6. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
7. Set _segmenter_.[[Locale]] to _resolvedLocale_.[[Locale]].
8. Let _granularity_ be ? GetOption(_options_, *"granularity"*, ~string~, « *"grapheme"*, *"word"*, *"sentence"* », *"grapheme"*).
9. Set _segmenter_.[[SegmenterGranularity]] to _granularity_.
10. Return _segmenter_.

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


CreateSegmentsObject ( _segmenter_: an Intl.Segmenter, _string_: a String, ): a Segments instance
  id: sec-createsegmentsobject   type: abstract operation

description: The Segments instance references _segmenter_ and _string_.

1. Let _internalSlotsList_ be « [[SegmentsSegmenter]], [[SegmentsString]] ».
2. Let _segments_ be OrdinaryObjectCreate(%IntlSegmentsPrototype%, _internalSlotsList_).
3. Set _segments_.[[SegmentsSegmenter]] to _segmenter_.
4. Set _segments_.[[SegmentsString]] to _string_.
5. Return _segments_.

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


CreateSegmentIterator ( _segmenter_: an Intl.Segmenter, _string_: a String, ): a Segment Iterator
  id: sec-createsegmentiterator   type: abstract operation

description: The Segment Iterator iterates over _string_ using the locale and options of _segmenter_.

1. Let _internalSlotsList_ be « [[IteratingSegmenter]], [[IteratedString]], [[IteratedStringNextSegmentCodeUnitIndex]] ».
2. Let _iterator_ be OrdinaryObjectCreate(%IntlSegmentIteratorPrototype%, _internalSlotsList_).
3. Set _iterator_.[[IteratingSegmenter]] to _segmenter_.
4. Set _iterator_.[[IteratedString]] to _string_.
5. Set _iterator_.[[IteratedStringNextSegmentCodeUnitIndex]] to 0.
6. Return _iterator_.

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


CreateSegmentDataObject ( _segmenter_: an Intl.Segmenter, _string_: a String, _startIndex_: a non-negative integer, _endIndex_: a non-negative integer, ): a Segment Data object
  id: sec-createsegmentdataobject   type: abstract operation

description: The Segment Data object describes the segment within _string_ from _segmenter_ that is bounded by the indices _startIndex_ and _endIndex_.

1. Let _length_ be the length of _string_.
2. Assert: _endIndex_ ≤ _length_.
3. Assert: _startIndex_ < _endIndex_.
4. Let _result_ be OrdinaryObjectCreate(%Object.prototype%).
5. Let _segment_ be the substring of _string_ from _startIndex_ to _endIndex_.
6. Perform ! CreateDataPropertyOrThrow(_result_, *"segment"*, _segment_).
7. Perform ! CreateDataPropertyOrThrow(_result_, *"index"*, 𝔽(_startIndex_)).
8. Perform ! CreateDataPropertyOrThrow(_result_, *"input"*, _string_).
9. Let _granularity_ be _segmenter_.[[SegmenterGranularity]].
10. If _granularity_ is *"word"*, then
   a. Let _isWordLike_ be a Boolean indicating whether the _segment_ in _string_ is "word-like" according to locale _segmenter_.[[Locale]].
   b. Perform ! CreateDataPropertyOrThrow(_result_, *"isWordLike"*, _isWordLike_).
11. Return _result_.

NOTE: Whether a segment is "word-like" is implementation-dependent, and implementations are recommended to use locale-sensitive tailorings. In general, segments consisting solely of spaces and/or punctuation (such as those terminated with "WORD_NONE" boundaries by ICU [International Components for Unicode, documented at https://unicode-org.github.io/icu-docs/]) are not considered to be "word-like".

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


## Done when

- Every test262 run under intl402/Segmenter/ passes (check with `./t262 intl402/Segmenter/`). 0 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 551: rejected by gates: clippy failed
- attempt 554: not merged (contradicts_spec); continue from this diff. 1) Use the resolved locale. Build the word, sentence and grapheme segmenters from segmenter.[[Locale]] (ICU4X locale-aware constructors), or throw agent.not_implemented naming locale-tailored segmentation. Do not silently use root data. 2) Compute the boundary list once per (segmenter, string) and cache it on the Segments and iterator slots, or otherwise avoid re-segmenting the whole string on every next()/containing() call. 3) Verify ICU4X iter_with_word_type semantics (does the flag belong to the segment ending or starting at index i?) and make isWordLike use the same boundary list. 4) The constructor's expect("a default") was replaced with unwrap_or_else(|| "grapheme") and expect("created above") with a not_implemented error. Both are harmless, but prefer keeping the spec-default path through GetOption rather than duplicating the default literal. 5) granularity_of() uses unwrap_or_default(), which silently maps a missing slot to grapheme. Prefer a clear error path. Segment() and the remaining methods are not in this diff; leaving them as not_implemented is fine.

