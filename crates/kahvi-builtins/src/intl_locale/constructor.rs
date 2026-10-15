//! Intl.Locale ( _tag_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale

use kahvi_core::prelude::*;

use crate::support::intl::IntlLocaleSlots;
use crate::support::intl_locale::initialize_locale;

/// Intl.Locale
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.Locale` function is called with an argument _tag_ and an optional argument _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _localeExtensionKeys_ be %Intl.Locale%.[[LocaleExtensionKeys]].
    // 3. Let _internalSlotsList_ be « [[InitializedLocale]], [[Locale]], [[Calendar]], [[Collation]], [[FirstDayOfWeek]], [[HourCycle]], [[NumberingSystem]] ».
    // 4. If _localeExtensionKeys_ contains *"kf"*, then
    //    a. Append [[CaseFirst]] to _internalSlotsList_.
    // 5. If _localeExtensionKeys_ contains *"kn"*, then
    //    a. Append [[Numeric]] to _internalSlotsList_.
    // 6. Let _locale_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.Locale.prototype%"*, _internalSlotsList_).
    // 7. If _tag_ is not a String and _tag_ is not an Object, throw a *TypeError* exception.
    // 8. If _tag_ is an Object and _tag_ has an [[InitializedLocale]] internal slot, then
    //    a. Set _tag_ to _tag_.[[Locale]].
    // 9. Else,
    //    a. Set _tag_ to ? ToString(_tag_).
    // 10. Set _options_ to ? CoerceOptionsToObject(_options_).
    // 11. If IsWellFormedLanguageTag(_tag_) is *false*, throw a *RangeError* exception.
    // 12. NOTE: Because LanguageId canonicalization can alter _tag_ in arbitrary ways according to Alias Rules from supplementalMetadata.xml, it is necessary to perform such canonicalization before applying overrides from _options_.
    // 13. Set _tag_ to CanonicalizeUnicodeLocaleId(_tag_).
    // 14. Set _tag_ to ? UpdateLanguageId(_tag_, _options_).
    // 15. Let _opt_ be a new Record.
    // 16. Let _calendar_ be ? GetOption(_options_, *"calendar"*, ~string~, ~empty~, *undefined*).
    // 17. If _calendar_ is not *undefined*, then
    //    a. If _calendar_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
    // 18. Set _opt_.[[ca]] to _calendar_.
    // 19. Let _collation_ be ? GetOption(_options_, *"collation"*, ~string~, ~empty~, *undefined*).
    // 20. If _collation_ is not *undefined*, then
    //    a. If _collation_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
    // 21. Set _opt_.[[co]] to _collation_.
    // 22. Let _fw_ be ? GetOption(_options_, *"firstDayOfWeek"*, ~string~, ~empty~, *undefined*).
    // 23. If _fw_ is not *undefined*, then
    //    a. Set _fw_ to WeekdayToUValue(_fw_).
    //    b. If _fw_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
    // 24. Set _opt_.[[fw]] to _fw_.
    // 25. Let _hc_ be ? GetOption(_options_, *"hourCycle"*, ~string~, « *"h11"*, *"h12"*, *"h23"*, *"h24"* », *undefined*).
    // 26. Set _opt_.[[hc]] to _hc_.
    // 27. Let _kf_ be ? GetOption(_options_, *"caseFirst"*, ~string~, « *"upper"*, *"lower"*, *"false"* », *undefined*).
    // 28. Set _opt_.[[kf]] to _kf_.
    // 29. Let _kn_ be ? GetOption(_options_, *"numeric"*, ~boolean~, ~empty~, *undefined*).
    // 30. If _kn_ is not *undefined*, set _kn_ to ! ToString(_kn_).
    // 31. Set _opt_.[[kn]] to _kn_.
    // 32. Let _numberingSystem_ be ? GetOption(_options_, *"numberingSystem"*, ~string~, ~empty~, *undefined*).
    // 33. If _numberingSystem_ is not *undefined*, then
    //    a. If _numberingSystem_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
    // 34. Set _opt_.[[nu]] to _numberingSystem_.
    // 35. Let _localeRecord_ be MakeLocaleRecord(_tag_, _opt_, _localeExtensionKeys_).
    // 36. Set _locale_.[[Locale]] to _localeRecord_.[[locale]].
    // 37. Set _locale_.[[Calendar]] to _localeRecord_.[[ca]].
    // 38. Set _locale_.[[Collation]] to _localeRecord_.[[co]].
    // 39. Set _locale_.[[FirstDayOfWeek]] to _localeRecord_.[[fw]].
    // 40. Set _locale_.[[HourCycle]] to _localeRecord_.[[hc]].
    // 41. If _localeExtensionKeys_ contains *"kf"*, then
    //    a. Set _locale_.[[CaseFirst]] to _localeRecord_.[[kf]].
    // 42. If _localeExtensionKeys_ contains *"kn"*, then
    //    a. If _localeRecord_.[[kn]] is either *"true"* or the empty String, then
    //       i. Set _locale_.[[Numeric]] to *true*.
    //    b. Else,
    //       i. Set _locale_.[[Numeric]] to *false*.
    // 43. Set _locale_.[[NumberingSystem]] to _localeRecord_.[[nu]].
    // 44. Return _locale_.
    let _ = this;
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor Intl.Locale requires 'new'"));
    };
    // 6. Let _locale_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.Locale.prototype%"*, _internalSlotsList_).
    let locale = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Intl.Locale.prototype%",
        ObjectKind::Native(Box::new(IntlLocaleSlots::default())),
    )?;
    // 7.-43. (the slots are computed from _tag_ and _options_)
    let slots = initialize_locale(agent, args.get_or_undefined(0), args.get_or_undefined(1))?;
    if let Some(s) = agent.native_mut::<IntlLocaleSlots>(locale) {
        *s = slots;
    }
    // 44. Return _locale_.
    Ok(Value::Object(locale))
}
