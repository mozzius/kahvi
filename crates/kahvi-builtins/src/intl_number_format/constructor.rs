//! Intl.NumberFormat ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.numberformat

use kahvi_core::prelude::*;

/// Intl.NumberFormat
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.NumberFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object, else let _newTarget_ be NewTarget.
    // 2. Let _numberFormat_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%Intl.NumberFormat.prototype%"*, « [[InitializedNumberFormat]], [[Locale]], [[LocaleData]], [[NumberingSystem]], [[Style]], [[Unit]], [[UnitDisplay]], [[Currency]], [[CurrencyDisplay]], [[CurrencySign]], [[MinimumIntegerDigits]], [[MinimumFractionDigits]], [[MaximumFractionDigits]], [[MinimumSignificantDigits]], [[MaximumSignificantDigits]], [[RoundingType]], [[Notation]], [[CompactDisplay]], [[UseGrouping]], [[SignDisplay]], [[RoundingIncrement]], [[RoundingMode]], [[ComputedRoundingPriority]], [[TrailingZeroDisplay]], [[BoundFormat]] »).
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.NumberFormat%, %Intl.NumberFormat%.[[LocaleData]], _locales_, _options_, « ~coerce-options~ »).
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    // 6. Set _numberFormat_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 7. Set _numberFormat_.[[LocaleData]] to _resolvedLocale_.[[LocaleData]].
    // 8. Set _numberFormat_.[[NumberingSystem]] to _resolvedLocale_.[[nu]].
    // 9. Perform ? SetNumberFormatUnitOptions(_numberFormat_, _options_).
    // 10. Let _style_ be _numberFormat_.[[Style]].
    // 11. Let _notation_ be ? GetOption(_options_, *"notation"*, ~string~, « *"standard"*, *"scientific"*, *"engineering"*, *"compact"* », *"standard"*).
    // 12. Set _numberFormat_.[[Notation]] to _notation_.
    // 13. If _style_ is *"currency"* and _notation_ is *"standard"*, then
    //    a. Let _currency_ be _numberFormat_.[[Currency]].
    //    b. Let _cDigits_ be CurrencyDigits(_currency_).
    //    c. Let _mnfdDefault_ be _cDigits_.
    //    d. Let _mxfdDefault_ be _cDigits_.
    // 14. Else,
    //    a. Let _mnfdDefault_ be 0.
    //    b. If _style_ is *"percent"*, then
    //       i. Let _mxfdDefault_ be 0.
    //    c. Else,
    //       i. Let _mxfdDefault_ be 3.
    // 15. Perform ? SetNumberFormatDigitOptions(_numberFormat_, _options_, _mnfdDefault_, _mxfdDefault_, _notation_).
    // 16. Let _compactDisplay_ be ? GetOption(_options_, *"compactDisplay"*, ~string~, « *"short"*, *"long"* », *"short"*).
    // 17. Let _defaultUseGrouping_ be *"auto"*.
    // 18. If _notation_ is *"compact"*, then
    //    a. Set _numberFormat_.[[CompactDisplay]] to _compactDisplay_.
    //    b. Set _defaultUseGrouping_ to *"min2"*.
    // 19. NOTE: For historical reasons, the strings *"true"* and *"false"* are accepted and replaced with the default value.
    // 20. Let _useGrouping_ be ? GetBooleanOrStringNumberFormatOption(_options_, *"useGrouping"*, « *"min2"*, *"auto"*, *"always"*, *"true"*, *"false"* », _defaultUseGrouping_).
    // 21. If _useGrouping_ is *"true"* or _useGrouping_ is *"false"*, set _useGrouping_ to _defaultUseGrouping_.
    // 22. If _useGrouping_ is *true*, set _useGrouping_ to *"always"*.
    // 23. Set _numberFormat_.[[UseGrouping]] to _useGrouping_.
    // 24. Let _signDisplay_ be ? GetOption(_options_, *"signDisplay"*, ~string~, « *"auto"*, *"never"*, *"always"*, *"exceptZero"*, *"negative"* », *"auto"*).
    // 25. Set _numberFormat_.[[SignDisplay]] to _signDisplay_.
    // 26. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Let _this_ be the *this* value.
    //    b. Return ? ChainNumberFormat(_numberFormat_, NewTarget, _this_).
    // 27. Return _numberFormat_.
    //
    // [subclause sec-chainnumberformat: ChainNumberFormat (
    // _numberFormat_: an Intl.NumberFormat,
    // _newTarget_: an ECMAScript language value,
    // _this_: an ECMAScript language value,
    // ): either a normal completion containing an Object or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.NumberFormat"))
}
